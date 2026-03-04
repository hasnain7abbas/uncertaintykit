use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::math::ast::Expr;
use crate::math::{differentiator, evaluator, formatter, parser, simplifier};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInput {
    pub name: String,
    pub value: f64,
    pub uncertainty: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableContribution {
    pub name: String,
    pub partial_derivative_latex: String,
    pub partial_derivative_value: f64,
    pub uncertainty: f64,
    pub contribution: f64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagationResult {
    pub value: f64,
    pub uncertainty: f64,
    pub relative_uncertainty: f64,
    pub formula_latex: String,
    pub propagation_formula_latex: String,
    pub contributions: Vec<VariableContribution>,
    pub formatted_result: String,
    pub formatted_result_latex: String,
}

pub fn propagate(
    formula_str: &str,
    variables: &[VariableInput],
) -> Result<PropagationResult, String> {
    // 1. Parse the formula
    let expr = parser::parse(formula_str)?;

    // 2. Build value map
    let mut values: HashMap<String, f64> = HashMap::new();
    for var in variables {
        values.insert(var.name.clone(), var.value);
    }

    // 3. Evaluate the formula
    let result_value = evaluator::evaluate(&expr, &values)?;

    // 4. For each variable, compute partial derivative
    let mut contributions = Vec::new();
    let mut total_variance = 0.0;

    for var in variables {
        if var.uncertainty <= 0.0 {
            continue;
        }

        let derivative = differentiator::differentiate(&expr, &var.name);
        let simplified = simplifier::simplify(&derivative);
        let deriv_value = evaluator::evaluate(&simplified, &values)?;

        let contribution = (deriv_value * var.uncertainty).powi(2);
        total_variance += contribution;

        contributions.push(VariableContribution {
            name: var.name.clone(),
            partial_derivative_latex: formatter::to_latex(&simplified),
            partial_derivative_value: deriv_value,
            uncertainty: var.uncertainty,
            contribution,
            percentage: 0.0,
        });
    }

    let total_uncertainty = total_variance.sqrt();

    // 5. Compute percentages
    if total_variance > 0.0 {
        for c in &mut contributions {
            c.percentage = (c.contribution / total_variance) * 100.0;
        }
    }

    // 6. Format the propagation formula in LaTeX
    let formula_latex = formatter::to_latex(&expr);

    let prop_terms: Vec<String> = contributions
        .iter()
        .map(|c| {
            let var_latex = formatter::to_latex(&Expr::var(&c.name));
            format!(
                r"\left(\frac{{\partial f}}{{\partial {}}}\right)^2 \delta {}^2",
                var_latex, var_latex
            )
        })
        .collect();

    let propagation_formula_latex = format!(r"\delta f = \sqrt{{{}}}", prop_terms.join(" + "));

    // 7. Format result
    let (formatted_val, formatted_unc) = format_with_uncertainty(result_value, total_uncertainty);
    let formatted_result = format!("{} \u{00B1} {}", formatted_val, formatted_unc);
    let formatted_result_latex = format!(r"{} \pm {}", formatted_val, formatted_unc);

    let relative = if result_value != 0.0 {
        (total_uncertainty / result_value.abs()) * 100.0
    } else {
        0.0
    };

    Ok(PropagationResult {
        value: result_value,
        uncertainty: total_uncertainty,
        relative_uncertainty: relative,
        formula_latex,
        propagation_formula_latex,
        contributions,
        formatted_result,
        formatted_result_latex,
    })
}

/// Format a value and uncertainty with correct significant figures.
fn format_with_uncertainty(value: f64, uncertainty: f64) -> (String, String) {
    if uncertainty == 0.0 || uncertainty.is_nan() || uncertainty.is_infinite() {
        return (format!("{:.6}", value), "0".to_string());
    }

    let unc_order = uncertainty.abs().log10().floor() as i32;
    let leading_digit = (uncertainty / 10_f64.powi(unc_order)) as u32;
    let sig_figs = if leading_digit <= 2 { 2 } else { 1 };

    let decimal_places = if unc_order >= 0 {
        0_usize
    } else {
        ((-unc_order) as usize) + sig_figs - 1
    };

    let rounded_unc = {
        let factor = 10_f64.powi(decimal_places as i32);
        (uncertainty * factor).round() / factor
    };
    let rounded_val = {
        let factor = 10_f64.powi(decimal_places as i32);
        (value * factor).round() / factor
    };

    (
        format!("{:.prec$}", rounded_val, prec = decimal_places),
        format!("{:.prec$}", rounded_unc, prec = decimal_places),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagate_velocity() {
        let result = propagate(
            "d / t",
            &[
                VariableInput {
                    name: "d".into(),
                    value: 1.5,
                    uncertainty: 0.03,
                    unit: None,
                },
                VariableInput {
                    name: "t".into(),
                    value: 2.1,
                    uncertainty: 0.05,
                    unit: None,
                },
            ],
        )
        .unwrap();
        assert!((result.value - 0.714286).abs() < 0.001);
        assert!((result.uncertainty - 0.0214).abs() < 0.005);
    }

    #[test]
    fn test_propagate_bragg() {
        let result = propagate(
            "n * lambda / (2 * sin(theta))",
            &[
                VariableInput {
                    name: "n".into(),
                    value: 1.0,
                    uncertainty: 0.0,
                    unit: None,
                },
                VariableInput {
                    name: "lambda".into(),
                    value: 1.5406,
                    uncertainty: 0.0001,
                    unit: None,
                },
                VariableInput {
                    name: "theta".into(),
                    value: 0.3927,
                    uncertainty: 0.0017,
                    unit: None,
                },
            ],
        )
        .unwrap();
        assert!(result.value > 0.0);
        assert!(result.uncertainty > 0.0);
    }
}
