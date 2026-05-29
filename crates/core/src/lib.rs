pub mod fs_utils;
pub mod git;
pub mod markdown;
pub mod output;
pub mod prompt;

pub fn tlog(msg: &str) {
    let now = chrono::Local::now();
    println!("[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), msg);
}

pub fn check_ffmpeg() -> Result<(), String> {
    std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map_err(|_| "ffmpeg not found — install it first".to_string())?;
    Ok(())
}
