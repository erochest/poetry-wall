use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use spectral::prelude::*;

#[test]
fn test_basic_interaction() {
    let output_dir = assert_fs::TempDir::new().unwrap();
    let output_file = output_dir.child("fly-buzz.png");

    let cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .arg("--poem")
        .arg("./tests/fixtures/fly-buzz.md")
        .arg("--color")
        .arg("white")
        .arg("--background")
        .arg("black")
        .arg("--font")
        .arg("./tests/fixtures/DejaVuSansMono.ttf")
        .arg("--max-font-size")
        .arg("24")
        .arg("--dimensions")
        .arg("2880x2560")
        .arg("--left")
        .arg("20")
        .arg("--top")
        .arg("100")
        .arg("--output")
        .arg(&output_file.path())
        .unwrap();

    assert_that(&cmd.status.success()).is_true();
    output_file.assert(predicate::path::exists());
}