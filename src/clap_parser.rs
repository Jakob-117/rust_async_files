use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(short, long)]
    pub(crate) n: i64, //wirte n files

    #[clap(short, long)]
    pub(crate) m: i64, //load m files
}
