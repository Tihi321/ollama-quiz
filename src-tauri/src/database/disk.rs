use std::fs::{self, File};
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;
use std::result::Result;

use super::constants::QUIZES_FOLDER;
use super::structs::Topics;

pub fn add_quiz(quiz_name: &str, quiz_info: Vec<Topics>) -> io::Result<()> {
    // Check for invalid characters in quiz_name
    if quiz_name.contains('/') || quiz_name.contains('\\') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid quiz name.",
        ));
    }

    let path = Path::new(QUIZES_FOLDER);
    if !path.exists() {
        fs::create_dir_all(&path)?;
        println!("Folder 'quizes' created successfully.");
    }

    let file_path = path.join(format!("{}.json", quiz_name));

    // Check if the quiz file already exists
    if file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Quiz already exists.",
        ));
    }

    // Serialize quiz_info
    let serialized = serde_json::to_string(&quiz_info)?;
    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

pub fn remove_quiz(quiz_name: &str) -> Result<(), std::io::Error> {
    let path = Path::new(QUIZES_FOLDER).join(format!("{}.json", quiz_name));

    // Check if the file exists and remove it
    if path.exists() {
        fs::remove_file(path)?;
        println!("Quiz {} removed successfully.", quiz_name);
    } else {
        println!("Quiz {} does not exist.", quiz_name);
    }

    Ok(())
}

pub fn import_quizes(paths: Vec<String>) -> Result<(), io::Error> {
    let quizes_folder_path = Path::new(QUIZES_FOLDER);

    for path_str in paths {
        let path = Path::new(&path_str);
        if let Some(file_name) = path.file_name() {
            let destination_path = quizes_folder_path.join(file_name);

            if !destination_path.exists() {
                fs::copy(&path, &destination_path)?;
                println!("File {:?} copied to {:?}", path, destination_path);
            } else {
                println!(
                    "File {:?} already exists in {:?}",
                    file_name, quizes_folder_path
                );
            }
        }
    }

    Ok(())
}

pub fn get_quizes() -> Result<Vec<(String, String)>, io::Error> {
    let path = Path::new(QUIZES_FOLDER);
    let mut quizes = Vec::new();

    // Ensure the "quizes" folder exists
    if !path.exists() {
        return Ok(quizes); // Return an empty vector if the folder does not exist
    }

    // Read the directory
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Skip if not a file
        if path.is_file() {
            let mut file = fs::File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            // Extract file name
            let file_name = entry
                .file_name()
                .into_string()
                .unwrap_or_default()
                .trim_end_matches(".json")
                .to_string();

            // Add (file_name, content) tuple to the vector
            quizes.push((file_name, content));
        }
    }

    Ok(quizes)
}
