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
        return Err(format!(
            "Godot executable not found at: {}",
            exec_path.display()
        ));
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

/// 以项目模式启动 Godot（直接打开指定项目）
///
/// # Arguments
/// * `exec_path`    - Godot 可执行文件 / .app bundle 路径
/// * `project_path` - 要打开的 Godot 项目目录路径
/// * `editor_mode`   - 是否以编辑器模式启动（true = 编辑器模式，false = 运行模式）
///
/// # Returns
/// * `Result<(), String>` - 启动结果
pub fn launch_godot_with_project(
    exec_path: &Path,
    project_path: &Path,
    editor_mode: bool,
) -> Result<(), String> {
    if !exec_path.exists() {
        return Err(format!(
            "Godot executable not found at: {}",
            exec_path.display()
        ));
    }

    log::info!(
        "Launching Godot: {} with project: {} (editor_mode: {})",
        exec_path.display(),
        project_path.display(),
        editor_mode
    );

    let result = launch_with_project_for_current_platform(exec_path, project_path, editor_mode);

    match result {
        Ok(_) => {
            log::info!("Godot launched with project successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to launch Godot with project: {}", e);
            Err(e)
        }
    }
}

/// 根据当前平台启动 Godot 并附带项目路径参数
#[cfg(target_os = "windows")]
fn launch_with_project_for_current_platform(
    exec_path: &Path,
    project_path: &Path,
    editor_mode: bool,
) -> Result<(), String> {
    use std::os::windows::process::CommandExt;

    let mut args = vec![
        "/C".to_string(),
        "start".to_string(),
        "".to_string(),
        exec_path.display().to_string(),
    ];

    if editor_mode {
        args.push("-e".to_string()); // Editor mode: open project in editor instead of running it
    }

    args.push("--path".to_string());
    args.push(project_path.display().to_string());

    Command::new("cmd")
        .args(&args)
        .spawn()
        .map_err(|e| format!("Failed to start Godot with project: {}", e))?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn launch_with_project_for_current_platform(
    exec_path: &Path,
    project_path: &Path,
    editor_mode: bool,
) -> Result<(), String> {
    let mut cmd = Command::new(exec_path);

    if editor_mode {
        cmd.arg("-e"); // Editor mode: open project in editor instead of running it
    }

    cmd.arg("--path")
        .arg(project_path)
        .spawn()
        .map_err(|e| format!("Failed to start Godot with project: {}", e))?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn launch_with_project_for_current_platform(
    exec_path: &Path,
    project_path: &Path,
    editor_mode: bool,
) -> Result<(), String> {
    // macOS 通过 `open` 命令启动 .app bundle，使用 `--args` 传递 Godot 参数
    let mut cmd = Command::new("open");
    cmd.arg(exec_path).arg("--args");

    if editor_mode {
        cmd.arg("-e"); // Editor mode: open project in editor instead of running it
    }

    cmd.arg("--path")
        .arg(project_path)
        .spawn()
        .map_err(|e| format!("Failed to start Godot with project: {}", e))?;

    Ok(())
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
///
/// 在 macOS 上，可执行文件路径可能是 `.app` 目录包，因此同时接受文件和目录。
#[allow(dead_code)]
pub fn validate_godot_executable(exec_path: &Path) -> bool {
    if !exec_path.exists() {
        return false;
    }
    // macOS .app bundle 是目录，其他平台是文件
    exec_path.is_file() || exec_path.is_dir()
}

/// 获取 Godot 版本信息
///
/// 注意：这需要 Godot 进程支持 --version 参数
#[allow(dead_code)]
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
        Err(format!(
            "Godot version check failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// 检查两个路径是否指向同一个文件
#[allow(dead_code)]
pub fn is_same_file(path1: &Path, path2: &Path) -> bool {
    match (path1.canonicalize(), path2.canonicalize()) {
        (Ok(p1), Ok(p2)) => p1 == p2,
        _ => false,
    }
}
