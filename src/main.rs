pub mod ast;
pub mod compile;

#[macro_use] extern crate lalrpop_util;

use crate::ast::Module;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn main() {
    let result = grammar::BodyParser::new().parse(r#"
    if 4 {
         abcde;
         [1,2,3,4] + 22 * 22 * 23 + 5 ** 3 + "str" - 36.2;
    };
    if 3 {
        abcde;
    } else {
        cdef;
    };
    [1,2,3];
    fun myfun (a,b,c) {
        if 3 {
            print(-"Hello world!");
        };
    };
    fun myfun () {
        if 3 {
            print(+"Hello world!".myattr[1].funky());
        };
    };
    "#);
    println!("{:#?}", result);
    compile::compile(Module{ body: result.unwrap()});
}
