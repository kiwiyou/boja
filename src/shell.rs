use std::process::Command;

#[cfg(target_os = "linux")]
pub fn make_shell_command(command: &str) -> Command {
    let mut ret = Command::new("/bin/bash");
    ret.arg("-c").arg(command);
    ret
}

#[cfg(target_os = "windows")]
pub fn make_shell_command(command: &str) -> Command {
    let mut ret = Command::new("cmd");
    ret.arg("/c").arg(command);
    ret
}

#[cfg(target_os = "macos")]
pub fn make_shell_command(command: &str) -> Command {
    let mut ret = Command::new("/bin/bash");
    ret.arg("-c").arg(command);
    ret
}
