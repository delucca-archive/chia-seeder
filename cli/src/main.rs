mod interface;

use structopt::StructOpt;

fn main() {
  let global_opts = interface::opts::Global::from_args();
  println!("{:#?}", global_opts);

  // TODO Draw CLI interface
  // TODO Write CLI interface ADR
  // TODO Start coding CLI interface
}
