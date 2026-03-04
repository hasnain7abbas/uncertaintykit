import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { VariableInput, PropagationResult, FormulaEntry } from './types';
import FormulaInput from './components/FormulaInput';
import VariableTable from './components/VariableTable';
import ResultPanel from './components/ResultPanel';
import LatexDisplay from './components/LatexDisplay';
import FormulaLibrary from './components/FormulaLibrary';

function App() {
  const [formula, setFormula] = useState<string>('');
  const [variables, setVariables] = useState<VariableInput[]>([]);
  const [result, setResult] = useState<PropagationResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [showLibrary, setShowLibrary] = useState(false);

  // Auto-detect variables when formula changes (debounced)
  useEffect(() => {
    if (!formula.trim()) {
      setVariables([]);
      setResult(null);
      setError(null);
      return;
    }

    const timer = setTimeout(async () => {
      try {
        const vars = await invoke<string[]>('detect_variables', { formula });
        setVariables(prev => {
          return vars.map(name => {
            const existing = prev.find(v => v.name === name);
            return existing || { name, value: 0, uncertainty: 0 };
          });
        });
        setError(null);
      } catch (e) {
        setError(String(e));
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [formula]);

  const calculate = useCallback(async () => {
    if (!formula.trim() || variables.length === 0) return;

    try {
      const res = await invoke<PropagationResult>('propagate_uncertainty', {
        formula,
        variables,
      });
      setResult(res);
      setError(null);
    } catch (e) {
      setError(String(e));
      setResult(null);
    }
  }, [formula, variables]);

  // Auto-calculate when variables change and have values
  useEffect(() => {
    if (variables.length > 0 && variables.some(v => v.value !== 0 || v.uncertainty !== 0)) {
      const timer = setTimeout(calculate, 300);
      return () => clearTimeout(timer);
    }
  }, [variables, calculate]);

  // Keyboard shortcuts
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault();
        calculate();
      }
      if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
        e.preventDefault();
        setShowLibrary(s => !s);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [calculate]);

  const handleFormulaSelect = (f: FormulaEntry) => {
    setFormula(f.formula);
    // Pre-populate variables with typical values if available
    setVariables(
      f.variables.map(v => ({
        name: v.name,
        value: v.typical_value ?? 0,
        uncertainty: 0,
        unit: v.unit || undefined,
      }))
    );
    setShowLibrary(false);
  };

  return (
    <div className="h-screen flex flex-col bg-[#0f0f1a] text-[#e0e0e0]">
      {/* Top bar */}
      <header className="h-12 flex items-center justify-between px-5 bg-[#161625] border-b border-[#2a2a40] shrink-0 select-none"
        data-tauri-drag-region>
        <span className="text-base font-semibold tracking-tight text-[#7ec8e3]">
          UncertaintyKit
        </span>
        <div className="text-xs text-[#8892b0]">
          <kbd className="px-1.5 py-0.5 bg-[#0f0f1a] rounded border border-[#2a2a40] text-[10px]">
            Ctrl+Enter
          </kbd>
          {' '}Calculate
          <span className="mx-2 text-[#2a2a40]">|</span>
          <kbd className="px-1.5 py-0.5 bg-[#0f0f1a] rounded border border-[#2a2a40] text-[10px]">
            Ctrl+L
          </kbd>
          {' '}Library
        </div>
      </header>

      {/* Main content */}
      <div className="flex-1 overflow-y-auto">
        <div className="max-w-3xl mx-auto p-6 space-y-6">
          {/* Formula input */}
          <FormulaInput
            value={formula}
            onChange={setFormula}
            onOpenLibrary={() => setShowLibrary(true)}
          />

          {/* Live LaTeX preview */}
          {formula && !error && (
            <div className="bg-[#161625] rounded-lg border border-[#2a2a40] p-4">
              <LatexDisplay formula={formula} displayMode={true} />
            </div>
          )}

          {/* Variable table */}
          {variables.length > 0 && (
            <VariableTable variables={variables} onChange={setVariables} />
          )}

          {/* Calculate button */}
          {variables.length > 0 && (
            <button
              onClick={calculate}
              className="w-full py-3 bg-[#7ec8e3] text-[#0f0f1a] font-semibold rounded-lg
                         hover:bg-[#6ab8d3] active:bg-[#5aa8c3] transition-colors"
            >
              Calculate Uncertainty
            </button>
          )}

          {/* Error */}
          {error && (
            <div className="bg-[#2a1520] border border-[#e94560]/30 rounded-lg p-4 text-sm text-[#e94560]">
              {error}
            </div>
          )}

          {/* Result */}
          {result && <ResultPanel result={result} />}
        </div>
      </div>

      {/* Formula Library Modal */}
      {showLibrary && (
        <FormulaLibrary
          onSelect={handleFormulaSelect}
          onClose={() => setShowLibrary(false)}
        />
      )}
    </div>
  );
}

export default App;
