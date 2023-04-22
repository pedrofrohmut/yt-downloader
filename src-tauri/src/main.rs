// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
struct DownloadRequest {
    url: String,
    output_dir: String,
    audio_only: bool,
    artist: String,
    track_name: String,
}

//# Examples of yt-dlp as reference
// yt-dlp --extract-audio --audio-format mp3 --audio-quality 0 "https://www.youtube.com/watch?v=oHg5SJYRHA0"
// youtube-dl https://www.youtube.com/playlist?list=PLOU2XLYxmsILe6_eGvDN3GyiodoV3qNSC -A

//# Formats for further reference (kinda hard to read)
// yt-dlp -f "bv*+ba/b"
// "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b"
// "bestaudio[ext=m4a]/best[ext=mp3]"
// "bestvideo*[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]"

#[tauri::command(rename_all="snake_case")]
fn download_video(download_request: DownloadRequest) -> String
{
    println!("...Starting Download");

    let format_string = if download_request.audio_only {
        // Audio only
        "bestaudio/best"
    } else {
        // Best audio and video
        "best"
    };

    let output_dir = if download_request.output_dir.ends_with("/") {
        download_request.output_dir
    } else {
        String::from(download_request.output_dir + "/")
    };

    let output_directory = format!(" -P \"{}\" ", &output_dir);
    let output_template = format!(" -o \"YT_DOWNLOADER_TEMP_FILE.webm\" ");
    let format_flag = format!(" --format \"{}\" ", format_string);
    let quoted_url = format!(" \"{}\" ", &download_request.url);

    let mut yt_dlp_cmd = String::new();
    yt_dlp_cmd.push_str("yt-dlp");
    yt_dlp_cmd.push_str(&output_directory);
    yt_dlp_cmd.push_str(&format_flag);
    yt_dlp_cmd.push_str(&output_template);
    yt_dlp_cmd.push_str(&quoted_url);

    let yt_dlp_output = Command::new("bash").arg("-c").arg(yt_dlp_cmd).output();

    if let Err(err) = yt_dlp_output {
        eprintln!("Error on yt-dlp download: {}", err);
        return "Error occured. Could not download".to_string();
    }

    let input_flag = format!(" -i \"{}YT_DOWNLOADER_TEMP_FILE.webm\" ", &output_dir);

    let output_file = if download_request.audio_only {
        format!(" \"{}{} - {}.mp3\" ", output_dir, download_request.artist, download_request.track_name)
    } else {
        format!(" \"{}{} - {}.mp4\" ", output_dir, download_request.artist, download_request.track_name)
    };

    let mut ffmpeg_cmd = String::new();
    ffmpeg_cmd.push_str("ffmpeg ");
    ffmpeg_cmd.push_str(&input_flag);
    if download_request.audio_only {
        ffmpeg_cmd.push_str(" -vn ");
    }
    ffmpeg_cmd.push_str(&output_file);

    let ffmpeg_output = Command::new("bash").arg("-c").arg(ffmpeg_cmd).output();

    if let Err(err) = ffmpeg_output {
        eprintln!("Error to convert the video/audio: {}", err);
        return String::from("Error occured. Could not convert the video/audio");
    }

    println!("ffmpeg stdout: {:?}", ffmpeg_output.unwrap().stdout);

    return String::from("Download and Convertion Complete");
}

fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}
