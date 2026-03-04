use crate::math::ast::{BinOp, Expr};

/// Recursively simplify an expression tree.
/// Apply repeatedly until no more changes occur.
pub fn simplify(expr: &Expr) -> Expr {
    let simplified = simplify_once(expr);
    let re_simplified = simplify_once(&simplified);
    if format!("{:?}", simplified) == format!("{:?}", re_simplified) {
        simplified
    } else {
        simplify(&re_simplified)
    }
}

fn simplify_once(expr: &Expr) -> Expr {
    match expr {
        Expr::Number(_) | Expr::Variable(_) | Expr::Constant(_, _) => expr.clone(),

        Expr::Negate(e) => {
            let e = simplify_once(e);
            match &e {
                Expr::Number(n) => Expr::num(-n),
                Expr::Negate(inner) => *inner.clone(),
                _ => Expr::neg(e),
            }
        }

        Expr::BinaryOp { op, left, right } => {
            let l = simplify_once(left);
            let r = simplify_once(right);

            // Constant folding
            if let (Expr::Number(a), Expr::Number(b)) = (&l, &r) {
                let result = match op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Mul => a * b,
                    BinOp::Div if *b != 0.0 => a / b,
                    BinOp::Pow => a.powf(*b),
                    _ => {
                        return Expr::BinaryOp {
                            op: *op,
                            left: Box::new(l),
                            right: Box::new(r),
                        }
                    }
                };
                return Expr::num(result);
            }

            match op {
                BinOp::Add => {
                    if l.is_zero() {
                        return r;
                    }
                    if r.is_zero() {
                        return l;
                    }
                    Expr::add(l, r)
                }
                BinOp::Sub => {
                    if r.is_zero() {
                        return l;
                    }
                    if l.is_zero() {
                        return Expr::neg(r);
                    }
                    if format!("{:?}", l) == format!("{:?}", r) {
                        return Expr::num(0.0);
                    }
                    Expr::sub(l, r)
                }
                BinOp::Mul => {
                    if l.is_zero() || r.is_zero() {
                        return Expr::num(0.0);
                    }
                    if l.is_one() {
                        return r;
                    }
                    if r.is_one() {
                        return l;
                    }
                    if matches!(&l, Expr::Number(n) if *n == -1.0) {
                        return Expr::neg(r);
                    }
                    if matches!(&r, Expr::Number(n) if *n == -1.0) {
                        return Expr::neg(l);
                    }
                    Expr::mul(l, r)
                }
                BinOp::Div => {
                    if r.is_one() {
                        return l;
                    }
                    if l.is_zero() {
                        return Expr::num(0.0);
                    }
                    if format!("{:?}", l) == format!("{:?}", r) {
                        return Expr::num(1.0);
                    }
                    Expr::div(l, r)
                }
                BinOp::Pow => {
                    if r.is_zero() {
                        return Expr::num(1.0);
                    }
                    if r.is_one() {
                        return l;
                    }
                    if l.is_zero() {
                        return Expr::num(0.0);
                    }
                    if l.is_one() {
                        return Expr::num(1.0);
                    }
                    Expr::pow(l, r)
                }
            }
        }

        Expr::Function { name, arg } => {
            let simplified_arg = simplify_once(arg);

            if let Expr::Number(n) = &simplified_arg {
                let result = match name.as_str() {
                    "sin" => Some(n.sin()),
                    "cos" => Some(n.cos()),
                    "tan" => Some(n.tan()),
                    "exp" => Some(n.exp()),
                    "ln" | "log" if *n > 0.0 => Some(n.ln()),
                    "log10" if *n > 0.0 => Some(n.log10()),
                    "sqrt" if *n >= 0.0 => Some(n.sqrt()),
                    "abs" => Some(n.abs()),
                    "asin" if *n >= -1.0 && *n <= 1.0 => Some(n.asin()),
                    "acos" if *n >= -1.0 && *n <= 1.0 => Some(n.acos()),
                    "atan" => Some(n.atan()),
                    "sinh" => Some(n.sinh()),
                    "cosh" => Some(n.cosh()),
                    "tanh" => Some(n.tanh()),
                    _ => None,
                };
                if let Some(val) = result {
                    return Expr::num(val);
                }
            }

            Expr::func(name, simplified_arg)
        }
    }
}
