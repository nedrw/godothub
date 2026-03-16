// FileUtils - 文件操作工具函数
#![allow(dead_code)]

use std::path::Path;

use std::path::PathBuf;

/// 确保目录存在，如果不存在则创建
///
/// # Arguments
/// * `path` - 目录路径
///
/// # Returns
/// * `Result<(), std::io::Error>` - 操作结果
pub fn ensure_directory(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 递归删除目录
///
/// # Arguments
/// * `path` - 要删除的目录路径
///
/// # Returns
/// * `Result<(), String>` - 操作结果
pub fn remove_directory_recursive(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_dir() {
        std::fs::remove_dir_all(path)
            .map_err(|e| format!("Failed to remove directory {}: {}", path.display(), e))
    } else {
        std::fs::remove_file(path)
            .map_err(|e| format!("Failed to remove file {}: {}", path.display(), e))
    }
}

/// 获取文件大小（字节）
///
/// # Arguments
/// * `path` - 文件路径
///
/// # Returns
/// * `Option<u64>` - 文件大小，如果文件不存在则返回 None
pub fn get_file_size(path: &Path) -> Option<u64> {
    std::fs::metadata(path).ok().map(|m| m.len())
}

/// 格式化文件大小为人类可读格式
///
/// # Arguments
/// * `bytes` - 字节数
///
/// # Returns
/// * `String` - 格式化后的大小字符串
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// 检查路径是否为有效的 Godot 可执行文件
///
/// # Arguments
/// * `path` - 要检查的路径
///
/// # Returns
/// * `bool` - 是否为有效的 Godot 可执行文件
pub fn is_valid_godot_executable(path: &Path) -> bool {
    if !path.exists() || !path.is_file() {
        return false;
    }

    // 检查文件名是否匹配 Godot 可执行文件命名模式
    if let Some(name) = path.file_name() {
        let name_str = name.to_string_lossy().to_lowercase();
        return name_str.starts_with("godot");
    }

    false
}

/// 在目录中查找符合条件的文件
///
/// # Arguments
/// * `dir` - 要搜索的目录
/// * `predicate` - 文件名匹配谓词
///
/// # Returns
/// * `Vec<PathBuf>` - 匹配的文件列表
pub fn find_files_in_directory<F>(dir: &Path, predicate: F) -> Vec<PathBuf>
where
    F: Fn(&str) -> bool,
{
    let mut results = Vec::new();

    if !dir.exists() || !dir.is_dir() {
        return results;
    }

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if predicate(&name.to_string_lossy()) {
                        results.push(path);
                    }
                }
            }
        }
    }

    results
}

/// 获取用户主目录下的标准路径
///
/// # Arguments
/// * `relative_path` - 相对于主目录的路径
///
/// # Returns
/// * `Option<PathBuf>` - 完整路径
pub fn get_home_directory_path(relative_path: &str) -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(relative_path))
}

/// 获取配置目录路径
///
/// # Arguments
/// * `app_name` - 应用名称
///
/// # Returns
/// * `Option<PathBuf>` - 配置目录路径
pub fn get_config_dir(app_name: &str) -> Option<PathBuf> {
    dirs::config_dir().map(|c| c.join(app_name))
}

/// 获取数据目录路径
///
/// # Arguments
/// * `app_name` - 应用名称
///
/// # Returns
/// * `Option<PathBuf>` - 数据目录路径
pub fn get_data_dir(app_name: &str) -> Option<PathBuf> {
    dirs::data_dir().map(|d| d.join(app_name))
}

/// 在系统文件管理器中打开指定路径的文件夹（跨平台）
///
/// # Arguments
/// * `path` - 要打开的目录路径
///
/// # Platform Notes
/// - macOS: 使用 `open` 命令
/// - Linux: 使用 `xdg-open` 命令
/// - Windows: 使用 `explorer` 命令
pub fn open_folder(path: &Path) {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn().ok();
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(path)
            .spawn()
            .ok();
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .ok();
    }
}
