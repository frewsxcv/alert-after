#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

#[cfg(target_os = "linux")]
extern crate notify_rust;

#[cfg(target_os = "windows")]
extern crate winrt;

use std::{borrow, env, error, io, process};
use std::io::Write;

type ExitCode = i32;

#[cfg(target_os = "macos")]
fn notify(msg_title: &str, msg_body: &str) {
    let bundle = mac_notification_sys::get_bundle_identifier("Script Editor").unwrap();
    mac_notification_sys::set_application(&bundle).unwrap();
    mac_notification_sys::send_notification(msg_title, &None, msg_body, &None).unwrap();
}

#[cfg(target_os = "linux")]
fn notify(msg_title: &str, msg_body: &str) {
    notify_rust::Notification::new()
        .summary(msg_title)
        .body(msg_body)
        .show()
        .unwrap();
}

#[cfg(target_os = "windows")]
fn notify(msg_title: &str, msg_body: &str) {
    use winrt::*;
    use winrt::windows::data::xml::dom::*;
    use winrt::windows::ui::notifications::*;
    unsafe {
    let toast_xml = ToastNotificationManager::get_template_content(ToastTemplateType_ToastText02).unwrap();
    let toast_text_elements = toast_xml.get_elements_by_tag_name(&FastHString::new("text")).unwrap();

    toast_text_elements.item(0).unwrap().append_child(&*toast_xml.create_text_node(&FastHString::from(msg_title)).unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();
    toast_text_elements.item(1).unwrap().append_child(&*toast_xml.create_text_node(&FastHString::from(msg_body)).unwrap().query_interface::<IXmlNode>().unwrap()).unwrap();

    let toast = ToastNotification::create_toast_notification(&*toast_xml).unwrap();
    ToastNotificationManager::create_toast_notifier_with_id(&FastHString::new("{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe")).unwrap().show(&*toast).unwrap();
    }
}

fn exit_status_to_message(exit_status: process::ExitStatus) -> borrow::Cow<'static, str> {
    match exit_status.code() {
        Some(0) => "Command exited successfully".into(),
        Some(code) => format!("Command exited with status code {}", code).into(),
        None => "Command exited".into(),
    }
}

fn spawn_command(args: &[String]) -> Result<process::Child, Box<error::Error>> {
    let program_name = first_arg_as_program_name(&args)?;
    process::Command::new(program_name.clone()).args(&args[1..])
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
    notify(&args.join(" "), &cmd_success);
    Ok(exit_status.code().unwrap_or(0))
}

#[cfg(windows)]
fn run() {
    let rt = winrt::RuntimeContext::init();
    match alert_after() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => writeln!(io::stderr(), "aa: {}", e).expect("could not write to stderr"),
    }
    rt.uninit();
}

#[cfg(not(windows))]
fn run() {
    match alert_after() {
        Ok(exit_code) => process::exit(exit_code),
        Err(e) => writeln!(io::stderr(), "aa: {}", e).expect("could not write to stderr"),
    }
}

fn main() {
    run();
}
