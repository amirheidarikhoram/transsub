mod args;
mod iso_639;

use args::*;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}