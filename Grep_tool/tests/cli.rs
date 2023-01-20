use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("Grep_tool")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("First test\nSecond test\nThird Test")?;

    let mut cmd = Command::cargo_bin("Grep_tool")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("One test\nAnother test"));
    Ok(())
}