use std::process::Command;

pub fn send_notification(title: &str, message: &str) {
    let script = format!(
        r#"display notification "{}" with title "{}""#,
        escape_quotes(message),
        escape_quotes(title)
    );

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output();
}

fn escape_quotes(s: &str) -> String {
    s.replace('"', r#"\""#)
}
