use std::{process::Command, fs};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadRequest {
    url: String,
    output_dir: String,
    audio_only: bool,
    artist: String,
    track_name: String,
}

#[tauri::command(rename_all="snake_case")]
pub fn download_video(download_request: DownloadRequest) -> String
{
    println!("[START] Download and convertion");

    const TEMP_FILE_PATH: &str = "/tmp/YT_DOWNLOADER_TEMP_FILE.webm";

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
    let output_template = format!(" -o \"{}\" ", TEMP_FILE_PATH);
    let format_flag = format!(" --format \"{}\" ", format_string);
    let quoted_url = format!(" \"{}\" ", &download_request.url);

    let mut yt_dlp_cmd = String::new();
    yt_dlp_cmd.push_str("yt-dlp");
    yt_dlp_cmd.push_str(&output_directory);
    yt_dlp_cmd.push_str(&format_flag);
    yt_dlp_cmd.push_str(&output_template);
    yt_dlp_cmd.push_str(&quoted_url);

    println!("...Starting Download");

    let yt_dlp_output = Command::new("bash").arg("-c").arg(yt_dlp_cmd).output();

    if let Err(err) = yt_dlp_output {
        eprintln!("Error on yt-dlp download: {}", err);
        return "Error occured. Could not download".to_string();
    }
    println!("...Intermediate file downloaded");

    let input_flag = format!(" -i \"{}\" ", TEMP_FILE_PATH);

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
    println!("...Intermediate file converted and named");

    match fs::remove_file(TEMP_FILE_PATH) {
        Ok(_) => println!("...Intermediate file was removed"),
        Err(err) => eprintln!("Error to remove intermediate file: {}", err)
    }

    println!("[END] Download and Convertion Complete");
    return String::from("Download and Convertion Complete");
}
