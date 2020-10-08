use clap::{Clap, crate_authors, crate_description, crate_version};
use std::path::PathBuf;

#[derive(Clap)]
#[clap(name = "yars")]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
#[clap(about = crate_description!())]
struct Opts {
    #[clap(short, long)]
    #[clap(about = "Logs instruction execution")]
    log: bool,

    #[clap(short, long, value_name = "size", default_value = "32")]
    #[clap(about = "Allocate <size> MiB for target memory")]
    memory: usize,

    #[clap(long, value_name = "address")]
    #[clap(about = "Override program entry point")]
    pc: Option<usize>,

    #[clap(about = "Path to target RISC-V program")]
    program: PathBuf,
}

fn main() {
    let opts = Opts::parse();
}
