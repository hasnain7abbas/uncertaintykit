import { VariableInput } from '../types';

interface Props {
  variables: VariableInput[];
  onChange: (vars: VariableInput[]) => void;
}

export default function VariableTable({ variables, onChange }: Props) {
  const update = (index: number, field: keyof VariableInput, value: string | number) => {
    const updated = [...variables];
    updated[index] = { ...updated[index], [field]: value };
    onChange(updated);
  };

  return (
    <div className="space-y-2">
      <label className="text-sm text-[#8892b0] uppercase tracking-wider font-medium">
        Variables
      </label>
      <div className="bg-[#161625] rounded-lg border border-[#2a2a40] overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="text-[#8892b0] text-xs uppercase border-b border-[#2a2a40]">
              <th className="px-4 py-2 text-left">Variable</th>
              <th className="px-4 py-2 text-left">Value</th>
              <th className="px-2 py-2 w-8 text-center">&plusmn;</th>
              <th className="px-4 py-2 text-left">Uncertainty</th>
              <th className="px-4 py-2 text-left">Unit</th>
            </tr>
          </thead>
          <tbody>
            {variables.map((v, i) => (
              <tr key={v.name} className="border-b border-[#1e1e30] last:border-b-0">
                <td className="px-4 py-2 font-mono font-semibold text-[#7ec8e3]">{v.name}</td>
                <td className="px-4 py-2">
                  <input
                    type="number"
                    step="any"
                    value={v.value || ''}
                    onChange={e => update(i, 'value', parseFloat(e.target.value) || 0)}
                    className="w-36 bg-[#0f0f1a] border border-[#2a2a40] rounded px-3 py-1.5
                               text-right font-mono focus:outline-none focus:border-[#7ec8e3]
                               transition-colors"
                    placeholder="0"
                  />
                </td>
                <td className="px-2 py-2 text-center text-[#8892b0] font-mono">&plusmn;</td>
                <td className="px-4 py-2">
                  <input
                    type="number"
                    step="any"
                    min="0"
                    value={v.uncertainty || ''}
                    onChange={e => update(i, 'uncertainty', parseFloat(e.target.value) || 0)}
                    className="w-36 bg-[#0f0f1a] border border-[#2a2a40] rounded px-3 py-1.5
                               text-right font-mono focus:outline-none focus:border-[#7ec8e3]
                               transition-colors"
                    placeholder="0"
                  />
                </td>
                <td className="px-4 py-2">
                  <input
                    type="text"
                    value={v.unit || ''}
                    onChange={e => update(i, 'unit', e.target.value)}
                    className="w-20 bg-[#0f0f1a] border border-[#2a2a40] rounded px-2 py-1.5
                               text-center font-mono text-[#8892b0] focus:outline-none
                               focus:border-[#7ec8e3] transition-colors"
                    placeholder="\u2014"
                  />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
