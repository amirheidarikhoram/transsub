use std::path::PathBuf;

use crate::iso_639::LanguageCodes;
pub use clap::{arg, Command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // TODO: skip possible values display for source and target languages
    #[arg(value_enum)]
    pub source_lang: LanguageCodes,
    // TODO: skip possible values display for source and target languages
    #[arg(value_enum)]
    pub target_lang: LanguageCodes,
    #[arg(short, long, value_name = "DIR")]
    pub dir: Option<PathBuf>,
    #[arg(short, long, value_name = "OUT")]
    pub out: Option<PathBuf>,
    #[arg(short, long, value_name = "TEXT")]
    pub txt: Option<String>,
}
