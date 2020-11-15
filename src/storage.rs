#[derive(Debug, Default)]
pub struct Storage {
    pub repo_path: std::path::PathBuf,
    pub filter_text: String,
    pub message: String,
}
