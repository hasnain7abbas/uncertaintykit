use tauri::command;

use crate::math::propagator::{self, PropagationResult, VariableInput};
use crate::math::{formatter, parser, simplifier};

#[command]
pub fn propagate_uncertainty(
    formula: String,
    variables: Vec<VariableInput>,
) -> Result<PropagationResult, String> {
    propagator::propagate(&formula, &variables)
}

#[command]
pub fn detect_variables(formula: String) -> Result<Vec<String>, String> {
    let expr = parser::parse(&formula)?;
    Ok(expr.variables())
}

#[command]
pub fn formula_to_latex(formula: String) -> Result<String, String> {
    let expr = parser::parse(&formula)?;
    Ok(formatter::to_latex(&expr))
}

#[command]
pub fn get_derivative_latex(formula: String, variable: String) -> Result<String, String> {
    let expr = parser::parse(&formula)?;
    let deriv = crate::math::differentiator::differentiate(&expr, &variable);
    let simplified = simplifier::simplify(&deriv);
    Ok(formatter::to_latex(&simplified))
}
