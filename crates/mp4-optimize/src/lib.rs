use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    High,
    Medium,
    Low,
    Web,
}

impl Quality {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "high" => Ok(Quality::High),
            "medium" => Ok(Quality::Medium),
            "low" => Ok(Quality::Low),
            "web" => Ok(Quality::Web),
            other => Err(format!(
                "invalid quality '{}', use high|medium|low|web",
                other
            )),
        }
    }

    fn crf(self) -> u8 {
        match self {
            Quality::High => 18,
            Quality::Medium => 23,
            Quality::Low => 28,
            Quality::Web => 25,
        }
    }

    fn max_w(self) -> u32 {
        match self {
            Quality::High => 1920,
            Quality::Medium | Quality::Web => 1280,
            Quality::Low => 854,
        }
    }

    fn max_h(self) -> u32 {
        match self {
            Quality::High => 1080,
            Quality::Medium | Quality::Web => 720,
            Quality::Low => 480,
        }
    }

    fn video_bitrate(self) -> &'static str {
        match self {
            Quality::High => "5000k",
            Quality::Medium => "2500k",
            Quality::Low => "1000k",
            Quality::Web => "1500k",
        }
    }

    fn audio_bitrate(self) -> &'static str {
        match self {
            Quality::High => "192k",
            Quality::Medium | Quality::Web => "128k",
            Quality::Low => "96k",
        }
    }

    fn level(self) -> &'static str {
        match self {
            Quality::High => "4.1",
            Quality::Medium => "3.1",
            Quality::Low => "3.0",
            Quality::Web => "3.0",
        }
    }

    fn profile(self) -> &'static str {
        match self {
            Quality::High => "high",
            Quality::Medium => "main",
            Quality::Low => "baseline",
            Quality::Web => "baseline",
        }
    }

    fn tune(self) -> Option<&'static str> {
        match self {
            Quality::High | Quality::Medium => Some("film"),
            Quality::Low | Quality::Web => None,
        }
    }

    fn fps_filter(self) -> Option<u32> {
        match self {
            Quality::High | Quality::Medium => None,
            Quality::Low | Quality::Web => Some(30),
        }
    }
}

pub struct Mp4OptimizeArgs<'a> {
    pub input: &'a Path,
    pub output_dir: Option<&'a Path>,
    pub quality: Quality,
}

pub fn run(args: Mp4OptimizeArgs) -> Result<(), String> {
    ttk_core::check_ffmpeg()?;

    if !args.input.exists() {
        return Err(format!("not found: {}", args.input.display()));
    }

    if args.input.is_file() {
        run_file_mode(args.input, args.output_dir, args.quality)
    } else {
        run_dir_mode(args.input, args.output_dir, args.quality)
    }
}

fn run_file_mode(input: &Path, output_dir: Option<&Path>, quality: Quality) -> Result<(), String> {
    if !is_mp4(input) {
        return Err(format!("not an MP4 file: {}", input.display()));
    }
    let out_dir = output_dir
        .map(|p| p.to_path_buf())
        .or_else(|| input.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    convert_file(input, &out_dir, quality)?;
    Ok(())
}

fn run_dir_mode(dir: &Path, output_dir: Option<&Path>, quality: Quality) -> Result<(), String> {
    let mp4s: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("read dir {}: {}", dir.display(), e))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_mp4(p))
        .collect();

    if mp4s.is_empty() {
        return Err(format!("no MP4 files found in {}", dir.display()));
    }

    let out_dir = output_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| dir.to_path_buf());

    let total = mp4s.len();
    let mut succeeded = 0usize;

    for mp4 in &mp4s {
        match convert_file(mp4, &out_dir, quality) {
            Ok(()) => succeeded += 1,
            Err(e) => eprintln!("warn: {}", e),
        }
    }

    println!("{}/{} files succeeded", succeeded, total);

    if succeeded == 0 && total > 0 {
        return Err(format!("all {} file(s) failed to convert", total));
    }

    Ok(())
}

