#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod ui;
mod commands;

use eframe::egui;
use models::{Operation, CookOptions};
use env_logger;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut unreal_pak_path = String::new();
    let mut input_path = String::new();
    let mut output_pak_name = String::new();
    let mut operation = Operation::Pack;
    let mut log_output = String::new();
    let mut unreal_editor_path=String::new();
    let mut target_platform=String::new();
    let mut unreal_project_path=String::new();
    eframe::run_simple_native("UE5 Project Packer", options, move |ctx, _frame| {
        ui::show_ui(
            ctx,
            &mut unreal_pak_path,
            &mut input_path,
            &mut output_pak_name,
            &mut operation,
            &mut log_output,
            &mut unreal_editor_path,
            &mut target_platform,
            &mut unreal_project_path
        );
    })
}