# Godot Hub 架构文档

## 1. 项目概述

Godot Hub 是一个使用 Rust + eframe/egui 构建的跨平台 Godot 引擎管理应用，类似于 GodotHub，允许用户管理多个 Godot 版本、安装、运行和下载不同的 Godot 变体。

### 核心功能

- **版本管理**: 安装、删除、运行多个 Godot 版本
- **变体支持**: Standard、Mono、Export Templates
- **版本下载**: 从 GitHub 官方仓库下载 Godot
- **项目管理**: 项目目录管理和扫描
- **配置管理**: 自定义安装目录、项目目录等

### 项目特点

- **现代化 UI**: 采用卡片式设计，清晰的信息层次
- **跨平台**: 支持 Windows、macOS、Linux
- **异步处理**: 使用 tokio 进行异步操作
- **类型安全**: Rust 的类型系统保证代码质量

## 2. 技术栈

| 组件 | 技术 | 版本 | 用途 |
|------|------|------|------|
| UI 框架 | eframe + egui | 0.31 | 即时模式 GUI |
| 异步运行时 | tokio | 1.x | 异步任务处理 |
| HTTP 客户端 | reqwest | 0.12 | HTTP 请求和下载 |
| 序列化 | serde + serde_json | 1.0 | 数据序列化 |
| 文件处理 | zip | 2.0 | ZIP 文件解压 |
| 错误处理 | thiserror, anyhow | 2.0, 1.0 | 错误处理 |
| 日期时间 | chrono | 0.4 | 时间处理 |
| 日志 | log, env_logger | 0.4, 0.11 | 日志记录 |

## 3. 模块架构

重构后的代码按功能划分为以下模块：

```
src/
├── main.rs                 # 应用程序入口点
├── models/                 # 数据模型层
│   ├── mod.rs
│   ├── godot_version.rs   # GodotVersion, 版本信息模型
│   ├── godot_variant.rs   # GodotVariant, 变体枚举
│   └── godot_install.rs   # GodotInstall, 已安装版本模型
├── state/                 # 应用状态层
│   ├── mod.rs
│   ├── app_config.rs      # AppConfig, 配置模型
│   ├── app_state.rs       # AppState, 应用状态
│   └── app_state_impl.rs # AppState 实现方法
├── ui/                    # UI 表现层
│   ├── mod.rs
│   ├── sidebar.rs         # 侧边栏组件（已优化）
│   ├── versions_panel.rs # 版本管理面板（已优化）
│   ├── projects_panel.rs # 项目管理面板（已优化）
│   ├── settings_panel.rs # 设置面板（已优化）
│   └── download_dialog.rs # 下载对话框（已优化）
├── services/              # 业务逻辑层
│   ├── mod.rs
│   ├── download.rs       # 下载服务
│   └── launcher.rs       # 启动器服务
└── utils/                 # 工具函数层
    ├── mod.rs
    └── file_utils.rs     # 文件操作工具
```

### 模块职责

#### Models 层
- 定义数据结构和业务对象
- 实现数据验证和转换
- 提供数据序列化/反序列化

#### State 层
- 管理应用全局状态
- 处理状态持久化
- 提供状态访问和修改接口

#### UI 层
- 实现用户界面组件
- 处理用户交互
- 展示数据状态

#### Services 层
- 实现核心业务逻辑
- 处理外部资源访问
- 提供异步操作支持

#### Utils 层
- 提供通用工具函数
- 封装平台相关操作
- 提供辅助功能

## 4. 数据模型

### 4.1 GodotVariant

表示 Godot 的不同版本类型：

```rust
pub enum GodotVariant {
    Standard,        // 标准版
    Mono,           // Mono 版本（支持 C#）
    ExportTemplates, // 导出模板
}
```

### 4.2 GodotVersion

表示一个可用的 Godot 版本信息：

