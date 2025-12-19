mod ast;

use lalrpop_util::lalrpop_mod;
use koopa::ir::{BasicBlock, BinaryOp, Function, FunctionData, Program, Type, Value};
use std::env::args;
use std::fs::read_to_string;
use std::{io, process};
use std::error::Error;
use std::fmt::{self, Display};

lalrpop_mod!(sysy);


// fn main2() -> Result<()> {
//     let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
//     println!("{ast:#?}");
//     Ok(())
// }

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e:#?}");
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn Error>> {
    let CliArgs { mode: _, input, output: _ } = parse_args()?;
    let ast = sysy::CompUnitParser::new().parse(&input)?;
    Ok(())
}

#[derive(Debug)]
enum CompError {
    Cli(String),
    Io(io::Error)
}

impl Error for CompError {}
impl Display for CompError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something")
    }
}

#[derive(Default)]
struct CliArgs {
    mode: String,
    input: String,
    output: String,
}

fn parse_args() -> Result<CliArgs, Box<dyn Error>> {
    let mut args = args();
    args.next();
    let mode = args.next().ok_or(CompError::Cli("mode".to_string()))?;
    let input = args.next().ok_or(CompError::Cli("input".to_string()))?;
    args.next();
    let output = args.next().ok_or(CompError::Cli("output".to_string()))?;
    Ok(CliArgs { mode, input, output })
}

