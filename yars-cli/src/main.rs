use clap::{crate_authors, crate_description, crate_version, Clap};
use std::io::{self, prelude::*};
use std::path::PathBuf;

use yars_lib::processor::ProcessorError;
use yars_lib::simulator::Simulator;

#[derive(Clap)]
#[clap(name = "yars")]
#[clap(author = crate_authors!())]
#[clap(version = crate_version!())]
#[clap(about = crate_description!())]
struct Opts {
    #[clap(short, long)]
    #[clap(about = "Logs instruction execution")]
    log: bool,

    #[clap(short, long)]
    #[clap(about = "Runs the program interactively")]
    interactive: bool,

    #[clap(short, long, value_name = "size", default_value = "32")]
    #[clap(about = "Allocate <size> MiB for target memory")]
    memory: u32,

    #[clap(long, value_name = "address")]
    #[clap(about = "Override program entry point")]
    pc: Option<u32>,

    #[clap(about = "Path to target RISC-V program")]
    program: PathBuf,
}

fn main() {
    let opts = Opts::parse();
    let stdout = io::stdout();

    let memory = opts.memory * 1048576;
    let logger = match opts.log {
        true => Some(stdout.lock()),
        false => None,
    };

    let mut sim = Simulator::new(opts.program, memory, opts.pc, logger).unwrap();
    match opts.interactive {
        false => sim.run(),
        true => loop {
            match sim.step() {
                Ok(()) => {
                    io::stdin().read(&mut [0u8]).unwrap();
                    continue;
                }
                Err(ProcessorError::Ecall) | Err(ProcessorError::Ebreak) => break Ok(()),
                e => break e,
            }
        },
    }
    .unwrap();

    println!("Program finished (Total cycles: {}).", sim.cycles());
}
