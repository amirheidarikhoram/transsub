mod args;
mod entry;
mod file;
mod iso_639;
mod translate;
mod utils;

use args::*;
use clap::Parser;
use translate::translate;
use utils::{log_error, log_success};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(txt) = args.txt {
        let res = translate(args.source_lang, args.target_lang, txt).await;

        if let Ok(res) = res {
            println!("{}", res);
        } else {
            println!("");
        }
    } else if let Some(dir) = args.dir {
        let files = file::get_files(dir.to_str().unwrap(), true);
        for file in files {
            let entries = file::read_file(file.as_str());

            if let Ok(entries) = entries {
                for entry in entries.iter() {
                    if let Ok(translated_text) =
                        translate(args.source_lang, args.target_lang, entry.text.clone()).await
                    {
                        println!(
                            "{}\n{} --> {}\n{}\n",
                            entry.id, entry.start_time, entry.end_time, translated_text
                        );
                    } else {
                        log_error("Failed to translate text");
                    }
                }
            } else {
                log_error(format!("Failed to read file {}", file.as_str()).as_str());
            }
        }
    } else {
        log_error("No text or directory provided")
    }

    log_success("Translation finished")
}
