#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

#[cfg(not(target_os = "macos"))]
extern crate notify_rust;

use std::{borrow, env, error, io, process};
use std::io::Write;

type ExitCode = i32;

#[cfg(target_os = "macos")]
fn notify(msg_title: &str, msg_body: &str) {
    let bundle = mac_notification_sys::get_bundle_identifier("safari").unwrap();

    mac_notification_sys::set_application(&bundle).unwrap();
    mac_notification_sys::send_notification(msg_title, &None, msg_body, &None).unwrap();
}

#[cfg(not(target_os = "macos"))]
fn notify(msg_title: &str, msg_body: &str) {
    notify_rust::Notification::new()
        .summary(msg_title)
        .body(msg_body)
        .show()
        .unwrap();
}

fn exit_status_to_message(exit_status: process::ExitStatus) -> borrow::Cow<'static, str> {
    match exit_status.code() {
        Some(0) => "Command exited successfully".into(),
        Some(code) => format!("Command exited with status code {}", code).into(),
        None => "Command exited".into(),
    }
}

fn spawn_command(mut command: process::Command, program_name: &str) -> Result<process::Child, Box<error::Error>> {
    match command.spawn() {
        Ok(child) => Ok(child),
        Err(e) => Err(format!("aa: Unknown command '{}': {}", program_name, e).into()),
    }
}

fn alert_after() -> Result<ExitCode, Box<error::Error>>  {
    let mut args = env::args();

    let _ = args.next().unwrap();

    let program_name = match args.next() {
        Some(program_name) => program_name,
        None => return Err("usage: aa <program name and args>".into()),
    };

    let mut command = process::Command::new(program_name.clone());
    let args = args.collect::<Vec<_>>();
    command.args(&args.clone());

    let mut child = try!(spawn_command(command, &program_name));

    let exit_status = child.wait().expect("failed to wait on command");

    let mut full_cmd = program_name;
    full_cmd.push_str(" ");
    full_cmd.push_str(&args.join(" "));

    let cmd_success = exit_status_to_message(exit_status);

    notify(&full_cmd, &cmd_success);

    Ok(exit_status.code().unwrap_or(0))
}

fn main() {
    match alert_after() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => writeln!(io::stderr(), "aa: {}", e).expect("could not write to stderr"),
    }
}
