// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use youtube_dl::YoutubeDl;

#[derive(Deserialize)]
struct DownloadRequest {
    url: String,
    output_dir: String,
}

// yt-dlp --extract-audio --audio-format mp3 --audio-quality 0 "https://www.youtube.com/watch?v=oHg5SJYRHA0"
// youtube-dl https://www.youtube.com/playlist?list=PLOU2XLYxmsILe6_eGvDN3GyiodoV3qNSC -A

#[tauri::command(rename_all="snake_case")]
fn download_video(download_request: DownloadRequest) -> String
{
    println!("...Starting Download");

    let output = YoutubeDl::new(&download_request.url)
                     .download(true)
                     .output_directory(download_request.output_dir)
                     .extract_audio(true)
                     .socket_timeout("15")
                     .run();

    println!("Download complete for url: {}", download_request.url);

    match output {
        Ok(_) => String::from("Download Complete"),
        Err(_) => String::from("Error occured. Could not download")
    }
}

fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}
