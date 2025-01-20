pub struct FileLogger {
    pub file_path: std::path::PathBuf,
}

impl FileLogger {
    pub fn new(file_path: std::path::PathBuf) -> Self {
        Self { file_path }
    }

    fn get_file_content(&self) -> String {
        if !self.file_path.exists() {
            return String::new();
        }
        std::fs::read_to_string(&self.file_path).expect("Could not read log file")
    }

    pub fn log(&self, path: &std::path::Path, path_in_trash: std::path::PathBuf) {
        let message = &format!("Moved {} to {}", path.display(), path_in_trash.display());
        let content = self.get_file_content();
        let new_content = format!("{}\n{}", content, message);
        std::fs::write(&self.file_path, new_content).expect("Could not write to log file");
    }
}
