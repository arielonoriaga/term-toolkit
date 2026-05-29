use image::{ImageFormat, ImageReader};
use std::fs;
use std::path::Path;

pub struct OptimizeArgs<'a> {
    pub input: &'a Path,
    pub output: Option<&'a Path>,
    pub quality: u8,
    pub keep_original: bool,
}

pub fn run(args: OptimizeArgs) -> Result<(), String> {
    let meta = fs::metadata(args.input).map_err(|e| e.to_string())?;
    if meta.is_file() {
        optimize_file(args.input, args.output, args.quality, args.keep_original)
    } else if meta.is_dir() {
        optimize_dir(args.input, args.output, args.quality, args.keep_original)
    } else {
        Err("path is neither a file nor a directory".to_string())
    }
}

fn optimize_file(
    input: &Path,
    output: Option<&Path>,
    quality: u8,
    keep_original: bool,
) -> Result<(), String> {
    let fmt = ImageFormat::from_path(input).map_err(|e| e.to_string())?;
    let img = ImageReader::open(input)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let dest = output
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| input.to_path_buf());

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    match fmt {
        ImageFormat::Jpeg => {
            let mut buf = Vec::new();
            let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
            enc.encode_image(&img).map_err(|e| e.to_string())?;
            // always write to dest (= output if Some, else input)
            fs::write(&dest, &buf).map_err(|e| e.to_string())?;
            // if output=Some && !keep_original → also overwrite original (matches TS behavior)
            if output.is_some() && !keep_original {
                fs::write(input, &buf).map_err(|e| e.to_string())?;
            }
        }
        _ => {
            img.save_with_format(&dest, fmt).map_err(|e| e.to_string())?;
            // if output=Some && !keep_original → also overwrite original (matches TS and JPEG behavior)
            if output.is_some() && !keep_original {
                let data = fs::read(&dest).map_err(|e| e.to_string())?;
                fs::write(input, data).map_err(|e| e.to_string())?;
            }
        }
    }

    println!("Optimized: {}", dest.display());
    Ok(())
}

fn optimize_dir(
    input: &Path,
    output: Option<&Path>,
    quality: u8,
    keep_original: bool,
) -> Result<(), String> {
    let entries = fs::read_dir(input).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let out_path = output.map(|o| o.join(entry.file_name()));
        let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
        if meta.is_file() && ImageFormat::from_path(&path).is_ok() {
            optimize_file(&path, out_path.as_deref(), quality, keep_original)?;
        } else if meta.is_dir() {
            optimize_dir(&path, out_path.as_deref(), quality, keep_original)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn tiny_jpeg(path: &Path) {
        let img = image::RgbImage::new(2, 2);
        img.save_with_format(path, image::ImageFormat::Jpeg).unwrap();
    }

    fn tiny_png(path: &Path) {
        let img = image::RgbImage::new(2, 2);
        img.save_with_format(path, image::ImageFormat::Png).unwrap();
    }

    #[test]
    fn test_optimize_jpeg_produces_valid_image() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.jpg");
        let output = dir.path().join("out.jpg");
        tiny_jpeg(&input);

        run(OptimizeArgs {
            input: &input,
            output: Some(&output),
            quality: 50,
            keep_original: true,
        })
        .unwrap();

        assert!(output.exists());
        ImageReader::open(&output).unwrap().decode().unwrap();
    }

    #[test]
    fn test_optimize_png_produces_valid_image() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.png");
        let output = dir.path().join("out.png");
        tiny_png(&input);

        run(OptimizeArgs {
            input: &input,
            output: Some(&output),
            quality: 80,
            keep_original: true,
        })
        .unwrap();

        assert!(output.exists());
        ImageReader::open(&output).unwrap().decode().unwrap();
    }

    #[test]
    fn test_optimize_overwrites_original_when_no_output() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.jpg");
        tiny_jpeg(&input);

        run(OptimizeArgs {
            input: &input,
            output: None,
            quality: 10,
            keep_original: false,
        })
        .unwrap();

        assert!(input.exists());
        // verify it's still a valid image after overwrite
        ImageReader::open(&input).unwrap().decode().unwrap();
    }

    #[test]
    fn test_optimize_dir_processes_images() {
        let dir = tempdir().unwrap();
        tiny_jpeg(&dir.path().join("a.jpg"));
        tiny_png(&dir.path().join("b.png"));
        let out_dir = dir.path().join("out");
        fs::create_dir(&out_dir).unwrap();

        run(OptimizeArgs {
            input: dir.path(),
            output: Some(&out_dir),
            quality: 80,
            keep_original: true,
        })
        .unwrap();

        assert!(out_dir.join("a.jpg").exists());
        assert!(out_dir.join("b.png").exists());
    }

    #[test]
    fn test_optimize_non_jpeg_overwrites_original_when_output_and_no_keep() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.png");
        let output = dir.path().join("out.png");
        tiny_png(&input);
        let _original_bytes = std::fs::read(&input).unwrap();

        run(OptimizeArgs {
            input: &input,
            output: Some(&output),
            quality: 80,
            keep_original: false,
        })
        .unwrap();

        assert!(output.exists(), "output must exist");
        assert!(input.exists(), "original must still exist");
        // both input and output should contain identical valid image data
        let out_bytes = std::fs::read(&output).unwrap();
        let inp_bytes = std::fs::read(&input).unwrap();
        assert_eq!(out_bytes, inp_bytes, "original must be overwritten with same data as output");
        // verify still a valid image
        ImageReader::open(&input).unwrap().decode().unwrap();
    }
}
