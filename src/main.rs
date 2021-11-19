use riddle_compiler::syntax::{ExpCompute, ExpVal};
use riddle_compiler::parser::{ComputationParser, ValueParser};
fn main() {
    
    println!("{:?}", ComputationParser::new().parse("return (0, (true, in1 { return 4 }))"));
}
