use crate::syntax::{Exp, ExpCompute, ExpVal, Prim2, Stack, TCompute, TVal, Terminal};

pub fn substVal(e: ExpVal<()>, x: &str, v: ExpVal<()>) -> ExpVal<()> {
    match e {
        ExpVal::Num(n, _) => {
            return ExpVal::Num(n, ());
        }
        ExpVal::Bool(b, _) => {
            return ExpVal::Bool(b, ());
        }
        ExpVal::Var(y, _) => {
            if x == y {
                return v;
            } else {
                return ExpVal::Var(y, ());
            }
        }
        ExpVal::Thunk(u, _) => {
            return ExpVal::Thunk(Box::new(substCompute(*u, x, v.clone())), ());
        }
        ExpVal::Sum(d, e, _) => {
            return ExpVal::Sum(d, Box::new(substVal(*e, x, v.clone())), ());
        }
        ExpVal::Prod(e1, e2, _) => {
            return ExpVal::Prod(
                Box::new(substVal(*e1, x, v.clone())),
                Box::new(substVal(*e2, x, v.clone())),
                (),
            );
        }
    }
}

pub fn substCompute(e: ExpCompute<()>, x: &str, v: ExpVal<()>) -> ExpCompute<()> {
    match e {
        ExpCompute::Let {
            bindings,
            body,
            ann,
        } => {
            let mut new_bindings = Vec::new();
            for (y, val) in bindings.iter() {
                if x == y {
                    new_bindings.push((y.to_string(), val.clone()));
                } else {
                    new_bindings.push((y.to_string(), substVal(val.clone(), x.clone(), v.clone())));
                }
            }
            return ExpCompute::Let {
                bindings: new_bindings,
                body: Box::new(substCompute(*body, x.clone(), v.clone())),
                ann: (),
            };
        }
        ExpCompute::To { binding, body, ann } => {
            let (y, val) = binding;
            if x == y {
                return ExpCompute::To {
                    binding: (y, val),
                    body: Box::new(substCompute(*body, x.clone(), v.clone())),
                    ann: (),
                };
            } else {
                return ExpCompute::To {
                    binding: (
                        y,
                        Box::new(substCompute(*val.clone(), x.clone(), v.clone())),
                    ),
                    body: Box::new(substCompute(*body, x.clone(), v.clone())),
                    ann: (),
                };
            }
            // let mut new_bindings = Vec::new();
            // for (y, val) in bindings.iter() {
            //     if x == y {
            //         new_bindings.push((y.to_string(), val.clone()));
            //     } else {
            //         new_bindings.push((
            //             y.to_string(),
            //             substCompute(val.clone(), x.clone(), v.clone()),
            //         ));
            //     }
            // }
            // return ExpCompute::To {
            //     bindings: new_bindings,
            //     body: Box::new(substCompute(*body, x.clone(), v.clone())),
            //     ann: (),
            // };
        }
        ExpCompute::Returner(e, _) => {
            return ExpCompute::Returner(Box::new(substVal(*e, x, v)), ());
        }
        ExpCompute::Force(e, _) => {
            return ExpCompute::Force(Box::new(substVal(*e, x, v)), ());
        }
        ExpCompute::PmSum {
            subject,
            branch1,
            branch2,
            ann,
        } => {
            let (l, e1) = branch1;
            let (r, e2) = branch2;
            if l != x && r != x {
                return ExpCompute::PmSum {
                    subject: Box::new(substVal(*subject, x.clone(), v.clone())),
                    branch1: (l, Box::new(substCompute(*e1, x.clone(), v.clone()))),
                    branch2: (r, Box::new(substCompute(*e2, x.clone(), v.clone()))),
                    ann: (),
                };
            } else if l != x && r == x {
                return ExpCompute::PmSum {
                    subject: Box::new(substVal(*subject, x.clone(), v.clone())),
                    branch1: (l, Box::new(substCompute(*e1, x.clone(), v.clone()))),
                    branch2: (r, e2),
                    ann: (),
                };
            } else if l == x && r != x {
                return ExpCompute::PmSum {
                    subject: Box::new(substVal(*subject, x.clone(), v.clone())),
                    branch1: (l, e1),
                    branch2: (r, Box::new(substCompute(*e2, x.clone(), v.clone()))),
                    ann: (),
                };
            } else {
                return ExpCompute::PmSum {
                    subject: Box::new(substVal(*subject, x.clone(), v.clone())),
                    branch1: (l, e1),
                    branch2: (r, e2),
                    ann: (),
                };
            }
        }
        ExpCompute::PmPair {
            subject,
            left,
            right,
            body,
            ann,
        } => {
            if x == left || x == right {
                return ExpCompute::PmPair {
                    subject: Box::new(substVal(*subject, x.clone(), v.clone())),
                    left: left,
                    right: right,
                    body: body,
                    ann: (),
                };
            } else {
                return ExpCompute::PmPair {
                    subject: Box::new(substVal(*subject, x.clone(), v.clone())),
                    left: left,
                    right: right,
                    body: Box::new(substCompute(*body, x.clone(), v.clone())),
                    ann: (),
                };
            }
        }
        ExpCompute::CoPm {
            branch1,
            branch2,
            ann,
        } => {
            return ExpCompute::CoPm {
                branch1: Box::new(substCompute(*branch1, x.clone(), v.clone())),
                branch2: Box::new(substCompute(*branch2, x.clone(), v.clone())),
                ann: (),
            }
        }
        ExpCompute::Proj(d, e, _) => {
            return ExpCompute::Proj(d, Box::new(substCompute(*e, x.clone(), v.clone())), ());
        }
        ExpCompute::Pop(y, t, e, _) => {
            if y == x {
                return ExpCompute::Pop(y, t, e, ());
            } else {
                return ExpCompute::Pop(y, t, Box::new(substCompute(*e, x.clone(), v.clone())), ());
            }
        }
        ExpCompute::Push(e1, e2, _) => {
            return ExpCompute::Push(
                Box::new(substVal(*e1, x.clone(), v.clone())),
                Box::new(substCompute(*e2, x.clone(), v.clone())),
                (),
            );
        }
        ExpCompute::Prim2(op, e1, e2, _) => {
            return ExpCompute::Prim2(
                op,
                Box::new(substVal(*e1, x.clone(), v.clone())),
                Box::new(substVal(*e2, x.clone(), v.clone())),
                (),
            );
        }
    }
}

