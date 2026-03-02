// Region 模块 - 地区检测功能
// 用于检测用户所在地区，自动选择合适的下载源

use std::process::Command;

/// 检测当前系统时区是否为亚洲/中国时区
/// 这是一个简单的检测方法，通过检查时区设置来判断
pub fn is_china_timezone() -> bool {
    // 检查常见的中国时区
    let china_timezones = [
        "Asia/Shanghai",
        "Asia/Hong_Kong",
        "Asia/Macau",
        "Asia/Chongqing",
        "Asia/Harbin",
        "PRC",
        "CST",
    ];

    // 读取系统时区
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("systemsetup")
            .arg("-gettimezone")
            .output()
        {
            let timezone = String::from_utf8_lossy(&output.stdout);
            for tz in &china_timezones {
                if timezone.contains(tz) {
                    return true;
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 尝试读取 /etc/timezone
        if let Ok(content) = std::fs::read_to_string("/etc/timezone") {
            for tz in &china_timezones {
                if content.contains(tz) {
                    return true;
                }
            }
        }
        // 也可以检查 TZ 环境变量
        if let Ok(tz) = std::env::var("TZ") {
            for ctz in &china_timezones {
                if tz.contains(ctz) {
                    return true;
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows 上检查注册表或时区名称
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "(Get-TimeZone).Id"])
            .output()
        {
            let timezone = String::from_utf8_lossy(&output.stdout);
            if timezone.contains("China") || timezone.contains("Shanghai") || timezone.contains("Hongkong") {
                return true;
            }
        }
    }

    false
}

/// 检测当前系统语言是否包含中文
pub fn has_chinese_locale() -> bool {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
        {
            let locale = String::from_utf8_lossy(&output.stdout);
            if locale.contains("zh") {
                return true;
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(locale) = std::env::var("LANG") {
            if locale.contains("zh_CN") || locale.contains("zh-CN")
                || locale.contains("zh_HK") || locale.contains("zh_TW") {
                return true;
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "(Get-Culture).Name"])
            .output()
        {
            let culture = String::from_utf8_lossy(&output.stdout);
            if culture.starts_with("zh") {
                return true;
            }
        }
    }

    false
}

/// 综合判断是否应该使用中国镜像源
/// 考虑时区和语言因素
pub fn should_use_china_mirror() -> bool {
    // 如果时区在中国，直接返回 true
    if is_china_timezone() {
        log::info!("Detected China timezone, will use China mirror");
        return true;
    }

    // 如果系统语言是中文，也返回 true
    if has_chinese_locale() {
        log::info!("Detected Chinese locale, will use China mirror");
        return true;
    }

    log::info!("Not in China region, using default source");
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone_detection() {
        // 这个测试会因系统配置不同而结果不同
        let _ = is_china_timezone();
    }

    #[test]
    fn test_chinese_locale() {
        let _ = has_chinese_locale();
    }

    #[test]
    fn test_should_use_china_mirror() {
        let _ = should_use_china_mirror();
    }
}
