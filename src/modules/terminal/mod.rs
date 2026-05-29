pub fn execute_single_command(cmd: &str) -> String {
    // For a "super light" terminal without full PTY (stateless commands)
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
            .args(&["-Command", cmd])
            .output()
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
    };

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            format!("{}{}", stdout, stderr)
        }
        Err(e) => format!("Error executing command: {}", e),
    }
}
