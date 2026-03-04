use crate::math::ast::{BinOp, Expr};

/// Compute the symbolic partial derivative of `expr` with respect to `var`.
pub fn differentiate(expr: &Expr, var: &str) -> Expr {
    match expr {
        Expr::Number(_) | Expr::Constant(_, _) => Expr::num(0.0),

        Expr::Variable(name) => {
            if name == var {
                Expr::num(1.0)
            } else {
                Expr::num(0.0)
            }
        }

        Expr::Negate(e) => Expr::neg(differentiate(e, var)),

        Expr::BinaryOp { op, left, right } => {
            let f = left.as_ref();
            let g = right.as_ref();
            let df = differentiate(f, var);
            let dg = differentiate(g, var);

            match op {
                BinOp::Add => Expr::add(df, dg),
                BinOp::Sub => Expr::sub(df, dg),

                // Product rule
                BinOp::Mul => Expr::add(Expr::mul(f.clone(), dg), Expr::mul(g.clone(), df)),

                // Quotient rule
                BinOp::Div => Expr::div(
                    Expr::sub(Expr::mul(g.clone(), df), Expr::mul(f.clone(), dg)),
                    Expr::pow(g.clone(), Expr::num(2.0)),
                ),

                // Power rule (general)
                BinOp::Pow => {
                    let f_has_var = contains_var(f, var);
                    let g_has_var = contains_var(g, var);

                    if !g_has_var {
                        // Simple power rule: g * f^(g-1) * df/dx
                        Expr::mul(
                            Expr::mul(
                                g.clone(),
                                Expr::pow(f.clone(), Expr::sub(g.clone(), Expr::num(1.0))),
                            ),
                            df,
                        )
                    } else if !f_has_var {
                        // Exponential rule: f^g * ln(f) * dg/dx
                        Expr::mul(
                            Expr::mul(
                                Expr::pow(f.clone(), g.clone()),
                                Expr::func("ln", f.clone()),
                            ),
                            dg,
                        )
                    } else {
                        // General: f^g * (g' * ln(f) + g * f'/f)
                        Expr::mul(
                            Expr::pow(f.clone(), g.clone()),
                            Expr::add(
                                Expr::mul(dg, Expr::func("ln", f.clone())),
                                Expr::mul(g.clone(), Expr::div(df, f.clone())),
                            ),
                        )
                    }
                }
            }
        }

        // Chain rule: func'(u) * du/dx
        Expr::Function { name, arg } => {
            let u = arg.as_ref();
            let du = differentiate(u, var);

            let outer_deriv = match name.as_str() {
                "sin" => Expr::func("cos", u.clone()),
                "cos" => Expr::neg(Expr::func("sin", u.clone())),
                "tan" => Expr::div(
                    Expr::num(1.0),
                    Expr::pow(Expr::func("cos", u.clone()), Expr::num(2.0)),
                ),
                "exp" => Expr::func("exp", u.clone()),
                "ln" | "log" => Expr::div(Expr::num(1.0), u.clone()),
                "log10" => Expr::div(
                    Expr::num(1.0),
                    Expr::mul(u.clone(), Expr::func("ln", Expr::num(10.0))),
                ),
                "sqrt" => Expr::div(
                    Expr::num(1.0),
                    Expr::mul(Expr::num(2.0), Expr::func("sqrt", u.clone())),
                ),
                "asin" => Expr::div(
                    Expr::num(1.0),
                    Expr::func(
                        "sqrt",
                        Expr::sub(Expr::num(1.0), Expr::pow(u.clone(), Expr::num(2.0))),
                    ),
                ),
                "acos" => Expr::neg(Expr::div(
                    Expr::num(1.0),
                    Expr::func(
                        "sqrt",
                        Expr::sub(Expr::num(1.0), Expr::pow(u.clone(), Expr::num(2.0))),
                    ),
                )),
                "atan" => Expr::div(
                    Expr::num(1.0),
                    Expr::add(Expr::num(1.0), Expr::pow(u.clone(), Expr::num(2.0))),
                ),
                "sinh" => Expr::func("cosh", u.clone()),
                "cosh" => Expr::func("sinh", u.clone()),
                "tanh" => Expr::sub(
                    Expr::num(1.0),
                    Expr::pow(Expr::func("tanh", u.clone()), Expr::num(2.0)),
                ),
                "abs" => Expr::div(u.clone(), Expr::func("abs", u.clone())),
                _ => return Expr::num(f64::NAN),
            };

            Expr::mul(outer_deriv, du)
        }
    }
}

/// Check if an expression contains a given variable
fn contains_var(expr: &Expr, var: &str) -> bool {
    match expr {
        Expr::Variable(name) => name == var,
        Expr::Number(_) | Expr::Constant(_, _) => false,
        Expr::Negate(e) | Expr::Function { arg: e, .. } => contains_var(e, var),
        Expr::BinaryOp { left, right, .. } => contains_var(left, var) || contains_var(right, var),
    }
}
