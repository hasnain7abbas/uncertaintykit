import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { X, Search } from 'lucide-react';
import { FormulaEntry } from '../types';

interface Props {
  onSelect: (formula: FormulaEntry) => void;
  onClose: () => void;
}

export default function FormulaLibrary({ onSelect, onClose }: Props) {
  const [formulas, setFormulas] = useState<FormulaEntry[]>([]);
  const [search, setSearch] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);

  useEffect(() => {
    invoke<FormulaEntry[]>('get_formulas').then(setFormulas).catch(() => {});
  }, []);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [onClose]);

  const categories = [...new Set(formulas.map(f => f.category))];

  const filtered = formulas.filter(f => {
    const matchesSearch = !search ||
      f.name.toLowerCase().includes(search.toLowerCase()) ||
      f.description.toLowerCase().includes(search.toLowerCase()) ||
      f.formula.toLowerCase().includes(search.toLowerCase());
    const matchesCategory = !selectedCategory || f.category === selectedCategory;
    return matchesSearch && matchesCategory;
  });

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50"
      onClick={onClose}>
      <div className="bg-[#161625] border border-[#2a2a40] rounded-xl w-[600px] max-h-[80vh] flex flex-col shadow-2xl"
        onClick={e => e.stopPropagation()}>
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-[#2a2a40]">
          <h2 className="text-lg font-semibold">Formula Library</h2>
          <button onClick={onClose} className="text-[#8892b0] hover:text-white transition-colors">
            <X size={20} />
          </button>
        </div>

        {/* Search */}
        <div className="p-4 border-b border-[#2a2a40]">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-[#8892b0]" />
            <input
              type="text"
              value={search}
              onChange={e => setSearch(e.target.value)}
              placeholder="Search formulas..."
              className="w-full bg-[#0f0f1a] border border-[#2a2a40] rounded-lg pl-10 pr-4 py-2
                         text-sm focus:outline-none focus:border-[#7ec8e3] transition-colors"
              autoFocus
            />
          </div>

          {/* Category filters */}
          <div className="flex flex-wrap gap-2 mt-3">
            <button
              onClick={() => setSelectedCategory(null)}
              className={`px-3 py-1 rounded-full text-xs transition-colors ${
                !selectedCategory
                  ? 'bg-[#7ec8e3] text-[#0f0f1a]'
                  : 'bg-[#0f0f1a] text-[#8892b0] hover:text-white'
              }`}
            >
              All
            </button>
            {categories.map(cat => (
              <button
                key={cat}
                onClick={() => setSelectedCategory(cat === selectedCategory ? null : cat)}
                className={`px-3 py-1 rounded-full text-xs transition-colors ${
                  selectedCategory === cat
                    ? 'bg-[#7ec8e3] text-[#0f0f1a]'
                    : 'bg-[#0f0f1a] text-[#8892b0] hover:text-white'
                }`}
              >
                {cat}
              </button>
            ))}
          </div>
        </div>

        {/* Formula list */}
        <div className="flex-1 overflow-y-auto p-2">
          {filtered.map((f, i) => (
            <button
              key={i}
              onClick={() => onSelect(f)}
              className="w-full text-left p-3 rounded-lg hover:bg-[#1e1e30] transition-colors group"
            >
              <div className="flex items-baseline justify-between">
                <span className="font-medium text-sm group-hover:text-[#7ec8e3] transition-colors">
                  {f.name}
                </span>
                <span className="text-xs text-[#8892b0]">{f.category}</span>
              </div>
              <div className="text-xs text-[#8892b0] mt-0.5">{f.description}</div>
              <div className="font-mono text-xs text-[#50fa7b] mt-1">{f.formula}</div>
            </button>
          ))}
          {filtered.length === 0 && (
            <div className="text-center text-[#8892b0] text-sm py-8">No formulas found</div>
          )}
        </div>
      </div>
    </div>
  );
}
