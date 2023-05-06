use std::{process::{Command, Output}, fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DownloadRequest {
    url: String,
    output_dir: String,
    audio_only: bool,
    file_name: String,
}

#[derive(Serialize)]
pub struct ReturnMessage {
    is_error: bool,
    message: String,
}

#[tauri::command(rename_all="snake_case")]
pub async fn check_file_exists(download_request: DownloadRequest) -> ReturnMessage
{
    println!("[PATH] Checking if File Exists");

    let output_dir = add_ending_slash(&download_request.output_dir);

    let str_path = if download_request.audio_only {
        format!("{}{}.mp3", &output_dir, &download_request.file_name)
    } else {
        format!("{}{}.mp4", &output_dir, &download_request.file_name)
    };

    let path = Path::new(&str_path);
    if path.exists() {
        eprintln!("[PATH] Error: file already exist");
        return ReturnMessage {
            is_error: true, message: "File with this name already exist".to_string()
        };
    } else {
        println!("[PATH] Path available. Continuing");
    }

    ReturnMessage { is_error: false, message: "Path available".to_string() }
}

#[tauri::command(rename_all="snake_case")]
pub async fn download_video(download_request: DownloadRequest) -> String
{
    let output_dir = add_ending_slash(&download_request.output_dir);
    const TEMP_FILE_PATH: &str = "/tmp/YT_DOWNLOADER_TEMP_FILE.webm";

    println!("[START] Download and convertion");
    println!("...Starting Download");
    let yt_dlp_output = execute_yt_dlp_command(TEMP_FILE_PATH,
                                               download_request.audio_only,
                                               &output_dir,
                                               &download_request.url);
    match yt_dlp_output {
        Err(err) => {
            eprintln!("Error on yt-dlp download: {}", err);
            return "Error occured. Could not download".to_string();
        }
        Ok(_) => println!("...Intermediate file downloaded")
    }

    let ffmpeg_output = execute_ffmpeg_command(TEMP_FILE_PATH,
                                               download_request.audio_only,
                                               &download_request.file_name,
                                               &output_dir);
    match ffmpeg_output {
        Err(err) => {
            eprintln!("Error to convert the video/audio: {}", err);
            return String::from("Error occured. Could not convert the video/audio");
        }
        Ok(_) => println!("...Intermediate file converted and named")
    }

    remove_intermediate_file(TEMP_FILE_PATH);

    println!("[END] Download and Convertion Complete");
    return String::from("Download and Convertion Complete");
}

fn add_ending_slash(path: &str) -> String
{
    if path.ends_with("/") {
        String::from(path)
    } else {
        String::from(path.to_string() + "/")
    }
}

fn execute_yt_dlp_command(temp_file_path: &str, audio_only: bool, output_dir: &str, url: &str) -> Result<Output, std::io::Error>
{
    let format_string = if audio_only {
        // Audio only
        "bestaudio/best"
    } else {
        // Best audio and video
        "best"
    };

    let output_directory = format!(" -P \"{}\" ", &output_dir);
    let output_template = format!(" -o \"{}\" ", temp_file_path);
    let format_flag = format!(" --format \"{}\" ", format_string);
    let quoted_url = format!(" \"{}\" ", url);

    let mut yt_dlp_cmd = String::new();
    yt_dlp_cmd.push_str("yt-dlp");
    yt_dlp_cmd.push_str(&output_directory);
    yt_dlp_cmd.push_str(&format_flag);
    yt_dlp_cmd.push_str(&output_template);
    yt_dlp_cmd.push_str(" --socket-timeout 15 ");
    yt_dlp_cmd.push_str(&quoted_url);

    Command::new("bash").arg("-c").arg(yt_dlp_cmd).output()
}

fn execute_ffmpeg_command(temp_file_path: &str,
                          audio_only: bool,
                          file_name: &str,
                          output_dir: &str) -> Result<Output, std::io::Error>
{
    let input_flag = format!(" -i \"{}\" ", temp_file_path);

    let output_file = if audio_only {
        format!(" \"{}{}.mp3\" ", output_dir, file_name)
    } else {
        format!(" \"{}{}.mp4\" ", output_dir, file_name)
    };

    let mut ffmpeg_cmd = String::new();
    ffmpeg_cmd.push_str("ffmpeg ");
    ffmpeg_cmd.push_str(&input_flag);
    if audio_only {
        ffmpeg_cmd.push_str(" -vn ");
    }
    ffmpeg_cmd.push_str(&output_file);

    Command::new("bash").arg("-c").arg(ffmpeg_cmd).output()
}

fn remove_intermediate_file(temp_file_path: &str)
{
    match fs::remove_file(temp_file_path) {
        Ok(_) => println!("...Intermediate file was removed"),
        Err(err) => eprintln!("Error to remove intermediate file: {}", err)
    }
}
