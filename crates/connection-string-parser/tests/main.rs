use duct::{cmd, Expression};

pub fn create_cmd(args: Vec<&str>) -> Expression {
    let bin_path = env!("CARGO_BIN_EXE_connection-string-parser");
    cmd(bin_path, args).stderr_to_stdout().stdout_capture()
}

const CONNECTION_STRING: &str =
    "postgresql://other:password@localhost:5000/database_name?connect_timeout=10&application_name=app";

#[test]
#[should_panic]
fn no_url_throws_error() {
    create_cmd(vec!["--part", "port"]).run().unwrap();
}

#[test]
fn parses_correct_scheme() {
    let output = create_cmd(vec!["--part", "scheme", CONNECTION_STRING])
        .run()
        .unwrap();
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("postgresql"));
}

#[test]
fn parses_correct_host() {
    let output = create_cmd(vec!["--part", "host", CONNECTION_STRING])
        .run()
        .unwrap();
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("localhost"));
}

#[test]
fn parses_correct_port() {
    let output = create_cmd(vec!["--part", "port", CONNECTION_STRING])
        .run()
        .unwrap();
    assert!(String::from_utf8(output.stdout).unwrap().contains("5000"));
}

#[test]
fn parses_correct_user() {
    let output = create_cmd(vec!["--part", "user", CONNECTION_STRING])
        .run()
        .unwrap();
    assert!(String::from_utf8(output.stdout).unwrap().contains("other"));
}

#[test]
fn parses_correct_password() {
    let output = create_cmd(vec!["--part", "password", CONNECTION_STRING])
        .run()
        .unwrap();
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("password"));
}

#[test]
fn parses_correct_name() {
    let output = create_cmd(vec!["--part", "path", CONNECTION_STRING])
        .run()
        .unwrap();
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("database_name"));
}

#[test]
#[should_panic]
fn non_existent_part_should_raise_error() {
    create_cmd(vec![
        "--part",
        "password",
        "edgedb://hostname.com:1234?port=5678",
    ])
    .run()
    .unwrap();
}
