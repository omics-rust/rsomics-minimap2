use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn ours() -> Command {
    Command::new(env!("CARGO_BIN_EXE_rsomics-minimap2"))
}

fn golden(n: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(n)
}

fn minimap2_available() -> bool {
    Command::new("minimap2")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

/// Sorted first-12-column PAF records (the core alignment fields).
fn paf12(out: &[u8]) -> Vec<String> {
    let mut v: Vec<String> = String::from_utf8_lossy(out)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split('\t').take(12).collect::<Vec<_>>().join("\t"))
        .collect();
    v.sort();
    v
}

#[test]
fn runs_and_emits_paf() {
    let out = ours()
        .args(["-x", "map-ont"])
        .arg(golden("ref.fa"))
        .arg(golden("reads.fa"))
        .output()
        .unwrap();
    assert!(out.status.success());
    assert!(!out.stdout.is_empty(), "should emit PAF alignments");
}

// rsomics-minimap2 is a Quadrant-② FFI wrapper over the minimap2 C library
// (the `minimap2` crate). Its PAF output (first 12 core columns) must therefore
// be byte-identical to the minimap2 CLI on the same preset — they share the engine.
#[test]
fn matches_minimap2_cli() {
    if !minimap2_available() {
        eprintln!("skipping: minimap2 CLI not found");
        return;
    }
    let ours_out = ours()
        .args(["-x", "map-ont"])
        .arg(golden("ref.fa"))
        .arg(golden("reads.fa"))
        .output()
        .unwrap();
    let cli_out = Command::new("minimap2")
        .args(["-x", "map-ont"])
        .arg(golden("ref.fa"))
        .arg(golden("reads.fa"))
        .output()
        .unwrap();
    assert!(cli_out.status.success());
    assert_eq!(
        paf12(&ours_out.stdout),
        paf12(&cli_out.stdout),
        "PAF core columns must match the minimap2 CLI"
    );
}
