use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    pub args: Vec<String>,

    #[clap(short = 'r', long = "recursive")]
    pub recursive: bool,

    // if it should ask to confirm
    #[clap(short = 'f', long = "force")]
    pub force: bool,
}
