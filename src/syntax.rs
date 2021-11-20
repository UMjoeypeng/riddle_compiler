#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpVal<Ann> {
    Num(i64, Ann),                                 // Val
    Bool(bool, Ann),                               // Val
    Var(String, Ann),                              // Val
    Thunk(Box<ExpCompute<Ann>>, Ann),              // Val
    Sum(bool, Box<ExpVal<Ann>>, Ann),  // Val Changed
    Prod(Box<ExpVal<Ann>>, Box<ExpVal<Ann>>, Ann), // Val
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpCompute<Ann> {
    Let {
        bindings: Vec<(String, ExpVal<Ann>)>,
        body: Box<ExpCompute<Ann>>,
        ann: Ann,
    }, //Computation
    To {
        binding: (String, Box<ExpCompute<Ann>>), // Change this to one binding at a time
        body: Box<ExpCompute<Ann>>,
        ann: Ann,
    }, //Computation
    Returner(Box<ExpVal<Ann>>, Ann), // Computation
    Force(Box<ExpVal<Ann>>, Ann),    // Computation
    PmSum {
        subject: Box<ExpVal<Ann>>,
        branch1: (String, Box<ExpCompute<Ann>>),
        branch2: (String, Box<ExpCompute<Ann>>),
        ann: Ann,
    }, // Computation
    PmPair {
        subject: Box<ExpVal<Ann>>,
        left: String,
        right: String,
        body: Box<ExpCompute<Ann>>,
        ann: Ann,
    }, //Computation
    CoPm {
        branch1: Box<ExpCompute<Ann>>,
        branch2: Box<ExpCompute<Ann>>,
        ann: Ann
    }, //Computation
    Proj(bool, Box<ExpCompute<Ann>>, Ann), // Computation
    Pop(String, TVal, Box<ExpCompute<Ann>>, Ann), //Computation
    Push(Box<ExpVal<Ann>>, Box<ExpCompute<Ann>>, Ann), //Computation
    Prim2(Prim2, Box<ExpVal<Ann>>, Box<ExpVal<Ann>>, Ann)
}
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum Exp<Ann> {
//     Num(i64, Ann),    // Val
//     Bool(bool, Ann),  // Val
//     Var(String, Ann), // Val
//     Let {
//         bindings: Vec<(String, ExpVal<Ann>)>,
//         body: Box<Exp<Ann>>,
//         ann: Ann,
//     }, //Computation
//     To {
//         bindings: Vec<(String, ExpCompute<Ann>)>,
//         body: Box<Exp<Ann>>,
//         ann: Ann,
//     }, //Computation
//     Returner(Box<ExpVal<Ann>>, Ann), // Computation
//     Thunk(Box<ExpCompute<Ann>>, Ann), // Val
//     Force(Box<ExpVal<Ann>>, Ann), // Computation
//     Sum(ExpVal<Ann>, ExpVal<Ann>, Ann), // Val
//     Prod(ExpVal<Ann>, ExpVal<Ann>, Ann), // Val
//     PmSum {
//         subject: Box<ExpVal<Ann>>,
//         branch1: (String, ExpCompute<Ann>),
//         branch2: (String, ExpCompute<Ann>),
//         ann: Ann,
//     }, // Computation
//     PmPair {
//         subject: Box<Exp<Ann>>,
//         left: String,
//         right: String,
//         body: Box<Exp<Ann>>,
//         ann: Ann,
//     }, //Computation
//     CoPm {
//         branch1: Box<ExpCompute<Ann>>,
//         branch2: Box<ExpCompute<Ann>>,
//     }, //Computation
//     Proj(bool, Box<ExpCompute<Ann>>), // Computation
//     Pop(String, Box<ExpCompute<Ann>>, Ann), //Computation
//     Push(Box<ExpVal<Ann>>, Box<ExpCompute<Ann>>, Ann), //Computation
// }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exp<Ann> {
    ExpVal(Box<ExpVal<Ann>>), 
    ExpCompute(Box<ExpCompute<Ann>>),
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Terminal {
    Return(Box<ExpVal<()>>),
    Pop(String, Box<ExpCompute<()>>),
    CoPm {
        branch1: Box<ExpCompute<()>>,
        branch2: Box<ExpCompute<()>>,
    },
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Prim2 {
    // TODO:How to define Prim2 into our language
    // Prim2 should be ExpCompute?
    // What is the corresponding type for Prim2
    Add,
    Sub,
    Mul,

    And,
    Or,

    Lt,
    Gt,
    Le,
    Ge,

    Eq,
    Neq,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TVal {
    Unit,
    Num,
    Bool,
    Thunk(Box<TCompute>),
    Prod(Box<TVal>, Box<TVal>),
    Sum(Box<TVal>, Box<TVal>),
    Empty, //
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TCompute {
    Returner(Box<TVal>),
    Pi(Box<TCompute>, Box<TCompute>),
    Arrow(Box<TVal>, Box<TCompute>),
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stack<Ann>{
    Arg(ExpVal<Ann>, Box<Stack<Ann>>, Ann),
    Prj(bool, Box<Stack<Ann>>, Ann),
    cont(String, ExpCompute<Ann>, Box<Stack<Ann>>, Ann), 
    End(Ann),
}

