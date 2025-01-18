use clap::Parser;
use same_rm::args::Opts;

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

fn create_path_in_trash(path: &std::path::Path) -> std::path::PathBuf {
    let trash_path = get_trash_path();

    let mut path_in_trash = trash_path.join(path.file_name().expect("could not get filename"));

    let path_exists_in_trash = std::fs::metadata(&path_in_trash).is_ok();

    if path_exists_in_trash {
        path_in_trash = path_in_trash.with_extension(rand::random::<u32>().to_string());
    }

    if path.is_dir() {
        std::fs::create_dir_all(&path_in_trash).expect("Could not create path in trash");
    }

    path_in_trash
}

fn delete(path: &std::path::Path, file_logger: FileLogger) {
    let path_in_trash = create_path_in_trash(path);

    println!(
        "Moving from: {} to {}",
        path.display(),
        path_in_trash.display()
    );

    let result = std::fs::rename(path, &path_in_trash);

    match result {
        Ok(_) => {
            file_logger.log(path, path_in_trash);
        }
        Err(e) => {
            eprintln!("Could not move file: {}", e);
            std::process::exit(1);
        }
    }
}

fn confirm_and_delete(path: &std::path::Path, force: bool, file_logger: FileLogger) {
    if force {
        delete(path, file_logger);
        return;
    }

    let mut input = String::new();
    println!("rm: remove {}? ", path.display());
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input != "y" {
        std::process::exit(0);
    }

    delete(path, file_logger);
}

fn main() {
    let opts = Opts::parse();
    let path = opts.args.get(0);

    let file_logger = FileLogger::new(std::path::PathBuf::from("/var/log/safe-rm.log"));

    if let None = path {
        eprintln!("No path provided");
        std::process::exit(1);
    }

    let path = path.unwrap();

    let full_path = std::fs::canonicalize(path).expect("Invalid path");

    let is_dir = full_path.is_dir();

    if is_dir && !opts.recursive {
        eprintln!("rm: {}: is a directory", full_path.display());
        std::process::exit(1);
    }

    confirm_and_delete(&full_path, opts.force, file_logger);
}
