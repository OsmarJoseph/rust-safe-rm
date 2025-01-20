use clap::Parser;
use same_rm::{args::Opts, file_logger::FileLogger, safe_rm::SafeRm, trash_manager::TrashManager};

fn main() {
    let (full_path, opts) = validation();
    let safe_rm = factory();
    safe_rm.confirm_and_delete(&full_path, opts.force);
}

fn factory() -> SafeRm {
    let file_logger = FileLogger::new(std::path::PathBuf::from("/var/log/safe-rm.log"));
    let trash_manager = TrashManager::new();

    SafeRm::new(file_logger, trash_manager)
}

fn validation() -> (std::path::PathBuf, Opts) {
    let opts = Opts::parse();
    let path = opts.args.get(0);

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

    (full_path, opts)
}
