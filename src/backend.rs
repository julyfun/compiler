use crate::CompErr;
use koopa::ir::{Value, ValueKind, dfg::DataFlowGraph};
use std::io::Write;

pub trait GenerateAsm<T: Write> {
    fn generate(&self, out: &mut T) -> Result<(), CompErr>;
}

impl<T: Write> GenerateAsm<T> for koopa::ir::Program {
    fn generate(&self, out: &mut T) -> Result<(), CompErr> {
        out.write_all(b"  .text\n  .globl main\n")
            .map_err(CompErr::Io)?;
        for &func in self.func_layout() {
            self.func(func).generate(out)?;
        }
        Ok(())
    }
}

impl<T: Write> GenerateAsm<T> for koopa::ir::FunctionData {
    fn generate(&self, out: &mut T) -> Result<(), CompErr> {
        // remove @ / %
        out.write_all(
            self.name()
                .trim_start_matches(|c| c == '@' || c == '%')
                .as_bytes(),
        )
        .map_err(CompErr::Io)?;
        out.write_all(b":\n").map_err(CompErr::Io)?;
        for (_, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                out.write_all((inst.to_string(self.dfg())? + "\n").as_bytes())
                    .map_err(CompErr::Io)?;
            }
        }
        Ok(())
    }
}

trait InstString {
    fn to_string(&self, dfg: &DataFlowGraph) -> Result<String, CompErr>;
}

impl InstString for Value {
    fn to_string(&self, dfg: &DataFlowGraph) -> Result<String, CompErr> {
        let data = dfg.value(*self);
        match data.kind() {
            ValueKind::Integer(integer) => Ok(format!("{}", integer.value())),
            ValueKind::Return(val) => {
                if let Some(val) = val.value() {
                    Ok(format!("  li a0, {}\n  ret", val.to_string(dfg)?))
                } else {
                    Err(CompErr::Unimplemented("return value is None".to_string()))
                }
            }
            _ => Err(CompErr::Unimplemented(format!(
                "value kind: {:?}",
                data.kind()
            ))),
        }
    }
}

// impl<T: Write> GenerateAsm<T> for koopa::ir::layout::BasicBlockNode {
//     fn generate(&self, out: &mut T) -> Result<(), CompErr> {
//         for &inst in self.insts().keys() {
//             let value_data =
//         }
//         Ok(())
//     }
// }

// impl<T: Write> GenerateAsm<T> for koopa::ir::layout::InstNode {
//     fn generate(&self, out: &mut T) -> Result<(), CompErr> {
//         Ok(())
//     }
// }
