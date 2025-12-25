mod ast;

use koopa::back::{KoopaGenerator, LlvmGenerator};
use koopa::ir::builder::{
    BasicBlockBuilder, BlockBuilder, LocalBuilder, LocalInstBuilder, ValueBuilder,
};
use koopa::ir::{BasicBlock, BinaryOp, Function, FunctionData, Program, Type, Value};
use lalrpop_util::{ParseError, lalrpop_mod, lexer};
use std::env::args;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::{File, read_to_string};
use std::io::{self, Read, Write};
use std::process;

// wtf, it *generates* sysy::CompUnitParser that returns CompUnit.
// lalrpop_mod(anything) reads `anything.lalrpop`.
lalrpop_mod!(sysy);

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e:#?}");
        process::exit(1);
    } else {
        println!("Finished");
    }
}

macro_rules! add_bb {
    ($func:expr, $bb:expr) => {
        $func.layout_mut().bbs_mut().push_key_back($bb).unwrap()
    };
}

macro_rules! add_inst {
    ($func:expr, $bb:expr, $inst:expr) => {
        $func
            .layout_mut()
            .bb_mut($bb)
            .insts_mut()
            .push_key_back($inst)
            .unwrap()
    };
}

fn try_main() -> Result<(), CompErr> {
    let CliArgs {
        mode: _,
        input,
        output,
    } = parse_args()?;
    let code = read_to_string(&input).map_err(CompErr::Io)?;
    let ast = sysy::CompUnitParser::new()
        .parse(&code)
        .map_err(|e| CompErr::ParseError(format!("{:?}", e)))?;
    eprintln!("{:?}", ast);

    let program = ast_to_mem(&ast)?;
    mem_to_ir(&program, File::create(output).map_err(CompErr::Io)?, false)
}

fn ast_to_mem(ast: &ast::CompUnit) -> Result<Program, CompErr> {
    if ast.func_def.ident != "main" {
        return Err(CompErr::NoMain);
    }
    let mut program = Program::new();
    let main_data = FunctionData::new("@main".into(), Vec::new(), Type::get_i32());
    let main_func = program.new_func(main_data);
    let main = program.func_mut(main_func);
    let entry = main.new_bb().basic_block(Some("%entry".into()));
    add_bb!(main, entry);
    let retval = main.new_value().integer(ast.func_def.block.stmt.num);
    let ret = main.new_value().ret(Some(retval));
    add_inst!(main, entry, ret);
    Ok(program)
    // Ok(())
}

#[derive(Debug)]
enum CompErr {
    Cli(String),
    Io(io::Error),
    NoMain,
    ParseError(String),
    Wtf,
}

impl Error for CompErr {}
impl Display for CompErr {
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

fn parse_args() -> Result<CliArgs, CompErr> {
    let mut args = args();
    args.next();
    let mode = args.next().ok_or(CompErr::Cli("mode".to_string()))?;
    let input = args.next().ok_or(CompErr::Cli("input".to_string()))?;
    args.next();
    let output = args.next().ok_or(CompErr::Cli("output".to_string()))?;
    Ok(CliArgs {
        mode,
        input,
        output,
    })
}

fn mem_to_ir<T: Write>(program: &Program, out: T, llvm: bool) -> Result<(), CompErr> {
    if llvm {
        LlvmGenerator::new(out).generate_on(program)
    } else {
        KoopaGenerator::new(out).generate_on(program)
    }
    .map_err(CompErr::Io)
}

struct Env<'a> {
    main: &'a mut FunctionData,
}

// fn build_program<T>(input: T) -> Result<Program, CompErr>
// where
//     T: Read,
// {
//     let mut pro = Program::new();
//     let main = FunctionData::new("@main".into(), vec![], Type::get_i32());
//     let main = pro.new_func(main);
//     gen_main(
//         input,
//         Env {
//             main: pro.func_mut(main),
//         },
//     )?;
//     Ok(pro)
// }

trait Newbb {
    /// block builder
    fn new_bb<'a>(&'a mut self) -> BlockBuilder<'a>;
    fn new_value(&mut self) -> impl LocalInstBuilder;
}
impl Newbb for FunctionData {
    fn new_bb<'a>(&'a mut self) -> BlockBuilder<'a> {
        self.dfg_mut().new_bb()
    }
    fn new_value(&mut self) -> impl LocalInstBuilder {
        self.dfg_mut().new_value()
    }
}

// fn gen_main<T: Read>(input: T, mut env: Env) -> Result<(), CompErr> {
//     let main = &mut env.main;
//     let entry = main.new_bb().basic_block(Some("%entry"))
// }
