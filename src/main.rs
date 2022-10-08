mod arg_parsing;

use crate::arg_parsing::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    args.execute_command();
}
