#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use serde_json::{Value, json};
use std::fs::File;
use std::io::Write;
use image::{self};
use base64;

#[tauri::command]
fn on_image_clicked(image_details: Value) -> Value {
    let python_image_procession_path = "src/image_procession.py";
    let python_executable = "python";

    let image_details_str = serde_json::to_string(&image_details)
    .expect("Failed to serialize JSON value to a string");

    let output = Command::new(python_executable)
        .arg(python_image_procession_path)
        .arg(image_details_str)
        .output()
        .expect("Failed to execute the Python script");

    // Check if the command was successful
    if output.status.success() {
        // Capture the output of the Python script
        let python_output = String::from_utf8_lossy(&output.stdout);
        // Return the Python script output
        let lines: Vec<String> = python_output.lines().map(|line| line.to_owned()).collect();
        return json!({
            "hex": &lines[0],
            "rgb": &lines[1],
        })
    }

    let err = String::from_utf8_lossy(&output.stderr).to_string();
    // Return the fallback message if the command was not successful
    json!({
        "error": format!("Couldn't process the image, {}", err)
    })
}

#[tauri::command]
fn on_image_uploaded(base64_image: &str) -> &str {
    // Decode base64 to bytes
    let base64_content = base64_image
        .split(";base64,")
        .nth(1)
        .unwrap_or_default()
        .to_string();

    let image_bytes_result = base64::decode(&base64_content).map_err(|e| e.to_string());

    let image_bytes = match image_bytes_result {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Failed to decode base64: {}", err);
            return "Failed to decode base64"
        }
    };

    // Create a dynamic image from the decoded bytes
    let img = image::load_from_memory(&image_bytes).expect("Failed to load image from bytes");
    // Save the image to a file
    let output_path = "../public/assets/images/output.png";
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    img.write_to(&mut output_file, image::ImageOutputFormat::Png)
        .expect("Failed to write image to file");

    output_path
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            on_image_clicked,
            on_image_uploaded 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
