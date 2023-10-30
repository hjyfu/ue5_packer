#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use native_dialog::FileDialog;
use std::env;
use std::process::Command;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut unreal_pak_path = String::new();  // path to UnrealPak.exe
    let mut input_path = String::new();  // path to the directory or file to be packed
    let mut output_pak_name = String::new();  // name of the output pak file
    let mut log_output = String::new();  // log output

    eframe::run_simple_native("UE5 Project Packer", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("UE5 Project Packer");

            if ui.button("Select UnrealPak.exe").clicked() {
                let result = FileDialog::new()
                    .show_open_single_file()
                    .unwrap();

                if let Some(path) = result {
                    unreal_pak_path = path.display().to_string();
                }
            }

            if ui.button("Select Input Directory or File").clicked() {
                let result = FileDialog::new()
                    .show_open_single_file()
                    .unwrap();

                if let Some(path) = result {
                    input_path = path.display().to_string();
                }
            }

            ui.horizontal(|ui| {
                ui.label("Output PAK Name:");
                ui.text_edit_singleline(&mut output_pak_name);
            });

            ui.horizontal(|ui| {
                ui.label("Selected UnrealPak Path:");
                ui.monospace(&unreal_pak_path);
            });

            ui.horizontal(|ui| {
                ui.label("Selected Input Path:");
                ui.monospace(&input_path);
            });

            if ui.button("Pack Project").clicked() {
                let current_exe_path = env::current_exe().expect("failed to get current exe path");
                let program_directory = current_exe_path.parent().expect("failed to get program directory");
                let output_pak_path = program_directory.join(&output_pak_name);

                if !unreal_pak_path.is_empty() && !input_path.is_empty() && !output_pak_name.is_empty() {
                    let output = Command::new(&unreal_pak_path)
                        .arg(output_pak_path)
                        .arg("-create=").arg(&input_path)
                        .output()
                        .expect("failed to execute process");

                    log_output += &String::from_utf8_lossy(&output.stdout);
                    log_output += &String::from_utf8_lossy(&output.stderr);

                    if output.status.success() {
                        log_output += "\nPacking successful!";
                    } else {
                        log_output += "\nPacking failed!";
                    }
                } else {
                    log_output += "\nPlease select UnrealPak.exe, input path, and specify output pak name.";
                }
            }

            // Display the log output
            ui.group(|ui| {
                ui.label("Log Output:");
                //添加一个滚动区域
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.monospace(&log_output);
                });
            });

        });
    })
}
