use crate::syntax::{Exp, ExpCompute, ExpVal, Prim2, TCompute, TVal, Terminal};

pub fn lookup(gamma: &Vec<(&str, TVal)>, x: &str) -> Result<TVal, String> {
    for (v, t) in gamma.iter().rev() {
        if *v == x {
            return Ok((*t).clone());
        }
    }
    Err(format!("{} is not defined", x))
}

pub fn typeSynVal<'exp>(
    e: &'exp ExpVal<()>,
    mut gamma: &Vec<(&'exp str, TVal)>,
) -> Result<TVal, String> {
    match e {
        ExpVal::Num(n, _) => Ok(TVal::Num),
        ExpVal::Bool(b, _) => Ok(TVal::Bool),
        ExpVal::Var(v, _) => lookup(gamma, &v),
        ExpVal::Thunk(t, _) => match typeSynCompute(t, gamma) {
            Ok(tc) => Ok(TVal::Thunk(Box::new(tc))),
            Err(s) => Err(s),
        },
        // ExpVal::Sum(l, r, _) => match (typeSynVal(l, gamma), typeSynVal(r, gamma)) {
        //     (Ok(tvl), Ok(tvr)) => Ok(TVal::Sum(Box::new(tvl), Box::new(tvr))),
        //     _ => Err(format!("Sum Type Error")),
        // },
        ExpVal::Prod(l, r, _) => match (typeSynVal(l, gamma), typeSynVal(r, gamma)) {
            (Ok(tvl), Ok(tvr)) => Ok(TVal::Prod(Box::new(tvl), Box::new(tvr))),
            _ => Err(format!("Sum Type Error")),
        },
        ExpVal::Sum(_, _, _) => todo!(),
        
    }
}

