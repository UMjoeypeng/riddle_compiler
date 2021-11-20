use riddle_compiler::eval::{eval, eval_with_stack, substCompute, substVal};
use riddle_compiler::syntax::{ExpCompute, ExpVal, Prim2, TCompute, TVal, Terminal};
fn main() {
    let e1: ExpCompute<()> = ExpCompute::Let {
        bindings: vec![
            (String::from("x"), ExpVal::Num(3, ())),
            (String::from("y"), ExpVal::Var(String::from("z"), ())),
        ],
        body: Box::new(ExpCompute::Prim2(
            Prim2::Add,
            Box::new(ExpVal::Var(String::from("x"), ())),
            Box::new(ExpVal::Var(String::from("y"), ())),
            (),
        )),
        ann: (),
    };
    let e2 = substCompute(e1.clone(), &String::from("z"), ExpVal::Num(3, ()));
    println!("e1:{:?}", e1);
    println!("e2:{:?}", e2);
    println!("v2:{:?}", eval(&e2));
    println!("v2_s:{:?}", eval_with_stack(&e2));
    let e3 = ExpCompute::To {
        binding: (String::from("x"), Box::new(e2.clone())),
        body: Box::new(ExpCompute::Prim2(
            Prim2::Sub,
            Box::new(ExpVal::Var(String::from("x"), ())),
            Box::new(ExpVal::Num(3, ())),
            (),
        )),
        ann: (),
    };
    println!("e3: {:?}", e3);
    println!("v3: {:?}", eval(&e3));
    println!("v3_s: {:?}", eval_with_stack(&e3));
    // let e4 = ExpCompute::To {
    //     bindings: vec![(String::from("x"), e2.clone()),(String::from("y"), e2.clone())],
    //     body: Box::new(ExpCompute::Prim2(
    //         Prim2::Mul,
    //         Box::new(ExpVal::Var(String::from("x"), ())),
    //         Box::new(ExpVal::Var(String::from("y"), ())),
    //         (),
    //     )),
    //     ann: (),
    // };
    // println!("e3: {:?}", e3);
    // println!("v3: {:?}", eval(&e3));
    // println!("v3_s: {:?}", eval_with_stack(&e3));
    // println!("e4: {:?}", e4);
    // println!("v4: {:?}", eval(&e4));
    // println!("v4_s: {:?}", eval_with_stack(&e4));
}
