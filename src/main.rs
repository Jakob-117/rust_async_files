use async_std::{self, fs::OpenOptions};
use clap::Parser;
use clap_parser::Args;
use tokio::{
    self,
    fs::{self, File},
};

mod clap_parser;

#[tokio::main]
async fn main() {
    let cli = Args::parse();
    parser(cli).await;
}

async fn parser(cli: Args) {
    if cli.n >= 1 {
        loop {}
    } else {
        println!("something fucky");
    }
}

async fn create_file() -> async_std::fs::File {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("file.data")
        .await
        .unwrap_or_else(|err| {
            eprintln!("Something went wrong: {err:?}");
            std::process::exit(1)
        });

    file
}

async fn open_file(count: usize) {
    let contents = fs::read_to_string("file.data").await.unwrap_or_else(|err| {
        eprintln!("Something went wrong: {err:?}");
        std::process::exit(1)
    });
    println!("file content: {}", contents);
}