pub fn typeSynCompute<'exp>(
    e: &'exp ExpCompute<()>,
    mut gamma: &Vec<(&'exp str, TVal)>,
) -> Result<TCompute, String> {
    match e {
        ExpCompute::Let {
            bindings,
            body,
            ann,
        } => {
            let mut new_gamma = gamma.clone();
            for (x, v) in bindings.iter() {
                let tv = typeSynVal(v, &new_gamma)?;
                new_gamma.push((x, tv));
            }
            return typeSynCompute(body, &new_gamma);
        }
        ExpCompute::To {
            binding,
            body,
            ann,
        } => {
            todo!();
            // let mut new_gamma = gamma.clone();
            // for (x, v) in bindings.iter() {
            //     match typeSynCompute(v, &new_gamma)? {
            //         TCompute::Returner(tv) => {
            //             new_gamma.push((x, *tv));
            //         }
            //         _ => {
            //             return Err(format!("To Type Error!"));
            //         }
            //     }
            // }
            // return typeSynCompute(body, &new_gamma);
        }
        ExpCompute::Returner(ev, _) => {
            let tv = typeSynVal(ev, gamma)?;
            return Ok(TCompute::Returner(Box::new(tv)));
        }
        ExpCompute::Force(v, _) => match typeSynVal(v, gamma)? {
            TVal::Thunk(t) => {
                return Ok(*t);
            }
            _ => {
                return Err(format!("Force Type Error!"));
            }
        },
        ExpCompute::PmSum {
            subject,
            branch1,
            branch2,
            ann,
        } => match typeSynVal(subject, gamma)? {
            TVal::Sum(l, r) => {
                let mut gamma_l = gamma.clone();
                gamma_l.push((&branch1.0, *l));
                let mut gamma_r = gamma.clone();
                gamma_r.push((&branch2.0, *r));
                match (
                    typeSynCompute(&branch1.1, &gamma_l),
                    typeSynCompute(&branch2.1, &gamma_r),
                ) {
                    (Ok(tcl), Ok(tcr)) => {
                        if tcl == tcr {
                            return Ok(tcl);
                        } else {
                            return Err(format!("PmSum Type Error"));
                        }
                    }
                    _ => {
                        return Err(format!("PmSum Type Error"));
                    }
                }
            }
            _ => {
                return Err(format!("PmSum Type Error"));
            }
        },
        ExpCompute::PmPair {
            subject,
            left,
            right,
            body,
            ann,
        } => match typeSynVal(subject, gamma)? {
            TVal::Prod(tvl, tvr) => {
                let mut new_gamma = gamma.clone();
                new_gamma.push((left, *tvl));
                new_gamma.push((right, *tvr));
                return typeSynCompute(body, &new_gamma);
            }
            _ => {
                return Err(format!("PmPair Type Error"));
            }
        },
        ExpCompute::CoPm {
            branch1,
            branch2,
            ann,
        } => {
            match (
                typeSynCompute(branch1, gamma),
                typeSynCompute(branch2, gamma),
            ) {
                (Ok(tcl), Ok(tcr)) => {
                    return Ok(TCompute::Pi(Box::new(tcl), Box::new(tcr)));
                }
                _ => {
                    return Err(format!("CoPm Type Error"));
                }
            }
        }
        ExpCompute::Proj(b, e, _) => match typeSynCompute(e, gamma)? {
            TCompute::Pi(tcl, tcr) => {
                if *b {
                    return Ok(*tcl);
                } else {
                    return Ok(*tcr);
                }
            }
            _ => {
                return Err(format!("Proj Type Error"));
            }
        },
        ExpCompute::Pop(x, tv, e, _) => {
            // Need declare input type explicitly
            let mut new_gamma = gamma.clone();
            new_gamma.push((x, (*tv).clone()));
            return typeSynCompute(e, &new_gamma);
        }
        ExpCompute::Push(ev, ec, _) => {
            let tv = typeSynVal(ev, gamma)?;
            match typeSynCompute(ec, gamma)? {
                TCompute::Arrow(tin, tout) => {
                    if *tin == tv {
                        return Ok(*tout);
                    } else {
                        return Err(format!("Push Type Error"));
                    }
                }
                _ => {
                    return Err(format!("Push Type Error"));
                }
            }
        }
        ExpCompute::Prim2(op, a, b, _) => match op {
            Prim2::Add | Prim2::Sub | Prim2::Mul => {
                match (typeSynVal(a, gamma)?, typeSynVal(b, gamma)?) {
                    (TVal::Num, TVal::Num) => {
                        return Ok(TCompute::Returner(Box::new(TVal::Num)));
                    }
                    _ => {
                        return Err(format!("Prim2 Arithmetic Type Error"));
                    }
                }
            }

            Prim2::And | Prim2::Or => match (typeSynVal(a, gamma)?, typeSynVal(b, gamma)?) {
                (TVal::Bool, TVal::Bool) => {
                    return Ok(TCompute::Returner(Box::new(TVal::Bool)));
                }
                _ => {
                    return Err(format!("Prim2 and/or Type Error"));
                }
            },

            Prim2::Lt | Prim2::Gt | Prim2::Le | Prim2::Ge => {
                match (typeSynVal(a, gamma)?, typeSynVal(b, gamma)?) {
                    (TVal::Num, TVal::Num) => {
                        return Ok(TCompute::Returner(Box::new(TVal::Bool)));
                    }
                    _ => {
                        return Err(format!("Prim2 Compare Type Error"));
                    }
                }
            }
            //TODO: eq and neq only works for int
            Prim2::Eq => match (typeSynVal(a, gamma)?, typeSynVal(b, gamma)?) {
                (TVal::Num, TVal::Num) | (TVal::Bool, TVal::Bool) => {
                    return Ok(TCompute::Returner(Box::new(TVal::Bool)))
                }
                _ => {
                    return Err(format!("Prim2 == Type Error"));
                }
            },
            Prim2::Neq => match (typeSynVal(a, gamma), typeSynVal(b, gamma)) {
                (Ok(tvl), Ok(tvr)) => {
                    if (tvl == TVal::Bool || tvl == TVal::Num)
                        && (tvr == TVal::Bool || tvr == TVal::Num)
                    {
                        return Ok(TCompute::Returner(Box::new(TVal::Bool)));
                    } else {
                        return Err(format!("Prim2 Neq Error"));
                    }
                }
                _ => {
                    return Err(format!("Prim2 Neq Error"));
                }
            },
        },
    }
}
