use std::path::PathBuf;

use crate::iso_639::LanguageCodes;
pub use clap::{arg, Command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // TODO: skip possible values display for source and target languages
    #[arg(value_enum)]
    source_lang: LanguageCodes,
    // TODO: skip possible values display for source and target languages
    #[arg(value_enum)]
    target_lang: LanguageCodes,
    #[arg(short, long, value_name = "DIR")]
    dir: Option<PathBuf>,
}
