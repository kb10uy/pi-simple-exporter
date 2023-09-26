use crate::app::AppState;

use std::process::Stdio;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub async fn watch_under_voltage(state: AppState) {
    // -f              : follow logs
    // -n 0            : surpress logs on spawn
    // -k              : print only kernel logs
    // -g Undervoltage : grep by Undervoltage
    let uv_follow = match Command::new("journalctl")
        .args(["-f", "-n", "0", "-k"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to execute journalctl: {e}");
            return;
        }
    };

    let Some(uv_stdout) = uv_follow.stdout else {
        eprintln!("no stdout detected");
        return;
    };

    let mut uv_bufreader = BufReader::new(uv_stdout);
    let mut kernel_log_buffer = String::new();
    while uv_bufreader.read_line(&mut kernel_log_buffer).await.is_ok() {
        println!("kernel log: {}", kernel_log_buffer.trim());
        state.exporter.under_voltage.inc();
        kernel_log_buffer.clear();
    }
}
