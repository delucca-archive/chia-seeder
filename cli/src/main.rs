mod interface;

use structopt::StructOpt;

fn main() {
  let global_opts = interface::opts::Global::from_args();
  println!("{:#?}", global_opts);
}
