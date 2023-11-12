use assert_cmd::prelude::*;
use assert_fs::prelude::*;
// use predicates::prelude::*;
use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn one_file_default_flags() -> Result<(), Box<dyn std::error::Error>> {
    let args = vec!["./src/main.rs"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn multiple_files_default_flags() -> Result<(), Box<dyn std::error::Error>> {
    let args = vec!["./src/main.rs", "./src/cli.rs", "./src/wc.rs"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn one_file_w_flag() -> Result<(), Box<dyn std::error::Error>> {
    let args = vec!["-w", "./src/main.rs"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn multiple_files_w_flag() -> Result<(), Box<dyn std::error::Error>> {
    let args = vec!["-w", "./src/main.rs", "./src/cli.rs", "./src/wc.rs"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn multiple_files_cw_flags() -> Result<(), Box<dyn std::error::Error>> {
    let args = vec!["-cw", "./src/main.rs", "./src/cli.rs", "./src/wc.rs"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn multiple_files_L_flag() -> Result<(), Box<dyn std::error::Error>> {
    let args = vec!["-L", "./src/main.rs", "./src/cli.rs", "./src/wc.rs"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn stdin_default_flags() -> Result<(), Box<dyn std::error::Error>> {
    let input = Command::new("ls").arg("-aFl").arg("/").output()?.stdout;
    let args = vec!["-"];

    let mut expected = Command::new("wc")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    expected.stdin.as_mut().unwrap().write_all(&input)?;

    let mut actual = Command::cargo_bin("wc")?
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    actual.stdin.as_mut().unwrap().write_all(&input)?;

    let expected_out = String::from_utf8(expected.wait_with_output()?.stdout)?;
    let actual_out = String::from_utf8(actual.wait_with_output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn stdin_c_flag() -> Result<(), Box<dyn std::error::Error>> {
    let input = Command::new("ls").arg("-aFl").arg("/").output()?.stdout;
    let args = vec!["-c", "-"];

    let mut expected = Command::new("wc")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    expected.stdin.as_mut().unwrap().write_all(&input)?;

    let mut actual = Command::cargo_bin("wc")?
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    actual.stdin.as_mut().unwrap().write_all(&input)?;

    let expected_out = String::from_utf8(expected.wait_with_output()?.stdout)?;
    let actual_out = String::from_utf8(actual.wait_with_output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn stdin_and_files_default_flags() -> Result<(), Box<dyn std::error::Error>> {
    let input = Command::new("ls").arg("-aFl").arg("/").output()?.stdout;
    let args = vec!["./src/main.rs", "-"];

    let mut expected = Command::new("wc")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    expected.stdin.as_mut().unwrap().write_all(&input)?;

    let mut actual = Command::cargo_bin("wc")?
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    actual.stdin.as_mut().unwrap().write_all(&input)?;

    let expected_out = String::from_utf8(expected.wait_with_output()?.stdout)?;
    let actual_out = String::from_utf8(actual.wait_with_output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn stdin_and_files_c_flag() -> Result<(), Box<dyn std::error::Error>> {
    let input = Command::new("ls").arg("-aFl").arg("/").output()?.stdout;
    let args = vec!["-c", "./src/main.rs", "-"];

    let mut expected = Command::new("wc")
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    expected.stdin.as_mut().unwrap().write_all(&input)?;

    let mut actual = Command::cargo_bin("wc")?
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    actual.stdin.as_mut().unwrap().write_all(&input)?;

    let expected_out = String::from_utf8(expected.wait_with_output()?.stdout)?;
    let actual_out = String::from_utf8(actual.wait_with_output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn file_from_default_flags() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("./src/main.rs")?;

    let args = vec!["--files0-from", file.path().to_str().unwrap()];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
fn files_from_default_flags() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("./src/main.rs\0./src/cli.rs\0./src/wc.rs")?;

    let args = vec!["--files0-from", file.path().to_str().unwrap()];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn files_from_L_flag() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("./src/main.rs\0./src/cli.rs\0./src/wc.rs")?;

    let args = vec!["--files0-from", file.path().to_str().unwrap(), "-L"];

    let mut expected = Command::new("wc");
    expected.args(&args);

    let mut actual = Command::cargo_bin("wc")?;
    actual.args(&args);

    expected.assert().success();
    actual.assert().success();

    let expected_out = String::from_utf8(expected.output()?.stdout)?;
    let actual_out = String::from_utf8(actual.output()?.stdout)?;

    assert_eq!(expected_out, actual_out);

    Ok(())
}
