use assert_cmd::prelude::*;
// use predicates::prelude::*;
use std::process::Command;

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