```rust
pub struct GodotVersion {
    pub version: String,         // 版本号 (如 "4.3")
    pub variant: GodotVariant,   // 变体类型
    pub platform: String,        // 平台 (如 "Linux64")
    pub download_url: String,    // 下载链接
    pub release_date: String,    // 发布日期
    pub is_installed: bool,     // 是否已安装
    pub install_path: Option<PathBuf>, // 安装路径
}
```

### 4.3 GodotInstall

表示已安装的 Godot 实例：

```rust
pub struct GodotInstall {
    pub version: String,                    // 版本号
    pub variant: GodotVariant,               // 变体类型
    pub path: PathBuf,                       // 可执行文件路径
    pub is_favorite: bool,                  // 是否收藏
    pub last_used: Option<DateTime<Utc>>,   // 最后使用时间
}
```

### 4.4 AppConfig

应用配置：

```rust
pub struct AppConfig {
    pub install_dir: PathBuf,           // 安装目录
    pub projects_dir: PathBuf,          // 项目目录
    pub check_updates_on_start: bool,   // 启动时检查更新
}
```

### 4.5 AppState

全局应用状态：

```rust
pub struct AppState {
    pub installed_versions: Vec<GodotInstall>,      // 已安装版本
    pub available_versions: Vec<GodotVersion>,      // 可用版本
    pub downloads_in_progress: HashMap<String, f32>,// 下载进度
    pub selected_version_index: Option<usize>,      // 选中版本索引
    pub show_download_dialog: bool,                 // 显示下载对话框
    pub current_tab: MainTab,                       // 当前标签页
    pub config: AppConfig,                          // 应用配置
}
```

## 5. UI 结构

### 5.1 整体布局

应用采用侧边栏 + 中央面板的布局：

```
┌─────────────────────────────────────────────────────────┐
│  Godot Hub - v0.1.0                        [─][□][×]  │
├────────────────┬────────────────────────────────────────┤
│                │                                        │
│  🎮 Godot Hub  │     [面板标题 + 描述 + 操作按钮]      │
│  Engine Mgr    │                                        │
│                │     ┌──────────────────────────────┐  │
│  ───────────── │     │                              │  │
│                │     │     主内容区域               │  │
│  NAVIGATION    │     │     (卡片式布局)             │  │
│  📦 Versions   │     │                              │  │
│  📁 Projects   │     │     - 版本卡片               │  │
│  ⚙️ Settings   │     │     - 项目卡片               │  │
│                │     │     - 设置分组               │  │
│  ───────────── │     │                              │  │
│                │     └──────────────────────────────┘  │
│  STATISTICS    │                                        │
│  ┌──────────┐ │                                        │
│  │📦 3      │ │                                        │
│  │Installed │ │                                        │
│  └──────────┘ │                                        │
│                │                                        │
│  ┌──────────┐ │                                        │
│  │⬇️ 1      │ │                                        │
│  │Download  │ │                                        │
│  └──────────┘ │                                        │
│                │                                        │
│  [⬇️ Download]│                                        │
│                │                                        │
└────────────────┴────────────────────────────────────────┘
```

### 5.2 侧边栏组件 (Sidebar)

**功能特点：**
- 应用标题和版本信息
- 导航按钮（带图标和状态反馈）
- 统计信息卡片
- 底部固定下载按钮

**设计规范：**
- 宽度范围：200-300px
- 默认宽度：220px
- 使用 emoji 图标增强可识别性
- 选中状态使用蓝色高亮

### 5.3 版本管理面板 (Versions Panel)

**功能特点：**
- 已安装版本展示（卡片式布局）
- 可用版本分组显示（按主版本号）
- 状态标签（Standard/Mono/Installed）
- 下载进度条显示
- 操作菜单（运行/打开文件夹/删除）

**设计规范：**
- 卡片内边距：12px
- 卡片圆角：8px
- 操作按钮最小尺寸：64x28px
- 进度条宽度：120px

### 5.4 项目管理面板 (Projects Panel)

**功能特点：**
- 项目扫描和展示
- 项目有效性检测
- 快捷操作（打开/打开文件夹）
- 空状态友好提示

