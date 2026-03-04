import { BookOpen } from 'lucide-react';

interface Props {
  value: string;
  onChange: (value: string) => void;
  onOpenLibrary: () => void;
}

export default function FormulaInput({ value, onChange, onOpenLibrary }: Props) {
  return (
    <div className="space-y-2">
      <label className="text-sm text-[#8892b0] uppercase tracking-wider font-medium">
        Formula
      </label>
      <div className="flex gap-2">
        <input
          type="text"
          value={value}
          onChange={e => onChange(e.target.value)}
          placeholder="e.g. d / t, 0.5 * m * v^2, sin(theta)..."
          className="flex-1 bg-[#161625] border border-[#2a2a40] rounded-lg px-4 py-3 text-lg font-mono
                     text-[#e0e0e0] placeholder-[#4a4a60] focus:outline-none focus:border-[#7ec8e3]
                     transition-colors"
          autoFocus
          spellCheck={false}
        />
        <button
          onClick={onOpenLibrary}
          className="px-4 py-3 bg-[#161625] border border-[#2a2a40] rounded-lg hover:bg-[#1e1e30]
                     hover:border-[#7ec8e3] transition-colors text-[#8892b0] hover:text-[#7ec8e3]"
          title="Formula Library (Ctrl+L)"
        >
          <BookOpen size={20} />
        </button>
      </div>
    </div>
  );
}
