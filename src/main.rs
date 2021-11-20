use riddle_compiler::syntax::{ExpCompute, ExpVal};
use riddle_compiler::parser::{ComputationParser, ValueParser};
use riddle_compiler::eval::{eval, eval_with_stack, substCompute, substVal};
use riddle_compiler::syntax::{ExpCompute, ExpVal, Prim2, TCompute, TVal, Terminal};
fn main() {
    
    println!("{:?}", ComputationParser::new().parse("return (0, (true, in1 { return 4 }))"));
}
