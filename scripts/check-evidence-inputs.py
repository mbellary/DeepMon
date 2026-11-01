#!/usr/bin/env python3
"""Verify Aether's frozen research inputs; does not validate a serving deployment."""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import re
import sys


def verify(root):
    manifest_path = root / 'evidence/manifest-v1.json'
    data = manifest_path.read_bytes()
    expected = (root / 'evidence/manifest-v1.sha256').read_text().split()[0]
    if hashlib.sha256(data).hexdigest() != expected:
        raise ValueError('manifest SHA-256 mismatch')
    manifest = json.loads(data)
    if manifest['bundle_id'] != 'aether-source-evidence-v1':
        raise ValueError('unexpected evidence bundle')
    seen_paths = set()
    for item in manifest['artifacts']:
        relative = item['path']
        path = (root / relative).resolve()
        if not path.is_relative_to(root.resolve()) or relative in seen_paths:
            raise ValueError(f'unsafe or duplicate artifact path: {relative}')
        seen_paths.add(relative)
        raw = path.read_bytes()
        if len(raw) != item['bytes'] or hashlib.sha256(raw).hexdigest() != item['sha256']:
            raise ValueError(f'artifact checksum mismatch: {relative}')
    sources = {entry['id']: entry for entry in manifest['source_inputs']}
    if len(sources) != 10 or len(manifest['source_inputs']) != 10:
        raise ValueError('expected ten unique pinned upstream inputs')
    source_text = {}
    for entry in sources.values():
        if not re.fullmatch(r'[0-9a-f]{40}', entry['commit']):
            raise ValueError(f'unpinned commit: {entry["id"]}')
        if f'/blob/{entry["commit"]}/' not in entry['url']:
            raise ValueError(f'URL does not pin commit: {entry["id"]}')
        if entry['snapshot'] not in seen_paths:
            raise ValueError('source omitted from artifact checksums')
        raw = (root / entry['snapshot']).read_bytes()
        blob = hashlib.sha1(b'blob ' + str(len(raw)).encode() + b'\0' + raw).hexdigest()
        if blob != entry['blob_sha1'] or hashlib.sha256(raw).hexdigest() != entry['sha256']:
            raise ValueError(f'upstream Git blob/content mismatch: {entry["id"]}')
        source_text[entry['id']] = raw.decode('utf-8')
    index_path = manifest['source_metadata']
    if index_path not in seen_paths:
        raise ValueError('source metadata omitted from artifact checksums')
    index = json.loads((root / index_path).read_text())
    if index['entries'] != manifest['source_inputs']:
        raise ValueError('source index and manifest disagree')
    local = {entry['id']: entry for entry in manifest['local_inputs']}
    for entry in local.values():
        if entry['snapshot'] not in seen_paths:
            raise ValueError('local baseline omitted from checksums')
    for key in ['reconciliation', 'environment']:
        if manifest[key] not in seen_paths:
            raise ValueError(f'{key} omitted from checksums')
    coverage = json.loads((root / local['B03']['snapshot']).read_text())
    catalog = json.loads((root / local['B01']['snapshot']).read_text())
    metrics = {row['id']: row for row in coverage['metrics']}
    if len(coverage['metrics']) != 250 or len(metrics) != 250:
        raise ValueError('coverage baseline must have 250 unique IDs')
    if set(metrics) != {row['id'] for row in catalog['metrics']}:
        raise ValueError('requirement IDs differ from original catalog')
    counts = collections.Counter(row['route'] for row in metrics.values())
    if counts != {'E': 88, 'R': 35, 'H': 100, 'I': 26, 'U': 1}:
        raise ValueError('unexpected v1 documentation route totals')
    reconciliation = json.loads((root / manifest['reconciliation']).read_text())
    rows = reconciliation['entries']
    historical = {entry['id'] for entry in coverage['source_mapping_review']['changes']
                  if entry['before_route'] == 'U'}
    if len(rows) != 26 or len({row['id'] for row in rows}) != 26:
        raise ValueError('expected exactly 26 unique historical U rows')
    if {row['id'] for row in rows} != historical:
        raise ValueError('historical U reconciliation is incomplete')
    if collections.Counter(row['current_route'] for row in rows) != {'E': 25, 'U': 1}:
        raise ValueError('historical U outcomes disagree')
    for row in rows:
        if row['current_route'] != metrics[row['id']]['route']:
            raise ValueError(f'route mismatch: {row["id"]}')
        if not row['pinned_evidence_ids'] or not row['native_name_checks']:
            raise ValueError(f'missing pinned evidence: {row["id"]}')
        for eid in row['pinned_evidence_ids']:
            if eid not in sources:
                raise ValueError(f'unknown source input: {eid}')
        for check in row['native_name_checks']:
            hits = check['present_in_inputs']
            if not hits or not set(hits).issubset(row['pinned_evidence_ids']):
                raise ValueError('native-name check has missing or unrelated evidence')
            for eid in hits:
                if check['native_name'] not in source_text[eid]:
                    raise ValueError(f'native name absent: {row["id"]}/{eid}')
        if row['deployed_schema_verification'] != 'not_run' or row['capability_verification'] != 'not_run':
            raise ValueError('v1 must not claim deployed verification')
    env = json.loads((root / manifest['environment']).read_text())
    if env['kind'] != 'synthetic_local_reference_environment':
        raise ValueError('v1 environment must be synthetic')
    if env['readiness']['real_deployment_available'] or env['readiness']['ready_for_acceptance_run']:
        raise ValueError('v1 cannot claim a real environment or acceptance readiness')
    print(f'PASS: {len(sources)} pinned upstream inputs; {len(manifest["artifacts"])} artifact checksums; 26/26 historical U rows reconciled.')
    print('Documentation routes: E=88 R=35 H=100 I=26 U=1. Remaining U: L1-001; L1-005 inherits its semantic gap.')
    print('Reference environment: aether-validation-synthetic-local-v1. Deployed schemas/capabilities: NOT VERIFIED.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    try:
        verify(args.root)
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f'FAIL: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
