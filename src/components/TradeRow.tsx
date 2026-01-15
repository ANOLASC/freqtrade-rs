import { Sparkles } from 'lucide-react';
import { Trade } from '../types';

interface TradeRowProps {
  trade: Trade;
  onAnalyze: (trade: Trade) => void;
}

const TradeRow = ({ trade, onAnalyze }: TradeRowProps) => (
    <tr className="border-b border-slate-700/50 hover:bg-slate-700/20 transition-colors">
      <td className="py-4 px-4">
        <div className="flex items-center">
          <div className="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-xs font-bold text-white mr-3">
            {trade.pair.substring(0, 1)}
          </div>
          <div>
            <div className="font-semibold text-white">{trade.pair}</div>
            <div className="text-xs text-slate-400">{trade.strategy}</div>
          </div>
        </div>
      </td>
      <td className="py-4 px-4 text-slate-300 font-mono text-sm">{trade.entry}</td>
      <td className="py-4 px-4 text-slate-300 font-mono text-sm">{trade.current}</td>
      <td className="py-4 px-4">
        <div className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${(trade.profit ?? 0) >= 0 ? 'bg-emerald-500/10 text-emerald-400' : 'bg-rose-500/10 text-rose-400'}`}>
          {(trade.profit ?? 0) >= 0 ? '+' : ''}{trade.profit ?? 0}%
        </div>
      </td>
      <td className="py-4 px-4 text-right">
        <div className="flex justify-end space-x-2">
          <button onClick={() => onAnalyze(trade)} className="flex items-center space-x-1 text-xs bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-300 border border-indigo-500/30 px-3 py-1.5 rounded-md transition-colors">
            <Sparkles size={12} />
            <span>Analyze</span>
          </button>
          <button className="text-xs bg-slate-700 hover:bg-rose-600 hover:text-white text-slate-300 px-3 py-1.5 rounded-md transition-colors border border-slate-600 hover:border-rose-500">
            Force Exit
          </button>
        </div>
      </td>
    </tr>
  );

  export default TradeRow;
