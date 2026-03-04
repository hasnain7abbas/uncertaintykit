export interface VariableInput {
  name: string;
  value: number;
  uncertainty: number;
  unit?: string;
}

export interface VariableContribution {
  name: string;
  partial_derivative_latex: string;
  partial_derivative_value: number;
  uncertainty: number;
  contribution: number;
  percentage: number;
}

export interface PropagationResult {
  value: number;
  uncertainty: number;
  relative_uncertainty: number;
  formula_latex: string;
  propagation_formula_latex: string;
  contributions: VariableContribution[];
  formatted_result: string;
  formatted_result_latex: string;
}

export interface FormulaEntry {
  name: string;
  category: string;
  formula: string;
  description: string;
  variables: FormulaVariable[];
}

export interface FormulaVariable {
  name: string;
  description: string;
  unit: string;
  typical_value: number | null;
}