**设计规范：**
- 项目卡片显示路径、版本、最后打开时间
- 无效项目使用红色标签标识
- 收藏项目使用星标显示

### 5.5 设置面板 (Settings Panel)

**功能特点：**
- 目录配置（安装目录、项目目录）
- 行为设置（启动检查、自动启动）
- 主题选择（占位）
- 关于信息

**设计规范：**
- 使用卡片分组
- 每个设置项包含标题、说明、控件
- 提供重置默认选项

### 5.6 下载对话框 (Download Dialog)

**功能特点：**
- 版本分组显示（Godot 4.x / 3.x）
- 搜索和筛选功能（占位）
- 下载队列状态显示
- 实时进度更新

**设计规范：**
- 默认尺寸：650x550px
- 最小宽度：550px
- 模态居中显示

## 6. UI 设计规范

### 6.1 颜色系统

```rust
// 语义颜色
Primary:    Color32::from_rgb(70, 130, 180)   // 主要操作
Success:    Color32::from_rgb(46, 139, 87)    // 成功状态
Warning:    Color32::from_rgb(255, 165, 0)    // 警告状态
Error:      Color32::from_rgb(220, 53, 69)    // 错误状态
Info:       Color32::from_rgb(70, 130, 180)   // 信息提示

// 变体标签颜色
Standard:   Color32::from_rgb(76, 175, 80)    // 标准版
Mono:       Color32::from_rgb(156, 39, 176)   // Mono 版
Export:     Color32::from_rgb(255, 152, 0)    // 导出模板
```

### 6.2 字体规范

```rust
// 标题
Heading 1: 20px, bold    // 应用标题
Heading 2: 18px, bold    // 面板标题
Heading 3: 16px, bold    // 区域标题

// 正文
Body:      14px, normal // 正文内容
Small:     12px, normal // 辅助信息
Tiny:      10px, normal // 标签、提示

// 特殊样式
Strong:    加粗文本
Weak:      弱化文本
Code:      代码样式（路径）
```

### 6.3 间距规范

```rust
xs:  4px  // 元素内部间距
sm:  8px  // 紧凑元素间距
md:  16px // 标准元素间距
lg:  24px // 区域间距
xl:  32px // 大块区域间距
```

### 6.4 组件规范

**卡片组件：**
```rust
Frame::group(ui.style())
    .inner_margin(12.0)
    .outer_margin(0.0)
    .rounding(8.0)
    .stroke(Stroke::new(1.0, border_color))
    .show(ui, |ui| {
        // 卡片内容
    });
```

**按钮组件：**
```rust
// 主要按钮
Button::new("Text")
    .fill(Color32::from_rgb(70, 130, 180))
    .min_size(Vec2::new(120.0, 32.0))

// 次要按钮
Button::new("Text")
    .fill(Color32::TRANSPARENT)

// 危险按钮
Button::new("Delete")
    .fill(Color32::from_rgb(220, 53, 69))
```

**进度条组件：**
```rust
ProgressBar::new(progress)
    .desired_width(120.0)
    .text(format!("{:.0}%", progress * 100.0))
    .animate(true)
```

## 7. 业务流程

### 7.1 启动流程

```
1. 初始化日志系统
   ↓
2. 创建 GodotHubApp 实例
   ↓
3. 加载 AppConfig（或使用默认值）
   ↓
4. 扫描已安装版本目录
   ↓
5. 加载可用版本列表（模拟数据）
   ↓
6. 启动 eframe 渲染循环
   ↓
7. 首次渲染 UI
```

### 7.2 版本下载流程

```
用户点击下载按钮
   ↓
检查版本是否已下载
   ↓
创建下载任务
   ↓
更新 downloads_in_progress
   ↓
启动异步下载
   ↓
更新下载进度
   ↓
下载完成，解压文件
   ↓
更新安装列表
   ↓
清理下载状态
```

### 7.3 版本启动流程

