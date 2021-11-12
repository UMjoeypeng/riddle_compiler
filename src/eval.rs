use crate::syntax::{Exp, ExpCompute, ExpVal, Prim2, TCompute, TVal, Terminal, Val};

fn lookup(env: &Vec<(&str, Val)>, x: &str) -> Result<Val, String> {
    for (v, t) in env.iter().rev() {
        if *v == x {
            return Ok((*t).clone());
        }
    }
    Err(format!("{} is not defined", x))
}

pub fn evalVal<'exp>(e: &'exp ExpVal<()>, mut env: &Vec<(&'exp str, Val)>) -> Result<Val, String> {
    match e {
        ExpVal::Num(n, _) => {
            return Ok(Val::Num(*n));
        }
        ExpVal::Bool(b, _) => {
            return Ok(Val::Bool(*b));
        }
        ExpVal::Var(x, _) => {
            return lookup(env, x);
        }
        ExpVal::Thunk(U, _) => {
            return Ok(Val::Thunk((*U).clone()));
        }
        ExpVal::Sum(l, r, _) => {
            return Ok(Val::Sum(
                Box::new(evalVal(l, env)?),
                Box::new(evalVal(r, env)?),
            ));
        }
        ExpVal::Prod(l, r, _) => {
            return Ok(Val::Prod(
                Box::new(evalVal(l, env)?),
                Box::new(evalVal(r, env)?),
            ));
        }
    }
}

pub fn evalCompute<'exp>(
    e: &'exp ExpCompute<()>,
    mut env: &Vec<(&'exp str, Val)>,
) -> Result<Terminal, String> {
    match e {
        ExpCompute::Let {
            bindings,
            body,
            ann,
        } => {
            let mut new_env = env.clone();
            for (v, eval) in bindings.iter() {
                let val = evalVal(eval, &new_env)?;
                new_env.push((v, val));
            }
            evalCompute(body, &new_env);
        }
        ExpCompute::To {
            bindings,
            body,
            ann: _,
        } => {
            let mut new_env = env.clone();
            for (v, c) in bindings.iter() {
                match evalCompute(c, env)? {
                    Terminal::Return(V) => {
                        // let val = evalVal(&*V, &new_env)?;
                        new_env.push((v, *V));
                    }
                    _ => {
                        return Err(format!("Eval To Error"));
                    }
                }
            }
            return evalCompute(body, &new_env);
        }
        ExpCompute::Returner(V, _) => {
            return Ok(Terminal::Return(Box::new(evalVal(V, env)?)));
        }
        ExpCompute::Force(U, _) => match &**U {
            ExpVal::Thunk(M, _) => {
                return evalCompute(&M, env);
            }
            _ => {
                return Err(format!("Eval Force Error"));
            }
        },
        ExpCompute::PmSum {
            subject,
            branch1,
            branch2,
            ann: _,
        } => {
            // TODO
        }
        ExpCompute::PmPair {
            subject,
            left,
            right,
            body,
            ann,
        } => todo!(),
        ExpCompute::CoPm {
            branch1,
            branch2,
            ann,
        } => {
            return Ok(Terminal::CoPm {
                branch1: (*branch1).clone(),
                branch2: (*branch2).clone(),
            });
        }
        ExpCompute::Proj(b, ec, _) => match evalCompute(ec, env)? {
            Terminal::CoPm { branch1, branch2 } => {
                if *b {
                    return evalCompute(&*branch1, env);
                } else {
                    return evalCompute(&*branch2, env);
                }
            }
            _ => {
                return Err(format!("Eval Proj Error"));
            }
        },
        ExpCompute::Pop(x, t, body, _) => return Ok(Terminal::Pop(x.to_string(), (*body).clone())),
        ExpCompute::Push(x, body, _) => {
            let t1 = evalCompute(body, env)?;
            match t1 {
                Terminal::Pop(v, N) => {
                    let mut new_env = env.clone();
                    let val = evalVal(&*x, env)?;
                    new_env.push((&v, val));
                    return evalCompute(&*N, &new_env);
                }
                _ => {
                    return Err(format!("Eval Push Error"));
                }
            }
        }
        ExpCompute::Prim2(op, a, b, _) => {
            let va = evalVal(a, env)?;
            let vb = evalVal(b, env)?;
            match op {
                Prim2::Add | Prim2::Sub | Prim2::Mul => match (va, vb) {
                    (Val::Num(n1), Val::Num(n2)) => match op {
                        Prim2::Add => {
                            return Ok(Terminal::Return(Box::new(Val::Num(n1 + n2))));
                        }
                        Prim2::Sub => {
                            return Ok(Terminal::Return(Box::new(Val::Num(n1 - n2))));
                        }
                        Prim2::Mul => {
                            return Ok(Terminal::Return(Box::new(Val::Num(n1 * n2))));
                        }
                        _ => (),
                    },
                    _ => {
                        return Err(format!("Eval Arithmetic Error"));
                    }
                },

                Prim2::And | Prim2::Or => match (va, vb) {
                    (Val::Bool(n1), Val::Bool(n2)) => {
                        if *op == Prim2::And {
                            return Ok(Terminal::Return(Box::new(Val::Bool(n1 && n2))));
                        } else {
                            return Ok(Terminal::Return(Box::new(Val::Bool(n1 || n2))));
                        }
                    }
                    _ => {
                        return Err(format!("Eval Logical Operation Error"));
                    }
                },

                Prim2::Lt | Prim2::Gt | Prim2::Le | Prim2::Ge => match (va, vb) {
                    (Val::Num(n1), Val::Num(n2)) => match op {
                        Prim2::Lt => {
                            return Ok(Terminal::Return(Box::new(Val::Bool(n1 < n2))));
                        }
                        Prim2::Gt => {
                            return Ok(Terminal::Return(Box::new(Val::Bool(n1 > n2))));
                        }
                        Prim2::Le => {
                            return Ok(Terminal::Return(Box::new(Val::Bool(n1 <= n2))));
                        }
                        Prim2::Ge => {
                            return Ok(Terminal::Return(Box::new(Val::Bool(n1 >= n2))));
                        }
                        _ => ()
                    },
                    _ => {
                        return Err(format!("Eval Arithmetic Error"));
                    }
                },
                Prim2::Eq => match (va, vb){
                    (Val::Num(n1), Val::Num(n2))=>{
                        return Ok(Terminal::Return(Box::new(Val::Bool(n1 == n2))));
                    },
                    (Val::Bool(n1), Val::Bool(n2))=>{
                        return Ok(Terminal::Return(Box::new(Val::Bool(n1 == n2))));
                    },
                    _=>{
                        return Err(format!("Eval == Error"));
                    }
                },
                Prim2::Neq => match (va, vb){
                    (Val::Num(n1), Val::Num(n2))=>{
                        return Ok(Terminal::Return(Box::new(Val::Bool(n1 != n2))));
                    }
                    (Val::Bool(n1), Val::Bool(n2))=>{
                        return Ok(Terminal::Return(Box::new(Val::Bool(n1 != n2))));
                    },
                    (Val::Num(_), Val::Bool(_)) | (Val::Bool(_), Val::Num(_))=>{
                        return Ok(Terminal::Return(Box::new(Val::Bool(false))));
                    },
                    _ => {
                        return Err(format!("Eval != Error"));
                    }
                },
            }
        }
    }
    Err(format!(""))
}
