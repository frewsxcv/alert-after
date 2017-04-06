#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

#[cfg(not(target_os = "macos"))]
extern crate notify_rust;

use std::{borrow, env, error, io, path, process};
use std::io::Write;

type ExitCode = i32;

#[cfg(target_os = "macos")]
fn notify(msg_title: &str, msg_body: &str) {
    let bundle = mac_notification_sys::get_bundle_identifier("Script Editor").unwrap();
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

#[derive(Copy, Clone)]
enum ShellType {
    Bash,
    Csh,
    Fish,
    Sh,
    Zsh,
}

impl ShellType {
    fn from_path<P: AsRef<path::Path>>(path: &P) -> Option<ShellType> {
        match path.as_ref() {
            p if p.ends_with("bash") => Some(ShellType::Bash),
            p if p.ends_with("csh") => Some(ShellType::Csh),
            p if p.ends_with("fish") => Some(ShellType::Fish),
            p if p.ends_with("sh") => Some(ShellType::Sh),
            p if p.ends_with("zsh") => Some(ShellType::Zsh),
            _ => None,
        }
    }

    /// Shell command flag used to run a subcommand
    ///
    /// e.g. for bash: `bash -c "ls -l"`
    fn subcommand_flag(&self) -> &'static str {
        match *self {
            ShellType::Bash | ShellType::Csh | ShellType::Fish | ShellType::Zsh | ShellType::Sh => {
                "-c"
            }
        }
    }
}

struct Shell {
    path: path::PathBuf,
    type_: ShellType,
}

impl Shell {
    fn run(&self, command: &[String]) -> Result<process::Child, Box<error::Error>> {
        Ok(self.path.clone())
            .map(process::Command::new)
            .and_then(|mut c| {
                          c.arg(self.type_.subcommand_flag())
                              .arg(command.join(" "))
                              .spawn()
                      })
            .map_err(|e| e.into())
    }

    fn from_path<P: AsRef<path::Path>>(path: &P) -> Option<Shell> {
        ShellType::from_path(path).map(|type_| {
                                           Shell {
                                               path: path.as_ref().into(),
                                               type_: type_,
                                           }
                                       })
    }

    fn from_env() -> Option<Shell> {
        shell_env().and_then(|p| Shell::from_path(&p))
    }
}

fn shell_env() -> Option<String> {
    env::var("SHELL").ok()
}

fn spawn_command(command: &[String]) -> Result<process::Child, Box<error::Error>> {
    // Try to determine the user's shell and execute the command in it. This
    // is done so the user can use aliases defined in their shell environment.
    // Otherwise, execute the command directly.
    if let Some(s) = Shell::from_env() {
        s.run(command)
    } else {
        let program_name = try!(first_arg_as_program_name(&command));
        match process::Command::new(program_name.clone())
                  .args(command)
                  .spawn() {
            Ok(child) => Ok(child),
            Err(e) => Err(format!("Unknown command '{}': {}", program_name, e).into()),
        }
    }
}

fn args() -> Vec<String> {
    env::args().skip(1).collect()
}

fn first_arg_as_program_name(args: &[String]) -> Result<String, Box<error::Error>> {
    match args.first() {
        Some(program_name) => Ok(program_name.clone()),
        None => Err("usage: aa <program name and args>".into()),
    }
}

fn alert_after() -> Result<ExitCode, Box<error::Error>> {
    let args = args();
    let mut child = try!(spawn_command(&args));
    let exit_status = child.wait().expect("failed to wait on command");
    let cmd_success = exit_status_to_message(exit_status);
    notify(&args.join(" "), &cmd_success);
    Ok(exit_status.code().unwrap_or(0))
}

fn main() {
    match alert_after() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => writeln!(io::stderr(), "aa: {}", e).expect("could not write to stderr"),
    }
}
