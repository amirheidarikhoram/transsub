mod args;
mod entry;
mod file;
mod iso_639;
mod translate;
mod utils;

use args::*;
use clap::Parser;
use file::write_file;
use futures::future::join_all;
use iso_639::LanguageCodes;
use std::io::prelude::*;
use std::{fs::File, vec};
use translate::{translate, translate_file};
use utils::{log_error, log_info, log_success};

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
    } else if let Some(dir) = args.dir.clone() {
        let mut failed_files: Vec<String> = vec![];

        let output_dir: Option<String> = match args.out.clone() {
            Some(out) => {
                let mut out = out.to_str().unwrap().to_string();
                if !out.ends_with("/") {
                    out.push('/');
                }

                if !std::path::Path::new(&out).exists() {
                    std::fs::create_dir_all(&out).unwrap();
                }

                Some(out)
            }
            None => None,
        };

        let files = file::get_files(dir.to_str().unwrap(), true);
        let chunks: Vec<&[String]> = files.chunks(10).collect();

        for files in chunks {
            failed_files.extend(
                handle_file_group(
                    files,
                    args.source_lang,
                    args.target_lang,
                    output_dir.clone(),
                )
                .await
                .into_iter(),
            );
        }

        if failed_files.len() > 0 {
            log_info(format!("Failed to translate {} of files", failed_files.len()).as_str());

            let step2_failed_files = handle_file_group(
                &failed_files,
                args.source_lang,
                args.target_lang,
                output_dir.clone(),
            )
            .await;

            if step2_failed_files.len() > 0 {
                log_error(
                    format!("Couldn't translate {} of files. Generated a file containing paths to the mentioned files.", step2_failed_files.len()).as_str(),
                );

                let mut faileds_file_content = String::new();
                for file in step2_failed_files.iter() {
                    if !faileds_file_content.is_empty() {
                        faileds_file_content.push_str("\n");
                    }
                    faileds_file_content.push_str(file);
                }

                let output_file_path = match args.out {
                    Some(out) => out.clone().to_str().unwrap().to_string(),
                    None => args.dir.clone().unwrap().to_str().unwrap().to_string(),
                };

                let mut file = match File::create(output_file_path) {
                    Ok(file) => file,
                    Err(_) => {
                        log_error("Failed to create files log. Writing here:");
                        for file in failed_files.iter() {
                            println!("{}", file);
                        }

                        panic!("Exited with errors");
                    }
                };

                match file.write_all(faileds_file_content.as_bytes()) {
                    Ok(_) => (),
                    Err(_) => {
                        log_error("Failed to write to files log. Writing here:");
                        for file in failed_files.iter() {
                            println!("{}", file);
                        }

                        panic!("Exited with errors");
                    }
                }
            }
        }
    } else {
        log_error("No text or directory provided")
    }

    log_success("Translation finished")
}

async fn handle_file_group(
    files: &[String],
    source_lang: LanguageCodes,
    target_lang: LanguageCodes,
    output_dir: Option<String>,
) -> Vec<String> {
    let mut failed_files: Vec<String> = vec![];

    let mut futures = vec![];

    for file in files {
        futures.push(translate_file(source_lang, target_lang, file.clone(), true))
    }

    let groups = join_all(futures).await;

    for group in groups {
        match group {
            Ok(group) => match write_file(group.0, output_dir.clone(), group.1, target_lang) {
                Ok(_) => {}
                Err(err) => {
                    log_error(err.as_str());
                }
            },
            Err(file) => {
                failed_files.push(file);
            }
        }
    }

    failed_files
}
