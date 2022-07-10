pub mod ast;
// mod lib;
mod interpreter;
mod lib;
// mod interpreter;

#[macro_use]
extern crate lalrpop_util;

use crate::ast::Module;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn main() {
    let result = grammar::BodyParser::new().parse(
        r#"
    import math as matt;
    print("Hello world!");
    "#,
    );
    println!("{:#?}", result);
    interpreter::Runtime::new(Module {
        body: result.unwrap(),
    })
    .eval();
}
