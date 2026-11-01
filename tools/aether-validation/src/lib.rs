//! Local research-integrity and planning validation; no collector or engine code.
mod evidence;
mod generate;
mod trace;

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

pub use evidence::{EvidenceSummary, check_evidence};
pub use generate::generate_records;
pub use trace::{TraceabilitySummary, check_traceability};

#[derive(Debug, Clone, Serialize)]
pub struct Violation {
    pub file: String,
    pub id: Option<String>,
    pub reason: String,
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}: {}",
            self.file,
            self.id
                .as_ref()
                .map(|s| format!(" [{s}]"))
                .unwrap_or_default(),
            self.reason
        )
    }
}
pub type ValidationResult<T> = Result<T, Vec<Violation>>;
pub(crate) fn error(file: &str, reason: impl ToString) -> Vec<Violation> {
    vec![Violation {
        file: file.into(),
        id: None,
        reason: reason.to_string(),
    }]
}
pub(crate) fn require(ok: bool, file: &str, reason: &str) -> ValidationResult<()> {
    if ok { Ok(()) } else { Err(error(file, reason)) }
}
pub(crate) fn safe_path(root: &Path, relative: &str) -> ValidationResult<PathBuf> {
    let p = Path::new(relative);
    require(
        !relative.is_empty() && p.components().all(|c| matches!(c, Component::Normal(_))),
        relative,
        "unsafe relative path",
    )?;
    let base = root.canonicalize().map_err(|e| error(relative, e))?;
    let resolved = base
        .join(p)
        .canonicalize()
        .map_err(|e| error(relative, e))?;
    require(
        resolved.starts_with(&base),
        relative,
        "path escapes root through symlink",
    )?;
    Ok(resolved)
}
pub(crate) fn read(root: &Path, path: &str) -> ValidationResult<Vec<u8>> {
    fs::read(safe_path(root, path)?).map_err(|e| error(path, e))
}
pub(crate) fn json(root: &Path, path: &str) -> ValidationResult<Value> {
    serde_json::from_slice(&read(root, path)?).map_err(|e| error(path, e))
}
pub(crate) fn text(root: &Path, path: &str) -> ValidationResult<String> {
    String::from_utf8(read(root, path)?).map_err(|e| error(path, e))
}
pub(crate) fn string<'a>(v: &'a Value, field: &str) -> ValidationResult<&'a str> {
    v[field]
        .as_str()
        .ok_or_else(|| error(field, "missing or invalid string"))
}
pub(crate) fn array<'a>(v: &'a Value, field: &str) -> ValidationResult<&'a Vec<Value>> {
    v[field]
        .as_array()
        .ok_or_else(|| error(field, "missing or invalid array"))
}
pub(crate) fn ids(v: &[Value], key: &str) -> ValidationResult<BTreeSet<String>> {
    let mut result = BTreeSet::new();
    for item in v {
        let id = string(item, key)?;
        require(result.insert(id.into()), key, &format!("duplicate ID {id}"))?;
    }
    Ok(result)
}
pub(crate) fn strings(v: &Value) -> ValidationResult<BTreeSet<String>> {
    let a = v
        .as_array()
        .ok_or_else(|| error("array", "expected string array"))?;
    let mut result = BTreeSet::new();
    for item in a {
        let s = item
            .as_str()
            .ok_or_else(|| error("array", "expected string item"))?;
        require(
            result.insert(s.into()),
            "array",
            &format!("duplicate reference {s}"),
        )?;
    }
    Ok(result)
}
pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) struct Baselines {
    pub manifest: Value,
    pub catalog: Value,
    pub coverage: Value,
    pub associations: Vec<Value>,
    pub budget: Value,
}
pub(crate) fn load_baselines(root: &Path) -> ValidationResult<Baselines> {
    let manifest = json(root, "evidence/manifest-v1.json")?;
    let local = |id: &str| -> ValidationResult<Value> {
        let entry = array(&manifest, "local_inputs")?
            .iter()
            .find(|v| v["id"] == id)
            .ok_or_else(|| error(id, "missing local input"))?;
        json(root, string(entry, "snapshot")?)
    };
    let catalog = local("B01")?;
    let coverage = local("B03")?;
    let budget = local("B06")?;
    let entry = array(&manifest, "local_inputs")?
        .iter()
        .find(|v| v["id"] == "B02")
        .ok_or_else(|| error("B02", "missing metadata baseline"))?;
    let associations = parse_association_rows(&text(root, string(entry, "snapshot")?)?)?;
    Ok(Baselines {
        manifest,
        catalog,
        coverage,
        associations,
        budget,
    })
}
/// Parse definition rows only, retaining all six cells and escaped pipes.
pub fn parse_association_rows(input: &str) -> ValidationResult<Vec<Value>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        if !line.starts_with("| M") {
            continue;
        }
        let mut cells = Vec::new();
        let mut cell = String::new();
        let mut escaped = false;
        for c in line.chars() {
            if c == '|' && !escaped {
                cells.push(cell.trim().to_string());
                cell.clear();
            } else {
                cell.push(c);
            }
            escaped = c == '\\' && !escaped;
        }
        if !cell.trim().is_empty() {
            cells.push(cell.trim().into());
        }
        if cells.first().is_some_and(String::is_empty) {
            cells.remove(0);
        }
        if cells.first().is_none_or(|s| {
            s.len() != 3 || !s.starts_with('M') || !s[1..].bytes().all(|b| b.is_ascii_digit())
        }) {
            continue;
        }
        require(
            cells.len() == 6,
            "metadata",
            "association row must have six cells",
        )?;
        rows.push(serde_json::json!({"id":cells[0], "cells":cells}));
    }
    let unique = ids(&rows, "id")?;
    let expected: BTreeSet<_> = (1..=58).map(|i| format!("M{i:02}")).collect();
    require(
        unique == expected,
        "metadata",
        "expected exactly M01 through M58",
    )?;
    Ok(rows)
}
