#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);
pub mod syntax;
mod typeSyn;
pub mod eval;