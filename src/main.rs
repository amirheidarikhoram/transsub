mod args;
mod iso_639;
mod translate;

use args::*;
use clap::Parser;
use translate::translate;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(txt) = args.txt {
        let res = translate(args.source_lang, args.target_lang, txt).await;
        println!("{:?}", res);
    }
}
