use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "scanner", setting = structopt::clap::AppSettings::TrailingVarArg)]
pub struct Opts {
    #[structopt(short, long, default_value = "./config/config.yml")]
    pub config: String,

    #[structopt(short, long, use_delimiter = true)]
    pub address: Vec<String>,
}

impl Opts {
    pub fn read() -> Self {
        Opts::from_args()
    }
}

