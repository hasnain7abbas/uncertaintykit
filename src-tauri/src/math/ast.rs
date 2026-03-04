use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    /// Numeric literal: 3.14, 0.5, 100
    Number(f64),

    /// Named variable: x, d, theta
    Variable(String),

    /// Named constant: pi, e
    Constant(String, f64),

    /// Binary operation: left op right
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    /// Unary negation: -expr
    Negate(Box<Expr>),

    /// Function call: sin(expr), ln(expr), sqrt(expr)
    Function {
        name: String,
        arg: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Expr {
    pub fn num(n: f64) -> Self {
        Expr::Number(n)
    }
    pub fn var(name: &str) -> Self {
        Expr::Variable(name.to_string())
    }
    pub fn add(l: Expr, r: Expr) -> Self {
        Expr::BinaryOp {
            op: BinOp::Add,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
    pub fn sub(l: Expr, r: Expr) -> Self {
        Expr::BinaryOp {
            op: BinOp::Sub,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
    pub fn mul(l: Expr, r: Expr) -> Self {
        Expr::BinaryOp {
            op: BinOp::Mul,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
    pub fn div(l: Expr, r: Expr) -> Self {
        Expr::BinaryOp {
            op: BinOp::Div,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
    pub fn pow(l: Expr, r: Expr) -> Self {
        Expr::BinaryOp {
            op: BinOp::Pow,
            left: Box::new(l),
            right: Box::new(r),
        }
    }
    pub fn neg(e: Expr) -> Self {
        Expr::Negate(Box::new(e))
    }
    pub fn func(name: &str, arg: Expr) -> Self {
        Expr::Function {
            name: name.to_string(),
            arg: Box::new(arg),
        }
    }

    /// Check if expression is the constant 0
    pub fn is_zero(&self) -> bool {
        matches!(self, Expr::Number(n) if *n == 0.0)
    }

    /// Check if expression is the constant 1
    pub fn is_one(&self) -> bool {
        matches!(self, Expr::Number(n) if *n == 1.0)
    }

    /// Collect all unique variable names in the expression
    pub fn variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars.sort();
        vars.dedup();
        vars
    }

    fn collect_vars(&self, vars: &mut Vec<String>) {
        match self {
            Expr::Variable(name) => vars.push(name.clone()),
            Expr::BinaryOp { left, right, .. } => {
                left.collect_vars(vars);
                right.collect_vars(vars);
            }
            Expr::Negate(e) | Expr::Function { arg: e, .. } => e.collect_vars(vars),
            _ => {}
        }
    }
}
