use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Mp3CompressArgs<'a> {
    pub input: &'a Path,
    pub output_dir: Option<&'a Path>,
    pub stereo: bool,
}

pub fn run(args: Mp3CompressArgs) -> Result<(), String> {
    ttk_core::check_ffmpeg()?;

    if !args.input.exists() {
        return Err(format!("not found: {}", args.input.display()));
    }

    if args.input.is_file() {
        run_file_mode(args.input, args.output_dir, args.stereo)
    } else {
        run_dir_mode(args.input, args.output_dir, args.stereo)
    }
}

fn run_file_mode(input: &Path, output_dir: Option<&Path>, stereo: bool) -> Result<(), String> {
    if !is_mp3(input) {
        return Err(format!("not an MP3 file: {}", input.display()));
    }
    let out_dir = output_dir
        .map(|p| p.to_path_buf())
        .or_else(|| input.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    convert_file(input, &out_dir, stereo)
}

fn run_dir_mode(dir: &Path, output_dir: Option<&Path>, stereo: bool) -> Result<(), String> {
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

    let total = mp3s.len();
    let mut succeeded = 0usize;

    for mp3 in &mp3s {
        match convert_file(mp3, &out_dir, stereo) {
            Ok(()) => succeeded += 1,
            Err(e) => eprintln!("warn: {}", e),
        }
    }

    println!("converted {}/{} files", succeeded, total);

    if succeeded == 0 && total > 0 {
        return Err(format!("all {} file(s) failed to convert", total));
    }

    Ok(())
}

fn convert_file(input: &Path, out_dir: &Path, stereo: bool) -> Result<(), String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| format!("no file stem: {}", input.display()))?
        .to_string_lossy();
    let out = out_dir.join(format!("{}.m4a", stem));

    let channels = if stereo { "2" } else { "1" };

    let input_str = input.to_str()
        .ok_or_else(|| format!("path contains non-UTF-8: {:?}", input))?;
    let out_str = out.to_str()
        .ok_or_else(|| format!("path contains non-UTF-8: {:?}", out))?;

    let status = Command::new("ffmpeg")
        .args([
            "-i",
            input_str,
            "-ac",
            channels,
            "-c:a",
            "aac",
            "-b:a",
            "96k",
            out_str,
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
            stereo: false,
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
            stereo: false,
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
            stereo: false,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("not an MP3 file"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }
}
