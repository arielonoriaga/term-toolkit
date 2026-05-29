use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Mp3CompressArgs<'a> {
    pub input: &'a Path,
    pub output_dir: Option<&'a Path>,
}

pub fn run(args: Mp3CompressArgs) -> Result<(), String> {
    check_ffmpeg()?;

    if !args.input.exists() {
        return Err(format!("not found: {}", args.input.display()));
    }

    if args.input.is_file() {
        run_file_mode(args.input, args.output_dir)
    } else {
        run_dir_mode(args.input, args.output_dir)
    }
}

fn check_ffmpeg() -> Result<(), String> {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|_| ())
        .map_err(|_| "ffmpeg not found — install it first".to_string())
}

fn run_file_mode(input: &Path, output_dir: Option<&Path>) -> Result<(), String> {
    if !is_mp3(input) {
        return Err(format!("no MP3 files found in {}", input.display()));
    }
    let out_dir = output_dir
        .map(|p| p.to_path_buf())
        .or_else(|| input.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    convert_file(input, &out_dir)
}

fn run_dir_mode(dir: &Path, output_dir: Option<&Path>) -> Result<(), String> {
    let mp3s: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("read dir {}: {}", dir.display(), e))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_mp3(p))
        .collect();

    if mp3s.is_empty() {
        return Err(format!("no MP3 files found in {}", dir.display()));
    }

    let out_dir = output_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| dir.to_path_buf());

    let mut converted = 0usize;
    for mp3 in &mp3s {
        match convert_file(mp3, &out_dir) {
            Ok(()) => converted += 1,
            Err(e) => eprintln!("warn: {}", e),
        }
    }

    println!("converted {}/{} files", converted, mp3s.len());
    Ok(())
}

fn convert_file(input: &Path, out_dir: &Path) -> Result<(), String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| format!("no file stem: {}", input.display()))?
        .to_string_lossy();
    let out = out_dir.join(format!("{}.m4a", stem));

    let status = Command::new("ffmpeg")
        .args([
            "-i",
            &input.display().to_string(),
            "-ac",
            "1",
            "-c:a",
            "aac",
            "-b:a",
            "96k",
            &out.display().to_string(),
        ])
        .status()
        .map_err(|e| format!("ffmpeg exec: {}", e))?;

    if !status.success() {
        return Err(format!(
            "ffmpeg failed for {} (exit {})",
            input.display(),
            status
        ));
    }

    println!("{} → {}", input.display(), out.display());
    Ok(())
}

fn is_mp3(path: &Path) -> bool {
    path.extension()
        .map(|e| e.to_string_lossy().eq_ignore_ascii_case("mp3"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    fn touch(path: &Path) {
        File::create(path).unwrap();
    }

    #[test]
    fn test_mp3_missing_input_err() {
        let tmp = tempdir().unwrap();
        let result = run(Mp3CompressArgs {
            input: &tmp.path().join("nope"),
            output_dir: None,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("not found"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }

    #[test]
    fn test_mp3_no_mp3_files_err() {
        let tmp = tempdir().unwrap();
        touch(&tmp.path().join("a.txt"));
        let result = run(Mp3CompressArgs {
            input: tmp.path(),
            output_dir: None,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("no MP3 files"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }

    #[test]
    fn test_is_mp3() {
        assert!(is_mp3(Path::new("track.mp3")));
        assert!(is_mp3(Path::new("TRACK.MP3")));
        assert!(!is_mp3(Path::new("track.m4a")));
        assert!(!is_mp3(Path::new("track.wav")));
    }

    #[test]
    fn test_mp3_single_file_not_mp3_err() {
        let tmp = tempdir().unwrap();
        let f = tmp.path().join("audio.wav");
        touch(&f);
        let result = run(Mp3CompressArgs {
            input: &f,
            output_dir: None,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("no MP3"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }
}
