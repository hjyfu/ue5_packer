use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::env;
use std::fs;

pub fn run_pack_command(
    unreal_pak_path: &str,
    input_path: &str,
    output_pak_name: &str,
    log_output: &mut String,
) {
    let unreal_pak_path = unreal_pak_path.to_string(); // 克隆为String
    let input_path = input_path.to_string();
    let output_pak_name = output_pak_name.to_string();

    // 获取当前目录并构造完整的输出.pak路径
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let full_output_path = current_dir.join(&output_pak_name);

    if !unreal_pak_path.is_empty() && !input_path.is_empty() && !output_pak_name.is_empty() {
        let mut child = Command::new(&unreal_pak_path)
            .arg(full_output_path)  // 使用完整路径代替之前的output_pak_name
            .arg("-create=")
            .arg(&input_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

            if let Some(ref mut stdout) = child.stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    let line = line.unwrap();
                    log_output.push_str(&line);
                    log_output.push_str("\n");
                }
            }

            if let Some(ref mut stderr) = child.stderr {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    let line = line.unwrap();
                    log_output.push_str(&line);
                    log_output.push_str("\n");
                }
            }

            let status = child.wait().expect("failed to wait on child");

            if status.success() {
                log_output.push_str("\nPacking successful!");
            } else {
                log_output.push_str("\nPacking failed!");
            }
        } else {
            log_output.push_str("\nPlease select UnrealPak.exe, input path, and specify output pak name.");
        }
    }
pub(crate) fn get_umap_files(directory: &str) -> Vec<String> {
    let mut maps = Vec::new();
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "umap" {
                        maps.push(entry.path().display().to_string());
                    }
                }
            }
        }
    }
    maps
}