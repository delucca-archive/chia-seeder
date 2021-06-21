use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
  name = "chia-seeder",
  about = "🌱 Easy to use, blazing fast, and reliable Chia plot manager for your farm"
)]
pub struct Global {
  #[structopt(short, long)]
  debug: bool,
}