```
用户点击运行按钮
   ↓
验证可执行文件存在
   ↓
根据平台选择启动方式
   ├─ Windows: cmd /C start "" <path>
   ├─ Linux: 直接执行
   └─ macOS: open <path>
   ↓
记录最后使用时间
   ↓
更新 UI 状态
```

## 8. 错误处理

### 8.1 错误类型

使用 `thiserror` 定义具体错误：

```rust
#[derive(Debug, thiserror::Error)]
pub enum GodotHubError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Config error: {0}")]
    Config(String),
}
```

### 8.2 错误处理策略

1. **用户级错误**: 显示友好的错误提示
2. **系统级错误**: 记录日志并尝试恢复
3. **网络错误**: 提供重试机制

## 9. 配置持久化

### 9.1 配置文件位置

- **配置目录**: `{config_dir}/gdhub/config.json`
- **Linux**: `~/.config/gdhub/config.json`
- **macOS**: `~/Library/Application Support/gdhub/config.json`
- **Windows**: `%APPDATA%/gdhub/config.json`

### 9.2 配置示例

```json
{
  "install_dir": "/home/user/.gdhub/versions",
  "projects_dir": "/home/user/Godot",
  "check_updates_on_start": true
}
```

## 10. 最近更新

### v0.1.3 (2025-03-02) - 中国镜像支持与地区自动检测

#### 问题修复
- ✅ 修复切换到中国镜像时获取引擎版本出错的 bug
- ✅ 优化 API URL 构建逻辑，使用 `full_api_url` 方法正确拼接镜像 URL
- ✅ 改进日志输出，便于调试镜像源请求
- ✅ **新增镜像回退机制**：当中国镜像服务不可用时，自动回退到 GitHub 官方 API，确保用户始终可以获取版本列表和下载引擎文件
- ✅ **下载内容验证**：新增 `validate_zip_file` 函数，在解压前验证下载的文件是否为有效的 ZIP 格式
- ✅ **下载回退机制**：如果镜像下载失败，自动尝试 GitHub 官方 URL 进行下载

#### 新增功能
- ✅ **地区自动检测**：新增 `region` 模块，自动检测用户时区和语言设置
- ✅ **自动切换镜像站**：在中国地区（检测到 Asia/Shanghai 等时区或 zh_CN 等语言时）自动切换为 `ChinaMirror` 下载源
- ✅ **手动选择镜像源**：支持 GitHub 官方源、ghproxy.com 镜像、gitclone.com 镜像三种选项
- ✅ **自动检测开关**：在设置面板中添加"Auto-detect region"选项，可手动开启或关闭地区自动检测
- ✅ **实时检测显示**：开启自动检测后，实时显示当前检测到的地区（🇨🇳 China / 🌍 International）

#### 技术改进
- ✅ 新增 `src/utils/region.rs` 模块，提供 `is_china_timezone()` 和 `has_chinese_locale()` 检测函数
- ✅ `AppConfig` 新增 `auto_detect_region` 配置项，默认启用
- ✅ `DownloadSource` 新增 `full_api_url()` 方法，正确处理镜像 URL 拼接
- ✅ 启动时自动检测地区并设置合适的下载源

#### 配置示例
```json
{
  "install_dir": "/Users/user/.gdhub/versions",
  "projects_dir": "/Users/user/Godot",
  "check_updates_on_start": true,
  "theme": "Dark",
  "download_source": "ChinaMirror",
  "auto_detect_region": true
}
```

### v0.1.3 已知问题
⚠️ **注意**：部分中国镜像服务（如 ghproxy.com）目前存在不稳定或已更换域名的情况。代码已实现自动回退机制，当检测到镜像不可用时会自动切换到官方 GitHub API。在中国大陆使用时，如遇到版本列表加载失败或下载报错，请检查网络连接或手动切换下载源（设置面板 → Download Source → GitHub）。


### v0.1.2 (2025-01-17) - 下载功能修复

#### 问题修复
- ✅ 修复点击下载后只显示假进度条的 bug
- ✅ 实现真实的下载进度更新机制
- ✅ 下载完成后自动将版本添加到已安装列表
- ✅ 支持流式下载，实时报告下载进度

