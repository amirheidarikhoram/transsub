mod args;
mod entry;
mod file;
mod iso_639;
mod translate;
mod utils;

use args::*;
use clap::Parser;
use futures::future::join_all;
use translate::{translate, translate_file};
use utils::{log_error, log_success};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(txt) = args.txt {
        let res = translate(args.source_lang, args.target_lang, txt, None).await;

        if let Ok(res) = res {
            println!("{}", res.1);
        } else {
            println!("");
        }
    } else if let Some(dir) = args.dir {
        let output_dir: Option<String> = match args.out {
            Some(out) => {
                let out = out.to_str().unwrap().to_string();
                if out.ends_with("/") {
                    Some(out)
                } else {
                    Some(out + "/")
                }
            }
            None => None,
        };

        let files = file::get_files(dir.to_str().unwrap(), true);
        let chunks = files.chunks(10);

        for files in chunks.into_iter() {
            let mut futures = vec![];

            for file in files {
                futures.push(translate_file(
                    args.source_lang,
                    args.target_lang,
                    file.clone(),
                    output_dir.clone(),
                    true,
                ))
            }

            let _ = join_all(futures).await;
        }
    } else {
        log_error("No text or directory provided")
    }

    log_success("Translation finished")
}
