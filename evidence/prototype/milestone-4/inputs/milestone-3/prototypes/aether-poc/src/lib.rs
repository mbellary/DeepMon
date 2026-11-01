//! Synthetic fixture harness; no real collectors or metric semantics.
pub mod config;
pub mod endpoint;
pub mod fixture;
pub mod report;

#[derive(Debug)]
pub struct PrototypeError {
    pub code: &'static str,
    pub context: String,
    pub fixture_id: Option<String>,
}
impl PrototypeError {
    pub fn new(code: &'static str, context: impl ToString) -> Self {
        Self {
            code,
            context: context.to_string(),
            fixture_id: None,
        }
    }
    pub fn fixture(mut self, id: &str) -> Self {
        self.fixture_id = Some(id.into());
        self
    }
}
impl std::fmt::Display for PrototypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.context)?;
        if let Some(id) = &self.fixture_id {
            write!(f, " [{id}]")?;
        }
        Ok(())
    }
}
impl std::error::Error for PrototypeError {}
impl From<std::io::Error> for PrototypeError {
    fn from(e: std::io::Error) -> Self {
        Self::new("io_error", e)
    }
}
pub type Result<T> = std::result::Result<T, PrototypeError>;
