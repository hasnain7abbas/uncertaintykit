import { useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import katex from 'katex';

interface Props {
  formula?: string;
  latex?: string;
  displayMode?: boolean;
}

export default function LatexDisplay({ formula, latex, displayMode = true }: Props) {
  const ref_ = useRef<HTMLDivElement>(null);
  const [latexStr, setLatexStr] = useState('');

  useEffect(() => {
    if (latex) {
      setLatexStr(latex);
    } else if (formula) {
      invoke<string>('formula_to_latex', { formula })
        .then(setLatexStr)
        .catch(() => {});
    }
  }, [formula, latex]);

  useEffect(() => {
    if (ref_.current && latexStr) {
      try {
        katex.render(latexStr, ref_.current, {
          throwOnError: false,
          displayMode,
        });
      } catch {
        // silently fail
      }
    }
  }, [latexStr, displayMode]);

  return <div ref={ref_} className="text-xl" />;
}
