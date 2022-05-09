use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    pub keyboard: String,

    #[structopt(long)]
    pub sampler: Option<PathBuf>,

    #[structopt(long)]
    pub wave: Option<String>,
}

pub fn args() -> Opt {
    Opt::from_args()
}
