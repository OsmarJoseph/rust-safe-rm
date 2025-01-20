pub struct TrashManager {
    pub trash_path: std::path::PathBuf,
}

impl TrashManager {
    pub fn new() -> Self {
        let trash_path = Self::get_trash_path();
        TrashManager { trash_path }
    }
    fn get_trash_path() -> std::path::PathBuf {
        let home = std::env::var("HOME").expect("HOME is not set");
        let trash = ".Trash";

        let trash_path = std::path::Path::new(&home).join(trash);

        if !trash_path.exists() {
            eprintln!("Trash does not exist");
            std::process::exit(1);
        }

        trash_path
    }

    pub fn create_path_in_trash(&self, path: &std::path::Path) -> std::path::PathBuf {
        let mut path_in_trash = self
            .trash_path
            .join(path.file_name().expect("could not get filename"));

        let path_exists_in_trash = std::fs::metadata(&path_in_trash).is_ok();

        if path_exists_in_trash {
            path_in_trash = path_in_trash.with_extension(rand::random::<u32>().to_string());
        }

        if path.is_dir() {
            std::fs::create_dir_all(&path_in_trash).expect("Could not create path in trash");
        }

        path_in_trash
    }
}
