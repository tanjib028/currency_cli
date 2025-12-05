use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd; // modern macro

#[test]
fn shows_help() {
    let mut cmd = Command::from(cargo_bin_cmd!("currency-cli"));
    cmd.arg("--help").assert().success();
}

#[test]
fn convert_example() {
    let mut cmd = Command::from(cargo_bin_cmd!("currency-cli"));
    cmd.args(["convert", "100", "USD", "EUR"]).assert().success();
}