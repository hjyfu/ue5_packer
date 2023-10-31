use eframe::egui;
use native_dialog::FileDialog;
use super::models::Operation;
use super::commands::run_pack_command;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::process::Command;



#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    unreal_pak_path: Option<String>,
}


impl Config {
    fn load() -> Self {
        let path = Path::new("config.toml");
        if path.exists() {
            let content = fs::read_to_string(path).expect("Failed to read config file");
            toml::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        }
    }
    fn save(&self) {
        let content = toml::to_string(self).expect("Failed to serialize config");
        fs::write("config.toml", content).expect("Failed to write config file");
    }
}

pub fn show_ui(
    ctx: &egui::Context,
    unreal_pak_path: &mut String,
    input_path: &mut String,
    output_pak_name: &mut String,
    operation: &mut Operation,
    log_output: &mut String,
    target_platform: &mut String,
    unreal_project_path: &mut String,
    ue5_root_path: &mut String,
) {
    let mut config = Config::load();
    if unreal_pak_path.is_empty() {
        if let Some(saved_path) = &config.unreal_pak_path {
            *unreal_pak_path = saved_path.clone();
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.label("UE5 Project Packer");

        ui.horizontal(|ui| {
            ui.label("Operation:");
            for op in &[Operation::Pack, Operation::Cook] {
                if ui.radio_value(operation, *op, format!("{:?}", *op)).clicked() {
                    log_output.clear();
                }
            }
        });

        match operation {
            Operation::Pack => {
                if ui.button("Select UnrealPak.exe").clicked() {
                    let result = FileDialog::new()
                        .show_open_single_file()
                        .unwrap();

                    if let Some(path) = result {
                        *unreal_pak_path = path.display().to_string();
                        config.unreal_pak_path = Some(unreal_pak_path.clone());
                        config.save();
                    }

                }

                if ui.button("Select Input Directory or File").clicked() {
                    let result = FileDialog::new()
                        .show_open_single_dir()
                        .unwrap();

                    if let Some(path) = result {
                        *input_path = path.display().to_string();
                    }
                }

                ui.horizontal(|ui| {
                    ui.label("Output PAK Name:");
                    ui.text_edit_singleline(output_pak_name);
                });

                ui.horizontal(|ui| {
                    ui.label("Selected UnrealPak Path:");
                    ui.monospace(&*unreal_pak_path);
                });

                ui.horizontal(|ui| {
                    ui.label("Selected Input Path:");
                    ui.monospace(&*input_path);
                });

                if ui.button("Pack Project").clicked() {
                    run_pack_command(&unreal_pak_path, &input_path, &output_pak_name, log_output);
                }

                // Display the log output
                ui.group(|ui| {
                    ui.label("Pack Log Output:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.monospace(log_output);
                    });
                });
            }
            Operation::Cook => {
                let platforms = vec!["Win64", "WindowsServer", "LinuxServer", "IOS", "Android"];
                egui::ComboBox::from_label("Target Platform")
                    .selected_text(target_platform.clone()) // 使用 target_platform 显示已选中的平台
                    .show_ui(ui, |ui| {
                        for platform in &platforms {
                            if ui.selectable_label(false, *platform).clicked() {
                                *target_platform = platform.to_string(); // 更新选择的平台
                            }
                        }
                    });
                ui.horizontal(|ui| {
                    if ui.button("Choose UnrealRootPath").clicked() {
                        let result = FileDialog::new()
                            .show_open_single_dir()
                            .unwrap();

                        if let Some(path) = result {
                            *ue5_root_path = path.display().to_string();
                        }
                    }
                    ui.monospace(&*ue5_root_path);
                });
               ui.horizontal(|ui|{
                   if ui.button("Choose UnrealProjectPath").clicked() {
                       let result = FileDialog::new()
                           .show_open_single_file()
                           .unwrap();

                       if let Some(path) = result {
                           *unreal_project_path = path.display().to_string();
                       }
                   }
                   ui.monospace(&*unreal_project_path);
               });
                if ui.button("Cook Project").clicked() {
                    println!("Cook Project button was clicked");
                    run_ue5_cook_command(&ue5_root_path, &unreal_project_path, &target_platform, log_output);
                }

                // Display the log output (keep this common to both Pack and Cook if you want)
                ui.group(|ui| {
                    ui.label("Cook Log Output:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.monospace(log_output);
                    });
                });
            }
        }
    });
}

fn run_ue5_cook_command(
    ue5_root_path: &str, // 修改参数
    project_path: &str,
    target_platform: &str,
    log_output: &mut String,
) {
    let uat_path = format!("{}/Engine/Build/BatchFiles/RunUAT.bat", ue5_root_path);
    let editor_path = format!("{}/Engine/Binaries/Win64/UnrealEditor.exe", ue5_root_path);

    let command = format!(
        "{} -ScriptsForProject=\"{}\" Turnkey -command=VerifySdk -platform={} -UpdateIfNeeded -EditorIO -EditorIOPort=62548 -project=\"{}\" BuildCookRun -nop4 -utf8output -nocompileeditor -skipbuildeditor -cook -project=\"{}\" -unrealexe=\"{}\" -platform={} -installed -skipstage -nocompile -nocompileuat",
        uat_path, project_path, target_platform, project_path, project_path, editor_path, target_platform
    );

    println!("command: {}", command);

    let output = Command::new("cmd")
        .args(&["/C", &command])
        .output()
        .expect("error");

    *log_output = String::from_utf8_lossy(&output.stdout).into_owned();
    let error_output = String::from_utf8_lossy(&output.stderr).into_owned();
    if !error_output.is_empty() {
        *log_output += &format!("\nerror: {}", error_output);
    }
}
