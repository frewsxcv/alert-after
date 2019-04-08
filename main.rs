extern crate notifica;

use std::io::Write;
use std::{borrow, env, error, io, process};

type ExitCode = i32;

fn exit_status_to_message(exit_status: process::ExitStatus) -> borrow::Cow<'static, str> {
    match exit_status.code() {
        Some(0) => "Command exited successfully".into(),
        Some(code) => format!("Command exited with status code {}", code).into(),
        None => "Command exited".into(),
    }
}

fn spawn_command(args: &[String]) -> Result<process::Child, Box<error::Error>> {
    let program_name = first_arg_as_program_name(&args)?;
    process::Command::new(program_name.clone())
        .args(&args[1..])
        .spawn()
        .map_err(|e| format!("aa: Unknown command '{}': {}", program_name, e).into())
}

fn args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn first_arg_as_program_name(args: &[String]) -> Result<String, Box<error::Error>> {
    args.first()
        .cloned()
        .ok_or_else(|| "usage: aa <program name and args>".into())
}

fn alert_after() -> Result<ExitCode, Box<error::Error>> {
    let args = args();
    let mut child = spawn_command(&args)?;
    let exit_status = child.wait().expect("failed to wait on command");
    let cmd_success = exit_status_to_message(exit_status);
    notifica::notify(&args.join(" "), &cmd_success);
    Ok(exit_status.code().unwrap_or(0))
}

fn run() {
    match alert_after() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => writeln!(io::stderr(), "aa: {}", e).expect("could not write to stderr"),
    }
}

fn main() {
    run();
}
