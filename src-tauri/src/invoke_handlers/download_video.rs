use std::{process::{Command, Output}, fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DownloadRequest {
    url: String,
    output_dir: String,
    audio_only: bool,
    file_name: String,
}

#[tauri::command(rename_all="snake_case")]
pub fn download_video(download_request: DownloadRequest) -> String
{
    println!("[START] Download and convertion");

    // Add the / at the end if needed
    let output_dir = if download_request.output_dir.ends_with("/") {
        download_request.output_dir
    } else {
        String::from(download_request.output_dir + "/")
    };

    let file_exists = check_file_exists(download_request.audio_only,
                                        &output_dir,
                                        &download_request.file_name);
    if file_exists {
        eprintln!("Error: file already exist");
        return "Error: File with this name already exist".to_string()
    }

    println!("...Starting Download");

    const TEMP_FILE_PATH: &str = "/tmp/YT_DOWNLOADER_TEMP_FILE.webm";

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

fn check_file_exists(audio_only: bool, output_dir: &str, file_name: &str) -> bool
{
    let str_path = if audio_only {
        format!("{}{}.mp3", output_dir, file_name)
    } else {
        format!("{}{}.mp4", output_dir, file_name)
    };
    let path = Path::new(&str_path);
    path.exists()
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
