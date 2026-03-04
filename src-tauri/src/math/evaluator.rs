use std::collections::HashMap;

use crate::math::ast::{BinOp, Expr};

/// Evaluate an expression given variable values.
pub fn evaluate(expr: &Expr, values: &HashMap<String, f64>) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),

        Expr::Constant(_, val) => Ok(*val),

        Expr::Variable(name) => values
            .get(name)
            .copied()
            .ok_or_else(|| format!("Undefined variable: {}", name)),

        Expr::Negate(e) => Ok(-evaluate(e, values)?),

        Expr::BinaryOp { op, left, right } => {
            let l = evaluate(left, values)?;
            let r = evaluate(right, values)?;
            match op {
                BinOp::Add => Ok(l + r),
                BinOp::Sub => Ok(l - r),
                BinOp::Mul => Ok(l * r),
                BinOp::Div => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
                BinOp::Pow => Ok(l.powf(r)),
            }
        }

        Expr::Function { name, arg } => {
            let x = evaluate(arg, values)?;
            match name.as_str() {
                "sin" => Ok(x.sin()),
                "cos" => Ok(x.cos()),
                "tan" => Ok(x.tan()),
                "asin" => Ok(x.asin()),
                "acos" => Ok(x.acos()),
                "atan" => Ok(x.atan()),
                "sinh" => Ok(x.sinh()),
                "cosh" => Ok(x.cosh()),
                "tanh" => Ok(x.tanh()),
                "exp" => Ok(x.exp()),
                "ln" | "log" => {
                    if x <= 0.0 {
                        Err(format!("ln of non-positive number: {}", x))
                    } else {
                        Ok(x.ln())
                    }
                }
                "log10" => {
                    if x <= 0.0 {
                        Err(format!("log10 of non-positive number: {}", x))
                    } else {
                        Ok(x.log10())
                    }
                }
                "log2" => {
                    if x <= 0.0 {
                        Err(format!("log2 of non-positive number: {}", x))
                    } else {
                        Ok(x.log2())
                    }
                }
                "sqrt" => {
                    if x < 0.0 {
                        Err(format!("sqrt of negative number: {}", x))
                    } else {
                        Ok(x.sqrt())
                    }
                }
                "abs" => Ok(x.abs()),
                "ceil" => Ok(x.ceil()),
                "floor" => Ok(x.floor()),
                _ => Err(format!("Unknown function: {}", name)),
            }
        }
    }
}