#[cfg(test)]
mod subst_tests {
    use super::*;
    #[test]
    fn subst_test0() {
        // test_let
        let e1 = ExpCompute::Let {
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
        let e3 = ExpCompute::Let {
            bindings: vec![
                (String::from("x"), ExpVal::Num(3, ())),
                (String::from("y"), ExpVal::Num(3, ())),
            ],
            body: Box::new(ExpCompute::Prim2(
                Prim2::Add,
                Box::new(ExpVal::Var(String::from("x"), ())),
                Box::new(ExpVal::Var(String::from("y"), ())),
                (),
            )),
            ann: (),
        };
        assert_eq!(e2, e3);
    }
}

pub fn eval(e: &ExpCompute<()>) -> Result<Terminal, String> {
    match e {
        ExpCompute::Let {
            bindings,
            body,
            ann,
        } => {
            let mut new_body = *body.clone();
            for (x, val) in bindings.iter() {
                new_body = substCompute(new_body, x, val.clone());
            }
            return eval(&new_body);
        }
        ExpCompute::To { binding, body, ann } => {
            let mut new_body = *body.clone();
            let (x, e) = binding;
            let t = eval(&*e)?;
            match t {
                Terminal::Return(v) => {
                    new_body = substCompute(new_body, x, *v);
                }
                _ => {
                    return Err(format!("Error: Eval To error"));
                }
            }
            return eval(&new_body);
        }
        ExpCompute::Returner(v, _) => {
            return Ok(Terminal::Return(v.clone()));
        }
        ExpCompute::Force(e, _) => match &**e {
            ExpVal::Thunk(u, _) => {
                return eval(&u);
            }
            _ => {
                return Err(format!("Error: Eval Force error"));
            }
        },
        ExpCompute::PmSum {
            subject,
            branch1,
            branch2,
            ann,
        } => match &**subject {
            ExpVal::Sum(d, e, _) => {
                if *d {
                    let (x, body) = branch1.clone();
                    let new_body = substCompute(*body, &x, *e.clone());
                    return eval(&new_body);
                } else {
                    let (x, body) = branch2.clone();
                    let new_body = substCompute(*body, &x, *e.clone());
                    return eval(&new_body);
                }
            }
            _ => {
                return Err(format!("Error: Eval PmSum error"));
            }
        },
        ExpCompute::PmPair {
            subject,
            left,
            right,
            body,
            ann,
        } => match &**subject {
            ExpVal::Prod(e1, e2, _) => {
                let mut new_body = *body.clone();
                new_body = substCompute(new_body, left, *e1.clone());
                new_body = substCompute(new_body, right, *e2.clone());
                return eval(&new_body);
            }
            _ => {
                return Err(format!("Error: Eval PmPair error"));
            }
        },
        ExpCompute::CoPm {
            branch1,
            branch2,
            ann,
        } => {
            return Ok(Terminal::CoPm {
                branch1: branch1.clone(),
                branch2: branch2.clone(),
            });
        }
        ExpCompute::Proj(d, e, _) => match &**e {
            ExpCompute::CoPm {
                branch1,
                branch2,
                ann,
            } => {
                if *d {
                    return eval(branch1);
                } else {
                    return eval(branch2);
                }
            }
            _ => {
                return Err(format!("Error: Eval Proj error"));
            }
        },
        ExpCompute::Pop(x, t, e, _) => {
            return Ok(Terminal::Pop(x.to_string(), e.clone()));
        }
        ExpCompute::Push(v, e, _) => {
            let t = eval(e)?;
            match t {
                Terminal::Pop(x, e) => {
                    return eval(&substCompute(*e, &x, *v.clone()));
                }
                _ => {
                    return Err(format!("Error: Eval Push error"));
                }
            }
        }
        ExpCompute::Prim2(op, e1, e2, _) => match op {
            Prim2::Add
            | Prim2::Sub
            | Prim2::Mul
            | Prim2::Lt
            | Prim2::Gt
            | Prim2::Le
            | Prim2::Ge
            | Prim2::Eq
            | Prim2::Neq => match (&**e1, &**e2) {
                (ExpVal::Num(n1, _), ExpVal::Num(n2, _)) => match op {
                    Prim2::Add => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Num(n1 + n2, ()))));
                    }
                    Prim2::Sub => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Num(n1 - n2, ()))));
                    }
                    Prim2::Mul => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Num(n1 * n2, ()))));
                    }
                    Prim2::Lt => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(n1 < n2, ()))));
                    }
                    Prim2::Gt => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(n1 > n2, ()))));
                    }
                    Prim2::Le => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(n1 <= n2, ()))));
                    }
                    Prim2::Ge => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(n1 >= n2, ()))));
                    }
                    Prim2::Eq => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(n1 == n2, ()))));
                    }
                    Prim2::Neq => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(n1 != n2, ()))));
                    }
                    _ => {
                        return Err(format!("Error: Impossible"));
                    }
                },
                _ => {
                    return Err(format!("Error: Eval arithmetic error"));
                }
            },
            Prim2::And | Prim2::Or => match (&**e1, &**e2) {
                (ExpVal::Bool(b1, _), ExpVal::Bool(b2, _)) => match op {
                    Prim2::And => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(*b1 && *b2, ()))));
                    }
                    Prim2::Or => {
                        return Ok(Terminal::Return(Box::new(ExpVal::Bool(*b1 || *b2, ()))));
                    }
                    _ => {
                        return Err(format!("Error: Impossible"));
                    }
                },
                _ => {
                    return Err(format!("Error: Eval Boolean Algebra error"));
                }
            },
        },
    }
}

