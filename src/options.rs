use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
  name = crate_name!(),
  version = crate_version!(),
  about = crate_description!(),
  author = crate_authors!()
)]

pub struct Options {
    #[structopt(
        short = "c",
        long = "config",
        required = true,
        help = "config file path",
        default_value = "config.toml"
    )]
    pub config: String,
}
