use crate::expr::{Expr, LiteralValue};

pub struct Interpreter {
    // Global data
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue, String> {
        expr.evaluate()
    }
}