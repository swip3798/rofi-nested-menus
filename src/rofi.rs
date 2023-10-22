use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::args::ARGS;

pub fn call_rofi_menu(options: &[String]) -> String {
    let mut com = Command::new("rofi");
    if let Some(theme) = &ARGS.theme {
        com.arg("-theme");
        com.arg(theme);
    }
    com.arg("-dmenu");
    com.arg("-i");
    com.stdin(Stdio::piped());
    com.stdout(Stdio::piped());
    let mut child = com.spawn().expect("Rofi command failed");
    if let Some(mut stdin) = child.stdin.take() {
        let options = options.join("\n");
        stdin
            .write_all(options.as_bytes())
            .expect("Failed to write options to rofis stdin");
    }
    let mut output = child
        .wait_with_output()
        .expect("Failed to get rofis output");
    output.stdout.pop();
    String::from_utf8(output.stdout).expect("Rofis output was not UTF-8!")
}

pub fn call_string_command(command: &str) {
    let mut com = Command::new("bash");
    com.args(["-c", command]);    
    com.status()
        .unwrap_or_else(|e| panic!("Failed to run command, error: {e}",));
}
