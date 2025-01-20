use crate::{file_logger::FileLogger, trash_manager::TrashManager};

pub struct SafeRm {
    file_logger: FileLogger,
    trash_manager: TrashManager,
}

impl SafeRm {
    pub fn new(file_logger: FileLogger, trash_manager: TrashManager) -> Self {
        SafeRm {
            file_logger,
            trash_manager,
        }
    }

    pub fn confirm_and_delete(&self, path: &std::path::Path, force: bool) {
        if force {
            self.delete(path);
            return;
        }

        let mut input = String::new();
        println!("rm: remove {}? ", path.display());
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input != "y" {
            std::process::exit(0);
        }

        self.delete(path);
    }
    fn delete(&self, path: &std::path::Path) {
        let path_in_trash = self.trash_manager.create_path_in_trash(path);

        println!(
            "Moving from: {} to {}",
            path.display(),
            path_in_trash.display()
        );

        let result = std::fs::rename(path, &path_in_trash);

        match result {
            Ok(_) => {
                self.file_logger.log(path, path_in_trash);
            }
            Err(e) => {
                eprintln!("Could not move file: {}", e);
                std::process::exit(1);
            }
        }
    }
}