pub fn eval_with_stack(e: &ExpCompute<()>) -> Result<Terminal, String> {
    let mut compute = e.clone();
    let mut stack = Stack::End(());
    loop {
        // println!("compute: {:?}",&compute);
        match compute.clone() {
            ExpCompute::Let {
                bindings,
                body,
                ann,
            } => {
                compute = *body;
                for (x, val) in bindings.iter() {
                    compute = substCompute(compute, x, val.clone());
                }
            }
            ExpCompute::To {
                binding,
                body,
                ann,
            } => {
                
                let (x, e) = binding;
                stack = Stack::cont(x.to_string(), *body.clone(), Box::new(stack), ());
                compute = *e.clone();
                // // if bindings.len() == 1 {
                // //     let (x, e) = bindings[0].clone();
                // //     println!("e: {:?}",e);
                // //     stack = Stack::cont(x.to_string(), *body.clone(), Box::new(stack), ());
                // //     compute = e.clone();
                // // } else {

                // // }
                // compute = *body;
                // for (x, e) in bindings.iter() {
                //     stack = Stack::cont(x.to_string(), compute.clone(), Box::new(stack), ());
                //     compute = e.clone();
                // }
            }
            ExpCompute::Returner(v, _) => match stack {
                Stack::cont(ref x, ref e, ref s, _) => {
                    compute = substCompute(e.clone(), &x, *v.clone());
                    stack = *s.clone();
                }
                Stack::End(_) => {
                    return Ok(Terminal::Return(v));
                }
                _ => {
                    return Err(format!("Error: Stack Error while eval return"));
                }
            },
            ExpCompute::Force(e, _) => match *e {
                ExpVal::Thunk(u, _) => {
                    compute = *u.clone();
                }
                _ => {
                    return Err(format!("Error: Eval Force error"));
                }
            },
            ExpCompute::PmSum {
                subject,
                branch1,
                branch2,
                ann,
            } => match *subject {
                ExpVal::Sum(d, v, _) => {
                    let (x, body) = branch1.clone();
                    if d {
                        compute = substCompute(*body, &x, *v.clone());
                    } else {
                        let (x, body) = branch2.clone();
                        compute = substCompute(*body, &x, *v.clone());
                    }
                }
                _ => {
                    return Err(format!("Error: Eval PmSum error"));
                }
            },
            ExpCompute::PmPair {
                subject,
                left,
                right,
                body,
                ann,
            } => match *subject {
                ExpVal::Prod(e1, e2, _) => {
                    compute = substCompute(*body, &left, *e1.clone());
                    compute = substCompute(compute, &right, *e2.clone());
                }
                _ => {
                    return Err(format!("Error: Eval PmPair error"));
                }
            },
            ExpCompute::CoPm {
                branch1,
                branch2,
                ann,
            } => match stack {
                Stack::Prj(d, ref s, _) => {
                    if d {
                        compute = *branch1;
                    } else {
                        compute = *branch2;
                    }
                    stack = *s.clone();
                }
                Stack::End(_) => {
                    return Ok(Terminal::CoPm {
                        branch1: branch1,
                        branch2: branch2,
                    });
                }
                _ => {
                    return Err(format!("Error: Stack Error while eval CoPm"));
                }
            },
            ExpCompute::Proj(d, e, _) => {
                compute = *e.clone();
                stack = Stack::Prj(d, Box::new(stack), ());
            }
            ExpCompute::Pop(x, t, e, _) => match stack {
                Stack::Arg(ref v, ref s, _) => {
                    compute = substCompute(*e, &x, v.clone());
                    stack = *s.clone();
                }
                Stack::End(_) => {
                    return Ok(Terminal::Pop(x, e));
                }
                _ => {
                    return Err(format!("Error: Stack Error while eval Pop"));
                }
            },
            ExpCompute::Push(v, e, _) => {
                compute = *e;
                stack = Stack::Arg(*v, Box::new(stack), ());
            }
            ExpCompute::Prim2(op, e1, e2, _) => match op {
                Prim2::Add
                | Prim2::Sub
                | Prim2::Mul
                | Prim2::Lt
                | Prim2::Gt
                | Prim2::Le
                | Prim2::Ge
                | Prim2::Eq
                | Prim2::Neq => match (*e1, *e2) {
                    (ExpVal::Num(n1, _), ExpVal::Num(n2, _)) => match op {
                        Prim2::Add => {
                            compute = ExpCompute::Returner(Box::new(ExpVal::Num(n1 + n2, ())), ());
                        }
                        Prim2::Sub => {
                            compute = ExpCompute::Returner(Box::new(ExpVal::Num(n1 - n2, ())), ());
                        }
                        Prim2::Mul => {
                            compute = ExpCompute::Returner(Box::new(ExpVal::Num(n1 * n2, ())), ());
                        }
                        Prim2::Lt => {
                            compute = ExpCompute::Returner(Box::new(ExpVal::Bool(n1 < n2, ())), ());
                        }
                        Prim2::Gt => {
                            compute = ExpCompute::Returner(Box::new(ExpVal::Bool(n1 > n2, ())), ());
                        }
                        Prim2::Le => {
                            compute =
                                ExpCompute::Returner(Box::new(ExpVal::Bool(n1 <= n2, ())), ());
                        }
                        Prim2::Ge => {
                            compute =
                                ExpCompute::Returner(Box::new(ExpVal::Bool(n1 >= n2, ())), ());
                        }
                        Prim2::Eq => {
                            compute =
                                ExpCompute::Returner(Box::new(ExpVal::Bool(n1 == n2, ())), ());
                        }
                        Prim2::Neq => {
                            compute =
                                ExpCompute::Returner(Box::new(ExpVal::Bool(n1 != n2, ())), ());
                        }
                        _ => {
                            return Err(format!("Error: Impossible"));
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Error: Eval arithmetic error, compute:{:?}",
                            compute
                        ));
                    }
                },
                Prim2::And | Prim2::Or => match (*e1, *e2) {
                    (ExpVal::Bool(b1, _), ExpVal::Bool(b2, _)) => match op {
                        Prim2::And => {
                            compute =
                                ExpCompute::Returner(Box::new(ExpVal::Bool(b1 && b2, ())), ());
                        }
                        Prim2::Or => {
                            compute =
                                ExpCompute::Returner(Box::new(ExpVal::Bool(b1 || b2, ())), ());
                        }
                        _ => {
                            return Err(format!("Error: Impossible"));
                        }
                    },
                    _ => {
                        return Err(format!("Error: Eval Boolean Algebra error"));
                    }
                },
            },
        }

        // let t = eval(&compute)?;
        // match t {
        //     Terminal::Return(v) => match stack {
        //         Stack::cont(ref x, ref e, ref s, _) => {
        //             compute = substCompute(e.clone(), &x, *v.clone());
        //             stack = *s.clone();
        //         }
        //         Stack::End(_) => {
        //             return Ok(Terminal::Return(v));
        //         }
        //         _ => {
        //             return Err(format!("Error: Stack Error while eval return"));
        //         }
        //     },
        //     Terminal::Pop(x, e) => match stack {
        //         Stack::Arg(ref v, ref s, _) => {
        //             compute = substCompute(*e.clone(), &x, v.clone());
        //             stack = *s.clone();
        //         }
        //         Stack::End(_) => {
        //             return Ok(Terminal::Pop(x, e));
        //         }
        //         _ => return Err(format!("Error: Stack Error while eval Pop")),
        //     },
        //     Terminal::CoPm { branch1, branch2 } => match stack {
        //         Stack::Prj(d, ref s, _) => {
        //             if d {
        //                 compute = *branch1;
        //                 stack = *s.clone();
        //             } else {
        //                 compute = *branch2;
        //                 stack = *s.clone();
        //             }
        //         }
        //         Stack::End(_) => {
        //             return Ok(Terminal::CoPm{
        //                 branch1: branch1,
        //                 branch2: branch2,
        //             })
        //         },
        //         _ => {
        //             return Err(format!("Error: Stack Error while eval CoPm"));
        //         }
        //     },
        // }
    }
    todo!();
}



#[cfg(test)]
mod eval_tests {
    use super::*;
    #[test]
    fn eval_test0(){
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
        assert_eq!(eval(&e2), Ok(Terminal::Return(Box::new(ExpVal::Num(6,())))));
        assert_eq!(eval_with_stack(&e2), Ok(Terminal::Return(Box::new(ExpVal::Num(6,())))));
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
        assert_eq!(eval(&e3), Ok(Terminal::Return(Box::new(ExpVal::Num(3,())))));
        assert_eq!(eval_with_stack(&e3), Ok(Terminal::Return(Box::new(ExpVal::Num(3,())))));
    }
}