// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use serde::Deserialize;
use youtube_dl::YoutubeDl;

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
// yt-dlp -f "bv*+ba/b"
// "bv*[ext=mp4]+ba[ext=m4a]/b[ext=mp4] / bv*+ba/b"

#[tauri::command(rename_all="snake_case")]
fn download_video(download_request: DownloadRequest) -> String
{
    println!("...Starting Download");

    // let executable_path = Path::new("/usr/bin/yt-dlp");

    let output_template = download_request.artist + " - " + &download_request.track_name;

    let format = if download_request.audio_only {
        // Audio only
        "bestaudio[ext=m4a]/best[ext=mp3]"
    } else {
        "bestvideo*[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]"
    };

    let output =
        YoutubeDl::new(&download_request.url)
            // .youtube_dl_path(executable_path)
            .download(true)
            .output_directory(download_request.output_dir)
            .output_template(output_template)
            .format(format)
            .extract_audio(download_request.audio_only)
            .socket_timeout("15")
            .run();

    println!("Download complete for url: {}", download_request.url);

    match output {
        Ok(_) => String::from("Download Complete"),
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
