import { Copy, FileText } from 'lucide-react';
import { PropagationResult } from '../types';
import LatexDisplay from './LatexDisplay';

interface Props {
  result: PropagationResult;
}

export default function ResultPanel({ result }: Props) {
  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      // fallback
    }
  };

  return (
    <div className="space-y-4">
      {/* Main result */}
      <div className="bg-[#161625] rounded-lg border border-[#2a2a40] p-6">
        <div className="text-sm text-[#8892b0] mb-2 uppercase tracking-wider">Result</div>
        <div className="text-3xl font-mono font-bold tracking-tight">
          <LatexDisplay latex={result.formatted_result_latex} displayMode={true} />
        </div>
        <div className="text-sm text-[#8892b0] mt-3">
          Relative uncertainty: {result.relative_uncertainty.toFixed(2)}%
        </div>
      </div>

      {/* Propagation formula */}
      <div className="bg-[#161625] rounded-lg border border-[#2a2a40] p-4">
        <div className="text-sm text-[#8892b0] mb-2 uppercase tracking-wider">
          Propagation Formula
        </div>
        <LatexDisplay latex={result.propagation_formula_latex} displayMode={true} />
      </div>

      {/* Contribution breakdown */}
      {result.contributions.length > 0 && (
        <div className="bg-[#161625] rounded-lg border border-[#2a2a40] p-4">
          <div className="text-sm text-[#8892b0] mb-3 uppercase tracking-wider">
            Uncertainty Contributions
          </div>
          {result.contributions.map(c => (
            <div key={c.name} className="mb-4 last:mb-0">
              <div className="flex justify-between text-sm mb-1">
                <span className="font-mono font-semibold text-[#7ec8e3]">{c.name}</span>
                <span className="font-mono">{c.percentage.toFixed(1)}%</span>
              </div>
              <div className="h-2.5 bg-[#0f0f1a] rounded-full overflow-hidden">
                <div
                  className="h-full rounded-full transition-all duration-500"
                  style={{
                    width: `${Math.max(c.percentage, 1)}%`,
                    background: `linear-gradient(90deg, #7ec8e3, ${c.percentage > 50 ? '#e94560' : '#50fa7b'})`,
                  }}
                />
              </div>
              <div className="text-xs text-[#8892b0] mt-1 font-mono">
                <LatexDisplay
                  latex={`\\frac{\\partial f}{\\partial ${c.name}} = ${c.partial_derivative_value.toExponential(3)}`}
                  displayMode={false}
                />
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Export buttons */}
      <div className="flex gap-2">
        <button
          className="flex items-center gap-2 px-4 py-2 bg-[#1e1e30] border border-[#2a2a40]
                     rounded-lg hover:bg-[#2a2a40] hover:border-[#7ec8e3] text-sm transition-colors"
          onClick={() => copyToClipboard(result.formatted_result)}
        >
          <Copy size={14} />
          Copy as Text
        </button>
        <button
          className="flex items-center gap-2 px-4 py-2 bg-[#1e1e30] border border-[#2a2a40]
                     rounded-lg hover:bg-[#2a2a40] hover:border-[#7ec8e3] text-sm transition-colors"
          onClick={() => copyToClipboard(result.formatted_result_latex)}
        >
          <FileText size={14} />
          Copy as LaTeX
        </button>
      </div>
    </div>
  );
}
