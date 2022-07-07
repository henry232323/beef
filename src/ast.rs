lalrpop_mod!(pub grammar); // synthesized by LALRPOP

pub enum Expr {
    IntExpr(i64),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

struct IntExpr {
    value: i64
}

