//! Validates the fixtures under `tests/fixtures` against `schema.xsd` using `xmllint` (libxml2).
//!
//! Every file in `fixtures/valid` must validate and every file in `fixtures/invalid` must be
//! rejected. To cover a new schema rule, drop a document into the matching directory.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn schema_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("schema.xsd")
}

fn fixtures(kind: &str) -> Vec<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(kind);
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "xml"))
        .collect();
    files.sort();
    assert!(!files.is_empty(), "no fixtures in {}", dir.display());
    files
}

/// Returns `(validated, output)` for `xmllint --noout --schema schema.xsd <document>`.
fn validate(document: &Path) -> (bool, String) {
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(schema_path())
        .arg(document)
        .output()
        .expect("failed to run xmllint; install libxml2 (provides xmllint) to run these tests");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (output.status.success(), text)
}

#[test]
fn schema_is_well_formed() {
    let output = Command::new("xmllint")
        .arg("--noout")
        .arg(schema_path())
        .output()
        .expect("failed to run xmllint; install libxml2 (provides xmllint) to run these tests");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn valid_documents_validate() {
    let failures: Vec<String> = fixtures("valid")
        .iter()
        .filter_map(|path| {
            let (ok, text) = validate(path);
            (!ok).then(|| format!("{} should be valid:\n{text}", path.display()))
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn invalid_documents_are_rejected() {
    let failures: Vec<String> = fixtures("invalid")
        .iter()
        .filter_map(|path| {
            let (ok, _) = validate(path);
            ok.then(|| format!("{} should be rejected but validated", path.display()))
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn embedded_schema_matches_file() {
    let on_disk = fs::read_to_string(schema_path()).unwrap();
    assert_eq!(schema_xsd::SCHEMA_XSD, on_disk);
}
