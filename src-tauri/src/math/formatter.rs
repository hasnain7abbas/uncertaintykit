use crate::math::ast::{BinOp, Expr};

/// Convert an expression to LaTeX string.
pub fn to_latex(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => {
            if *n == std::f64::consts::PI {
                return r"\pi".to_string();
            }
            if *n == std::f64::consts::E {
                return "e".to_string();
            }
            if n.fract() == 0.0 && n.abs() < 1e10 {
                format!("{}", *n as i64)
            } else {
                format!("{:.6}", n)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            }
        }

        Expr::Variable(name) => match name.as_str() {
            "theta" => r"\theta".to_string(),
            "lambda" => r"\lambda".to_string(),
            "alpha" => r"\alpha".to_string(),
            "beta" => r"\beta".to_string(),
            "gamma" => r"\gamma".to_string(),
            "delta" => r"\delta".to_string(),
            "Delta" => r"\Delta".to_string(),
            "omega" => r"\omega".to_string(),
            "mu" => r"\mu".to_string(),
            "sigma" => r"\sigma".to_string(),
            "rho" => r"\rho".to_string(),
            "epsilon" => r"\epsilon".to_string(),
            "phi" => r"\phi".to_string(),
            "psi" => r"\psi".to_string(),
            s if s.len() > 1 && s.chars().all(|c| c.is_alphabetic()) => {
                if s.len() == 2 && s.chars().nth(1).unwrap().is_ascii_digit() {
                    format!("{}_{}", &s[0..1], &s[1..2])
                } else {
                    format!(r"\mathrm{{{}}}", s)
                }
            }
            s => s.to_string(),
        },

        Expr::Constant(name, _) => match name.as_str() {
            "\u{03C0}" => r"\pi".to_string(),
            "e" => "e".to_string(),
            _ => name.clone(),
        },

        Expr::Negate(e) => {
            let inner = to_latex(e);
            match e.as_ref() {
                Expr::BinaryOp {
                    op: BinOp::Add | BinOp::Sub,
                    ..
                } => format!("-({})", inner),
                _ => format!("-{}", inner),
            }
        }

        Expr::BinaryOp { op, left, right } => {
            let l = to_latex(left);
            let r = to_latex(right);

            match op {
                BinOp::Add => format!("{} + {}", l, r),
                BinOp::Sub => match right.as_ref() {
                    Expr::BinaryOp {
                        op: BinOp::Add | BinOp::Sub,
                        ..
                    } => {
                        format!("{} - ({})", l, r)
                    }
                    _ => format!("{} - {}", l, r),
                },
                BinOp::Mul => {
                    let needs_dot = !matches!(
                        (left.as_ref(), right.as_ref()),
                        (Expr::Number(_), Expr::Variable(_))
                            | (Expr::Number(_), Expr::Function { .. })
                    );
                    if needs_dot {
                        let l_wrapped = needs_mul_parens(left, true);
                        let r_wrapped = needs_mul_parens(right, false);
                        format!(r"{} \cdot {}", l_wrapped, r_wrapped)
                    } else {
                        format!("{} {}", l, r)
                    }
                }
                BinOp::Div => format!(r"\frac{{{}}}{{{}}}", l, r),
                BinOp::Pow => {
                    let base = match left.as_ref() {
                        Expr::BinaryOp { .. } | Expr::Negate(_) => format!("({})", l),
                        _ => l,
                    };
                    format!("{}^{{{}}}", base, r)
                }
            }
        }

        Expr::Function { name, arg } => {
            let inner = to_latex(arg);
            match name.as_str() {
                "sqrt" => format!(r"\sqrt{{{}}}", inner),
                "abs" => format!(r"\left|{}\right|", inner),
                "ln" => format!(r"\ln\left({}\right)", inner),
                "log" => format!(r"\ln\left({}\right)", inner),
                "log10" => format!(r"\log_{{10}}\left({}\right)", inner),
                "log2" => format!(r"\log_{{2}}\left({}\right)", inner),
                n => format!(r"\{}\left({}\right)", n, inner),
            }
        }
    }
}

fn needs_mul_parens(expr: &Expr, _is_left: bool) -> String {
    let latex = to_latex(expr);
    match expr {
        Expr::BinaryOp {
            op: BinOp::Add | BinOp::Sub,
            ..
        } => format!("({})", latex),
        _ => latex,
    }
}
