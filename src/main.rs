use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const AUDIO_FORMATS: [&str; 6] = ["mp3", "aac", "flac", "wav", "m4a", "opus"];
const VIDEO_FORMATS: [&str; 4] = ["mp4", "mkv", "webm", "avi"];

fn main() {
    let args: Vec<String> = env::args().collect();

    // Argument parsing
    let mut ytdlp_path: String = String::new();
    let mut result_folder: String = String::new();
    let mut quality: String = String::from("high");
    let mut ffmpeg_path: String = String::new();
    let mut extension: String = String::new();
    let mut link: String = String::new();

    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ytdlpPath" => {
                if i + 1 < args.len() {
                    ytdlp_path = args[i + 1].clone();
                    i += 1;
                }
            }
            "--resultFolder" => {
                if i + 1 < args.len() {
                    result_folder = args[i + 1].clone();
                    i += 1;
                }
            }
            "--quality" => {
                if i + 1 < args.len() {
                    quality = args[i + 1].clone();
                    i += 1;
                }
            }
            "--ffmpegPath" => {
                if i + 1 < args.len() {
                    ffmpeg_path = args[i + 1].clone();
                    i += 1;
                }
            }
            "--extension" => {
                if i + 1 < args.len() {
                    extension = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => {
                // Assume the first unknown argument is the link
                if link.is_empty() {
                    link = args[i].clone();
                }
            }
        }
        i += 1;
    }

    // Interactive mode if link is empty
    if link.is_empty() {
        let mut input: String = String::new();

        print!("Is it music or a video? (m/v): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let media_type: String = input.trim().to_lowercase();
        input.clear();

        if media_type == "m" || media_type == "music" {
            extension = "mp3".to_string();
        } else {
            extension = "mp4".to_string();
        }

        // Ask for quality
        print!("Quality (high/medium/low) [high]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let qual: String = input.trim().to_lowercase();
        if qual == "medium" || qual == "low" {
            quality = qual;
        }
        input.clear();

        // Ask for save path
        print!("Save path (default: results): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let path: &str = input.trim();
        if !path.is_empty() {
            result_folder = path.to_string();
        }
        input.clear();

        // Ask for the link
        print!("Enter the link: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        link = input.trim().to_string();
    }

    // Set default ytdlp_path if not provided
    if ytdlp_path.is_empty() {
        #[cfg(windows)]
        {
            let exe_path: PathBuf = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
            let mut ytdlp_default: PathBuf = exe_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            ytdlp_default.push("libraries");
            ytdlp_default.push("yt-dlp.exe");
            ytdlp_path = ytdlp_default.to_string_lossy().to_string();
        }
        #[cfg(not(windows))]
        {
            ytdlp_path = "/usr/bin/yt-dlp".to_string();
        }
    }

    // Set default ffmpeg_path if not provided
    if ffmpeg_path.is_empty() {
        #[cfg(windows)]
        {
            let exe_path: PathBuf = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
            let mut ffmpeg_default: PathBuf = exe_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf();
            ffmpeg_default.push("libraries");
            ffmpeg_default.push("ffmpeg.exe");
            ffmpeg_path = ffmpeg_default.to_string_lossy().to_string();
        }
        #[cfg(not(windows))]
        {
            ffmpeg_path = "/usr/bin/ffmpeg".to_string();
        }
    }

    // Set default result_folder if not provided
    if result_folder.is_empty() {
        let exe_path: PathBuf = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        let mut results_default: PathBuf = exe_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        results_default.push("results");
        result_folder = results_default.to_string_lossy().to_string();
    }

    // Link is mandatory
    if link.is_empty() {
        eprintln!(
            "Usage: [--ytdlpPath <path>] [--resultFolder <folder>] [--quality <high|medium|low>] [--ffmpegPath <path>] [--extension <ext>] <link>"
        );
        std::process::exit(1);
    }

    // Map quality to ytdlp format
    let mut format: &'static str = match quality.as_str() {
        "high" => "bestvideo+bestaudio/best",
        "medium" => "bestvideo[height<=720]+bestaudio/best[height<=720]",
        "low" => "worstvideo+worstaudio/worst",
        _ => "bestvideo+bestaudio/best",
    };

    // music.youtube automaticamente muda para mp4
    if link.contains("music.youtube") {
        format = "bestaudio/best";
        if extension.is_empty() {
            extension = "mp3".to_string();
        }
    }

    let mut cmd: Command = Command::new(&ytdlp_path);
    cmd.arg("-f")
        .arg(format)
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", result_folder))
        .arg("--ffmpeg-location")
        .arg(&ffmpeg_path)
        .arg(&link);

    if !extension.is_empty() {
        if AUDIO_FORMATS.contains(&extension.as_str()) {
            cmd.arg("-x").arg("--audio-format").arg(&extension);
        } else if VIDEO_FORMATS.contains(&extension.as_str()) {
            cmd.arg("--recode-video").arg(&extension);
        } else {
            eprintln!("Warning: Unknown extension '{}', ignoring.", extension);
        }
    }

    // Wait for the process to finish and capture output
    let output: std::process::Output = cmd
        .spawn()
        .expect("Failed to start yt-dlp")
        .wait_with_output()
        .expect("Failed to wait for yt-dlp to finish");

    if output.status.success() {
        println!("Download completed successfully.");
    } else {
        eprintln!("Download failed.");
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
