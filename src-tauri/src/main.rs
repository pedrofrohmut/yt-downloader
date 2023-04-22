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

    let output_directory = format!(" -P \"{}\" ", download_request.output_dir);
    let output_template = format!(" -o \"{} - {}\" ", download_request.artist,
                                                      download_request.track_name);
    let quoted_url = format!(" \"{}\" ", &download_request.url);

    let mut yt_dlp_cmd = String::new();
    yt_dlp_cmd.push_str("yt-dlp");
    yt_dlp_cmd.push_str(&output_directory);
    if download_request.audio_only {
        // Requires (ffmpeg or ffprobe)
        yt_dlp_cmd.push_str("--extract-audio");
    }
    yt_dlp_cmd.push_str(&output_template);
    yt_dlp_cmd.push_str(&quoted_url);
    yt_dlp_cmd.to_string();

    let output = Command::new("bash")
                     .arg("-c")
                     .arg(yt_dlp_cmd)
                     .output();

    match output {
        Ok(_out) => {
            println!("Download complete for url: {}", download_request.url);
            // println!("LOG: {:?}", out);
            String::from("Download Complete")
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            String::from("Error occured. Could not download")
        }
    }
}

fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}