#### 技术改进
- ✅ 使用共享状态机制 (`Arc<Mutex<AppState>>`) 实现异步任务与主线程状态同步
- ✅ 实现流式下载支持大文件进度报告
- ✅ 添加解压后自动查找 Godot 可执行文件功能
- ✅ 移除假的进度条模拟代码

#### 架构优化
- ✅ `AppState` 实现手动 `Clone`，处理无法克隆的字段（Runtime、Receiver）
- ✅ 创建专门的共享状态版本供异步任务使用
- ✅ 下载服务通过回调机制实时更新 UI 进度

### v0.1.1 (2025-01-16) - UI 优化

#### 侧边栏优化
- ✅ 添加应用标题区域，显示应用名称和版本
- ✅ 使用 emoji 图标增强导航按钮可识别性
- ✅ 实现导航按钮选中状态高亮
- ✅ 添加统计信息卡片显示（已安装、可用、下载中）
- ✅ 优化下载按钮位置和样式
- ✅ 添加工具提示

#### 版本管理面板优化
- ✅ 实现卡片式布局展示版本信息
- ✅ 添加变体和状态标签（Standard/Mono/Installed）
- ✅ 优化操作按钮布局，添加下拉菜单
- ✅ 实现版本分组显示（Godot 4.x / 3.x）
- ✅ 添加空状态友好提示
- ✅ 实现路径截断和悬停显示完整路径
- ✅ 修复进度条显示问题

#### 项目管理面板优化
- ✅ 实现卡片式项目展示
- ✅ 添加项目有效性检测和标签
- ✅ 优化空状态提示
- ✅ 添加快捷操作按钮
- ✅ 实现项目目录扫描功能

#### 设置面板优化
- ✅ 使用卡片式分组
- ✅ 添加目录快捷操作（打开文件夹）
- ✅ 优化设置项布局和说明
- ✅ 添加主题选择占位

#### 下载对话框优化
- ✅ 增加对话框尺寸和可调整性
- ✅ 实现版本分组显示
- ✅ 添加下载队列状态显示
- ✅ 优化进度条显示
- ✅ 添加取消所有下载功能
- ✅ 添加搜索栏占位

### 设计改进
- ✅ 统一使用卡片式布局
- ✅ 实现清晰的信息层次
- ✅ 添加状态标签和图标
- ✅ 优化间距和对齐
- ✅ 添加工具提示和悬停效果
- ✅ 实现响应式设计基础

## 11. 后续开发计划

### 短期目标 (v0.2.0)
- [x] 集成 GitHub API 获取真实版本列表
- [x] 实现真实下载和解压功能
- [ ] 添加文件选择对话框
- [ ] 实现删除版本功能
- [ ] 添加状态持久化

### 中期目标 (v0.3.0)
- [ ] 完善项目管理功能
- [ ] 实现主题切换
- [ ] 添加键盘快捷键
- [ ] 优化错误处理和提示

### 长期目标 (v1.0.0)
- [ ] 实现自动更新检查
- [ ] 添加插件系统
- [ ] 支持多语言
- [ ] 完整的测试覆盖

## 12. 技术债务

### 待优化项
1. **状态管理**: AppState 需要更好的关注点分离
2. **异步处理**: 下载服务需要完整的错误处理和重试机制
3. **测试覆盖**: 缺少单元测试和集成测试
4. **性能优化**: 大量版本时的渲染性能优化

### 代码规范
- 遵循 Rust API 指南
- 使用 clippy 进行代码检查
- 保持函数长度适中
- 添加必要的文档注释

## 13. 参考资源

- [egui 官方文档](https://docs.rs/egui/)
- [eframe 示例](https://github.com/emilk/egui/tree/master/examples)
- [Godot 官方网站](https://godotengine.org/)
- [Godot GitHub Releases](https://github.com/godotengine/godot/releases)
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)

---

**文档版本**: 1.1  
**最后更新**: 2025-01-16  
**维护者**: Godot Hub 开发团队