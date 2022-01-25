use riddle_compiler::eval::{eval, eval_with_stack, substCompute, substVal};
use riddle_compiler::parser::{ComputationParser, ValueParser};
use riddle_compiler::syntax::{ExpCompute, ExpVal, Prim2, TCompute, TVal, Terminal};
fn main() {
    // let inf = ComputationParser::new()
    //     .parse("let {copm{f.f'!f}} be x.x'!x")
    //     .unwrap();
    // println!("{:?}", eval(&inf));
    // let Y = ComputationParser::new()
    //     .parse("let {copm{f.{copm{x.bind x'!x to t.t'!f}}'!{copm{x.bind x'!x to t.t'!f}}}} be Y.!Y")
    //     .unwrap();

    let fac = ComputationParser::new()
        .parse("
        let {copm{fac.copm{x.bind x == 0 to b.if b then return 1 else bind x - 1 to y.bind y'!fac to z.x * z}}} be fac1.let {copm{f.{copm{x.{x'!x}'!f}}'!{copm{x.{x'!x}'!f}}}} be Y.5'!{fac1'!Y}")
        .unwrap();
    println!("{:?}", fac);
    println!("{:?}", eval_with_stack(&fac));
}

mod eval_test {
    use super::*;
    #[test]
    fn eval_test_identity() {
        let id0 = ComputationParser::new()
            .parse("let { copm{x.return x} } be f.5'!f ")
            .unwrap();
        assert_eq!(
            eval(&id0),
            Ok(Terminal::Return(Box::new(ExpVal::Num(5, ()))))
        );
        assert_eq!(
            eval_with_stack(&id0),
            Ok(Terminal::Return(Box::new(ExpVal::Num(5, ()))))
        );

        let id1 = ComputationParser::new()
            .parse("let { copm{x.return x} } be f.true'!f ")
            .unwrap();
        assert_eq!(
            eval(&id1),
            Ok(Terminal::Return(Box::new(ExpVal::Bool(true, ()))))
        );
        assert_eq!(
            eval_with_stack(&id1),
            Ok(Terminal::Return(Box::new(ExpVal::Bool(true, ()))))
        );
        let id2 = *(ComputationParser::new()
            .parse("let { copm{x.return x} } be f.f'!f ")
            .unwrap());
        match id2.clone() {
            ExpCompute::Let {
                bindings,
                body,
                ann,
            } => {
                assert_eq!(
                    eval(&id2),
                    Ok(Terminal::Return(Box::new(bindings[0].1.clone())))
                );
                assert_eq!(
                    eval_with_stack(&id2),
                    Ok(Terminal::Return(Box::new(bindings[0].1.clone())))
                );
            }
            _ => {
                println!("parse error");
            }
        }
    }
    #[test]
    fn eval_test_if() {
        let if_else = *(ComputationParser::new()
            .parse("bind 1 < 2 to x.if x then return 1 else return 2")
            .unwrap());
        assert_eq!(
            eval(&if_else),
            Ok(Terminal::Return(Box::new(ExpVal::Num(1, ()))))
        );
        assert_eq!(
            eval_with_stack(&if_else),
            Ok(Terminal::Return(Box::new(ExpVal::Num(1, ()))))
        );

        let if_else2 = *(ComputationParser::new()
            .parse("bind 1 > 2 to x.if x then return 1 else return 2")
            .unwrap());
        assert_eq!(
            eval(&if_else2),
            Ok(Terminal::Return(Box::new(ExpVal::Num(2, ()))))
        );
        assert_eq!(
            eval_with_stack(&if_else2),
            Ok(Terminal::Return(Box::new(ExpVal::Num(2, ()))))
        );
    }
    #[test]
    fn eval_test_sum() {
        let sum = ComputationParser::new()
            .parse("pm in1 3 {in1 x.return x | in2 y.return y}")
            .unwrap();
        assert_eq!(
            eval(&sum),
            Ok(Terminal::Return(Box::new(ExpVal::Num(3, ()))))
        );
        assert_eq!(
            eval_with_stack(&sum),
            Ok(Terminal::Return(Box::new(ExpVal::Num(3, ()))))
        );

        let sum2 = ComputationParser::new()
            .parse("pm in2 false {in1 x.return x | in2 y.return y}")
            .unwrap();
        assert_eq!(
            eval(&sum2),
            Ok(Terminal::Return(Box::new(ExpVal::Bool(false, ()))))
        );
        assert_eq!(
            eval_with_stack(&sum2),
            Ok(Terminal::Return(Box::new(ExpVal::Bool(false, ()))))
        );
    }
    #[test]
    fn eval_test_pair() {
        let pair = ComputationParser::new()
            .parse("pm (1, 2) {(x,y).x + y}")
            .unwrap();
        assert_eq!(
            eval(&pair),
            Ok(Terminal::Return(Box::new(ExpVal::Num(3, ()))))
        );
        assert_eq!(
            eval_with_stack(&pair),
            Ok(Terminal::Return(Box::new(ExpVal::Num(3, ()))))
        );
    }

    #[test]
    fn eval_test_proj() {
        let proj = ComputationParser::new()
            .parse("prj1 copm {prj1. 1 + 2 | prj2. 1 - 2}")
            .unwrap();
        assert_eq!(
            eval(&proj),
            Ok(Terminal::Return(Box::new(ExpVal::Num(3, ()))))
        );
        assert_eq!(
            eval_with_stack(&proj),
            Ok(Terminal::Return(Box::new(ExpVal::Num(3, ()))))
        );

        let proj = ComputationParser::new()
            .parse("prj2 copm {prj1. 1 + 2 | prj2. 1 - 2}")
            .unwrap();
        assert_eq!(
            eval(&proj),
            Ok(Terminal::Return(Box::new(ExpVal::Num(-1, ()))))
        );
        assert_eq!(
            eval_with_stack(&proj),
            Ok(Terminal::Return(Box::new(ExpVal::Num(-1, ()))))
        );
    }
}
