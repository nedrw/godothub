// Launcher - Godot 启动器服务

use std::path::Path;
use std::process::Command;

/// 启动 Godot 引擎
///
/// # Arguments
/// * `exec_path` - Godot 可执行文件路径
///
/// # Returns
/// * `Result<(), String>` - 启动结果
pub fn launch_godot(exec_path: &Path) -> Result<(), String> {
    if !exec_path.exists() {
        return Err(format!("Godot executable not found at: {}", exec_path.display()));
    }

    log::info!("Launching Godot: {}", exec_path.display());

    let result = launch_for_current_platform(exec_path);

    match result {
        Ok(_) => {
            log::info!("Godot launched successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to launch Godot: {}", e);
            Err(e)
        }
    }
}

/// 根据当前平台启动 Godot
#[cfg(target_os = "windows")]
fn launch_for_current_platform(exec_path: &Path) -> Result<(), String> {
    use std::os::windows::process::CommandExt;

    // 使用 CMD /C START 启动，允许多个实例
    Command::new("cmd")
        .args(["/C", "start", "", &exec_path.display().to_string()])
        .spawn()
        .map_err(|e| format!("Failed to start Godot: {}", e))?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn launch_for_current_platform(exec_path: &Path) -> Result<(), String> {
    Command::new(exec_path)
        .spawn()
        .map_err(|e| format!("Failed to start Godot: {}", e))?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn launch_for_current_platform(exec_path: &Path) -> Result<(), String> {
    Command::new("open")
        .arg(exec_path)
        .spawn()
        .map_err(|e| format!("Failed to start Godot: {}", e))?;

    Ok(())
}

/// 检查 Godot 可执行文件是否有效
pub fn validate_godot_executable(exec_path: &Path) -> bool {
    exec_path.exists() && exec_path.is_file()
}

/// 获取 Godot 版本信息
///
/// 注意：这需要 Godot 进程支持 --version 参数
pub fn get_godot_version(exec_path: &Path) -> Result<String, String> {
    if !validate_godot_executable(exec_path) {
        return Err("Invalid Godot executable".to_string());
    }

    let output = Command::new(exec_path)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to execute Godot: {}", e))?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(version)
    } else {
        Err(format!("Godot version check failed: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

/// 检查两个路径是否指向同一个文件
pub fn is_same_file(path1: &Path, path2: &Path) -> bool {
    match (path1.canonicalize(), path2.canonicalize()) {
        (Ok(p1), Ok(p2)) => p1 == p2,
        _ => false,
    }
}
