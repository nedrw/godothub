import re

with open('src/state/app_state.rs', 'r') as f:
    content = f.read()

# Add new structs
new_structs = """
/// 项目删除确认对话框状态
#[derive(Debug, Clone)]
pub struct ProjectDeleteConfirmState {
    pub path: PathBuf,
    pub name: String,
}

/// 项目打开版本不匹配警告对话框状态
#[derive(Debug, Clone)]
pub struct ProjectOpenWarnState {
    pub project_path: PathBuf,
    pub project_version: String,
    pub engine_index: usize,
    pub engine_version: String,
}
"""

content = content.replace("/// 应用程序状态\n#[derive", new_structs + "\n/// 应用程序状态\n#[derive")

# Add to AppState
new_fields = """    /// 新建项目对话框状态（不序列化，仅对话框打开时存在）
    #[serde(skip)]
    pub new_project_dialog: Option<NewProjectDialogState>,
    /// 项目删除确认对话框状态
    #[serde(skip)]
    pub project_delete_confirm: Option<ProjectDeleteConfirmState>,
    /// 项目版本不匹配警告对话框状态
    #[serde(skip)]
    pub project_open_warn: Option<ProjectOpenWarnState>,"""

content = re.sub(r"    /// 新建项目对话框状态（不序列化，仅对话框打开时存在）\n    #\[serde\(skip\)\]\n    pub new_project_dialog: Option<NewProjectDialogState>,", new_fields, content)

# Add to Clone
clone_fields = """            new_project_dialog: None, // 对话框状态不跨异步任务传递
            project_delete_confirm: None,
            project_open_warn: None,"""
content = content.replace("            new_project_dialog: None, // 对话框状态不跨异步任务传递", clone_fields)

# Add to Default
default_fields = """            new_project_dialog: None,
            project_delete_confirm: None,
            project_open_warn: None,"""
content = content.replace("            new_project_dialog: None,", default_fields)

with open('src/state/app_state.rs', 'w') as f:
    f.write(content)
