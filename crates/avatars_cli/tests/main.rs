use duct::{cmd, Expression};

pub fn create_cmd(args: Vec<&str>) -> Expression {
    let bin_path = env!("CARGO_BIN_EXE_avatars_cli");
    cmd(bin_path, args).stderr_to_stdout().stdout_capture()
}

#[test]
fn calling_with_same_args_generates_same_avatar() {
    let cmd = create_cmd(vec!["seed1", "--gender", "female", "--mood", "happy"]);

    let output1 = cmd.run().unwrap();
    let output2 = cmd.run().unwrap();

    assert_eq!(output1.stdout, output2.stdout);
}
