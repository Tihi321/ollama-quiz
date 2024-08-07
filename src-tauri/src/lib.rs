mod database;
mod llm;
mod server;
mod utils;

use std::thread;

use database::{
    disk::{
        add_quiz, add_server_quiz, get_quiz_path, get_quizes, get_server_quizes, import_quizes,
        remove_quiz, remove_server_quiz,
    },
    structs::{QuizQuestionRequest, QuizQuestionSaveRequest, Topics},
};
use llm::requests::generate_quiz_questions;
use serde_json::Error as SerdeError;
use server::server::start_server;
use tauri::{Emitter, Listener, Manager};
use tokio;
use utils::{constants::WINDOW_LABEL, index::create_window};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    thread::spawn(move || {
        start_server().unwrap();
    });

    let _ = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let app_handle = app.app_handle();
            let generate_quiz_handle = app_handle.clone();
            let get_quizes_handle = app_handle.clone();
            let get_server_quizes_handle = app_handle.clone();
            let remove_quiz_handle = app_handle.clone();
            let add_to_server_quizes = app_handle.clone();
            let remove_server_quiz_handle = app_handle.clone();
            let import_quiz_handle = app_handle.clone();
            let save_quiz_handle = app_handle.clone();

            let _ = create_window(&app).unwrap();

            app.listen("generate_quiz", move |event| {
                let value = event.payload();
                let generate_quiz_handle_clone = generate_quiz_handle.clone();
                match serde_json::from_str::<QuizQuestionRequest>(value) {
                    Ok(payload) => {
                        let language = payload.language;
                        let model = payload.model;
                        let name = payload.name;
                        let topics = payload.topics;
                        let difficulty = payload.difficulty;
                        let num_questions = payload.num_questions;
                        let max_points = payload.max_points;

                        tokio::spawn(async move {
                            match generate_quiz_questions(
                                &language,
                                &model,
                                &topics,
                                &difficulty,
                                num_questions,
                                max_points,
                            )
                            .await
                            {
                                Ok(response) => {
                                    let clean_response = response
                                        .trim_start_matches("```json")
                                        .trim_end_matches("```");
                                    println!("Attempting to parse JSON: {}", clean_response);

                                    let quiz_info: Result<Vec<Topics>, SerdeError> =
                                        serde_json::from_str(clean_response);
                                    match quiz_info {
                                        Ok(quiz_info) => {
                                            // Add the serialized JSON string as a quiz
                                            add_quiz(&name, quiz_info).expect("Failed to add quiz");
                                        }
                                        Err(e) => {
                                            println!("{}", clean_response);
                                            println!("Failed to parse JSON: {}", e);
                                        }
                                    }
                                    let quizes = get_quizes().expect("Failed to add quiz");
                                    // Use the app_handle to get the window by its label
                                    if let Some(window) =
                                        generate_quiz_handle_clone.get_webview_window(WINDOW_LABEL)
                                    {
                                        // Now you can interact with the window, e.g., emit an event back to it
                                        window
                                            .emit_to(WINDOW_LABEL, "quizes", quizes)
                                            .expect("Failed to emit event");
                                    }
                                }
                                Err(e) => println!("Error generating questions: {}", e),
                            }
                        });
                    }
                    Err(e) => eprintln!("Failed to parse event payload: {}", e),
                }
            });

            app.listen("save_quiz", move |event| {
                let value = event.payload();
                match serde_json::from_str::<QuizQuestionSaveRequest>(value) {
                    Ok(payload) => {
                        let data = payload.data;
                        let name = payload.name;
                        add_quiz(&name, data).expect("Failed to add quiz");
                        let quizes = get_quizes().expect("Failed to add quiz");
                        // Use the app_handle to get the window by its label
                        if let Some(window) = save_quiz_handle.get_webview_window(WINDOW_LABEL) {
                            // Now you can interact with the window, e.g., emit an event back to it
                            window
                                .emit_to(WINDOW_LABEL, "quizes", quizes)
                                .expect("Failed to emit event");
                        }
                    }
                    Err(e) => eprintln!("Failed to parse event payload: {}", e),
                }
            });

            app.listen("get_quizes", move |_| {
                let quizes = get_quizes().expect("Failed to get quizes");
                // Use the app_handle to get the window by its label
                if let Some(window) = get_quizes_handle.get_webview_window(WINDOW_LABEL) {
                    // Now you can interact with the window, e.g., emit an event back to it
                    window
                        .emit_to(WINDOW_LABEL, "quizes", quizes)
                        .expect("Failed to emit event");
                }
            });

            app.listen("get_server_quizes", move |_| {
                let quizes = get_server_quizes().expect("Failed to get quizes");
                // Use the app_handle to get the window by its label
                if let Some(window) = get_server_quizes_handle.get_webview_window(WINDOW_LABEL) {
                    // Now you can interact with the window, e.g., emit an event back to it
                    window
                        .emit_to(WINDOW_LABEL, "server_quizes", quizes)
                        .expect("Failed to emit event");
                }
            });

            app.listen("remove_quiz", move |event| {
                let name = event.payload();
                let clean_response = name.trim_start_matches('"').trim_end_matches('"');
                remove_quiz(clean_response).expect("Failed to remove quiz");
                let quizes = get_quizes().expect("Failed to remove quizes");
                // Use the app_handle to get the window by its label
                if let Some(window) = remove_quiz_handle.get_webview_window(WINDOW_LABEL) {
                    // Now you can interact with the window, e.g., emit an event back to it
                    window
                        .emit_to(WINDOW_LABEL, "quizes", quizes)
                        .expect("Failed to emit event");
                }
            });

            app.listen("remove_server_quiz", move |event| {
                let name = event.payload();
                let clean_response = name.trim_start_matches('"').trim_end_matches('"');
                remove_server_quiz(clean_response).expect("Failed to remove quiz");
                let quizes = get_server_quizes().expect("Failed to remove quizes");
                // Use the app_handle to get the window by its label
                if let Some(window) = remove_server_quiz_handle.get_webview_window(WINDOW_LABEL) {
                    // Now you can interact with the window, e.g., emit an event back to it
                    window
                        .emit_to(WINDOW_LABEL, "server_quizes", quizes)
                        .expect("Failed to emit event");
                }
            });

            app.listen("add_to_server_quiz", move |event| {
                let name = event.payload();
                let clean_response = name.trim_start_matches('"').trim_end_matches('"');
                let quiz_path = get_quiz_path(clean_response).expect("Failed to get quizes");
                add_server_quiz(quiz_path).expect("Failed to remove quiz");
                let quizes = get_server_quizes().expect("Failed to remove quizes");
                // Use the app_handle to get the window by its label
                if let Some(window) = add_to_server_quizes.get_webview_window(WINDOW_LABEL) {
                    // Now you can interact with the window, e.g., emit an event back to it
                    window
                        .emit_to(WINDOW_LABEL, "server_quizes", quizes)
                        .expect("Failed to emit event");
                }
            });

            app.listen("import_quiz", move |event| {
                let values = event.payload();
                // Attempt to parse the JSON payload into a Vec<String>
                match serde_json::from_str::<Vec<String>>(values) {
                    Ok(paths) => {
                        println!("Parsed paths: {:?}", paths);
                        import_quizes(paths).expect("Failed to import quizes");
                    }
                    Err(e) => eprintln!("Failed to parse paths: {}", e),
                }
                let quizes = get_quizes().expect("Failed to import quizes");
                // Use the app_handle to get the window by its label
                if let Some(window) = import_quiz_handle.get_webview_window(WINDOW_LABEL) {
                    // Now you can interact with the window, e.g., emit an event back to it
                    window
                        .emit_to(WINDOW_LABEL, "quizes", quizes)
                        .expect("Failed to emit event");
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