fn convert_file(input: &Path, out_dir: &Path, quality: Quality) -> Result<(), String> {
    let stem = input
        .file_stem()
        .ok_or_else(|| format!("no file stem: {}", input.display()))?
        .to_string_lossy();
    let out = out_dir.join(format!("{}_optimized.mp4", stem));

    let orig_size = std::fs::metadata(input)
        .map(|m| m.len())
        .unwrap_or(0);

    let crf = quality.crf().to_string();
    let bitrate = quality.video_bitrate();
    let bufsize = {
        let n: u32 = bitrate.trim_end_matches('k').parse()
            .map_err(|_| format!("internal: bad bitrate format: {}", bitrate))?;
        format!("{}k", n * 2)
    };
    let max_w = quality.max_w();
    let max_h = quality.max_h();
    let vf = match quality.fps_filter() {
        Some(fps) => format!(
            "scale='min({},iw)':'min({},ih)':force_original_aspect_ratio=decrease,fps={}",
            max_w, max_h, fps
        ),
        None => format!(
            "scale='min({},iw)':'min({},ih)':force_original_aspect_ratio=decrease",
            max_w, max_h
        ),
    };
    let audio = quality.audio_bitrate();

    let input_str = input.to_str()
        .ok_or_else(|| format!("path contains non-UTF-8: {:?}", input))?;
    let out_str = out.to_str()
        .ok_or_else(|| format!("path contains non-UTF-8: {:?}", out))?;

    let mut cmd = Command::new("ffmpeg");
    cmd.args([
        "-i", input_str,
        "-c:v", "libx264",
        "-profile:v", quality.profile(),
        "-level:v", quality.level(),
        "-crf", &crf,
        "-maxrate", bitrate,
        "-bufsize", &bufsize,
        "-vf", &vf,
        "-pix_fmt", "yuv420p",
        "-c:a", "aac",
        "-b:a", audio,
        "-ac", "2",
        "-ar", "44100",
        "-movflags", "+faststart",
        "-preset", "slow",
    ]);
    if let Some(tune) = quality.tune() {
        cmd.arg("-tune").arg(tune);
    }
    cmd.arg("-y").arg(out_str);

    let status = cmd
        .status()
        .map_err(|e| format!("ffmpeg exec: {}", e))?;

    if !status.success() {
        return Err(format!(
            "ffmpeg failed for {} (exit {})",
            input.display(),
            status
        ));
    }

    let new_size = std::fs::metadata(&out).map(|m| m.len()).unwrap_or(0);
    if orig_size > 0 {
        let reduction = 100.0 - (new_size as f64 / orig_size as f64 * 100.0);
        println!(
            "{} → {} ({:.1}% reduction)",
            input.display(),
            out.display(),
            reduction
        );
    } else {
        println!("{} → {}", input.display(), out.display());
    }

    Ok(())
}

fn is_mp4(path: &Path) -> bool {
    path.extension()
        .map(|e| e.to_string_lossy().eq_ignore_ascii_case("mp4"))
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
    fn test_quality_parse_valid() {
        assert_eq!(Quality::parse("high").unwrap(), Quality::High);
        assert_eq!(Quality::parse("medium").unwrap(), Quality::Medium);
        assert_eq!(Quality::parse("low").unwrap(), Quality::Low);
        assert_eq!(Quality::parse("web").unwrap(), Quality::Web);
    }

    #[test]
    fn test_quality_parse_invalid() {
        let err = Quality::parse("ultra").unwrap_err();
        assert!(err.contains("invalid quality"));
        assert!(err.contains("high|medium|low|web"));
    }

    #[test]
    fn test_quality_presets() {
        assert_eq!(Quality::High.crf(), 18);
        assert_eq!(Quality::Medium.crf(), 23);
        assert_eq!(Quality::Low.crf(), 28);
        assert_eq!(Quality::Web.crf(), 25);

        assert_eq!(Quality::High.max_w(), 1920);
        assert_eq!(Quality::Low.max_h(), 480);
        assert_eq!(Quality::Web.video_bitrate(), "1500k");
        assert_eq!(Quality::High.audio_bitrate(), "192k");
    }

    #[test]
    fn test_quality_levels() {
        assert_eq!(Quality::High.level(), "4.1");
        assert_eq!(Quality::Medium.level(), "3.1");
        assert_eq!(Quality::Low.level(), "3.0");
        assert_eq!(Quality::Web.level(), "3.0");
    }

    #[test]
    fn test_quality_profiles() {
        assert_eq!(Quality::High.profile(), "high");
        assert_eq!(Quality::Medium.profile(), "main");
        assert_eq!(Quality::Low.profile(), "baseline");
        assert_eq!(Quality::Web.profile(), "baseline");
    }

    #[test]
    fn test_mp4_missing_input_err() {
        let tmp = tempdir().unwrap();
        let result = run(Mp4OptimizeArgs {
            input: &tmp.path().join("nope"),
            output_dir: None,
            quality: Quality::Web,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("not found"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }

    #[test]
    fn test_mp4_no_mp4_files_err() {
        let tmp = tempdir().unwrap();
        touch(&tmp.path().join("a.txt"));
        let result = run(Mp4OptimizeArgs {
            input: tmp.path(),
            output_dir: None,
            quality: Quality::Web,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("no MP4 files"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }

    #[test]
    fn test_mp4_single_file_not_mp4_err() {
        let tmp = tempdir().unwrap();
        let f = tmp.path().join("video.avi");
        touch(&f);
        let result = run(Mp4OptimizeArgs {
            input: &f,
            output_dir: None,
            quality: Quality::Web,
        });
        match result {
            Err(e) if e.contains("ffmpeg not found") => {}
            Err(e) => assert!(e.contains("not an MP4 file"), "unexpected: {}", e),
            Ok(()) => panic!("expected error"),
        }
    }

    #[test]
    fn test_is_mp4() {
        assert!(is_mp4(Path::new("clip.mp4")));
        assert!(is_mp4(Path::new("CLIP.MP4")));
        assert!(!is_mp4(Path::new("clip.avi")));
        assert!(!is_mp4(Path::new("clip.mkv")));
    }
}
