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
        while true {
            print("It works");
        };
    "#,
    );
    println!("{:#?}", result);
    interpreter::Runtime::new(Module {
        body: result.unwrap(),
    })
    .eval();
}
