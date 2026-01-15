import { useState } from 'react';
import { Cpu, Zap, Loader, Sparkles, Download, ChevronDown } from 'lucide-react';

interface HyperoptResult {
  epoch: number;
  profit: number | string;
  trades: number;
  winRate: number | string;
  avgDuration: string;
  best: boolean;
}

const strategies = ['NostalgiaForInfinity', 'BinanceCluc', 'DoesNothingStrategy', 'SMAOffset'];
const hyperoptResultsMock: HyperoptResult[] = [
    { epoch: 120, profit: 154.2, trades: 410, winRate: 68.5, avgDuration: '45m', best: true },
    { epoch: 98, profit: 142.8, trades: 380, winRate: 66.2, avgDuration: '50m', best: false },
    { epoch: 45, profit: 110.5, trades: 350, winRate: 60.1, avgDuration: '30m', best: false },
    { epoch: 12, profit: 45.2, trades: 200, winRate: 55.4, avgDuration: '1h 10m', best: false },
  ];

const HyperoptView = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [progress, setProgress] = useState(0);
    const [epochs, setEpochs] = useState(100);
    const [results, setResults] = useState<HyperoptResult[]>([]);

    const runHyperopt = () => {
      setIsRunning(true);
      setResults([]);
      setProgress(0);

      let currentEpoch = 0;
      const interval = setInterval(() => {
        currentEpoch += 2;
        const p = Math.min((currentEpoch / epochs) * 100, 100);
        setProgress(p);

        if (Math.random() > 0.7) {
          setResults(prev => [{
            epoch: currentEpoch,
            profit: (Math.random() * 200).toFixed(1),
            trades: Math.floor(Math.random() * 500),
            winRate: (40 + Math.random() * 40).toFixed(1),
            avgDuration: `${Math.floor(Math.random() * 60) + 10}m`,
            best: false
          }, ...prev].slice(0, 10)); // Keep last 10
        }

        if (p >= 100) {
          clearInterval(interval);
          setIsRunning(false);
          setResults(hyperoptResultsMock); // Show final mock results
        }
      }, 100);
    };

    return (
      <div className="h-[calc(100vh-140px)] flex flex-col lg:flex-row gap-6 animate-in fade-in duration-500">

        {/* Config Panel */}
        <div className="w-full lg:w-80 bg-slate-800/50 border border-slate-700/50 rounded-xl p-6 flex flex-col h-full overflow-y-auto custom-scrollbar">
          <h2 className="text-xl font-bold text-white mb-6 flex items-center">
            <Cpu size={20} className="mr-2 text-indigo-400" /> Hyperopt Config
          </h2>

          <div className="space-y-5 flex-1">
            <div>
              <label className="text-xs font-semibold text-slate-400 uppercase mb-2 block">Strategy</label>
              <div className="relative">
                <select className="w-full appearance-none bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 text-sm focus:border-indigo-500 outline-none transition-colors">
                  {strategies.map(s => <option key={s}>{s}</option>)}
                </select>
                <ChevronDown size={14} className="absolute right-3 top-3.5 text-slate-500 pointer-events-none" />
              </div>
            </div>

            <div>
              <label className="text-xs font-semibold text-slate-400 uppercase mb-2 block">Optimization Spaces</label>
              <div className="space-y-2 bg-slate-900/50 p-3 rounded-lg border border-slate-700/50">
                {['Buy Signal', 'Sell Signal', 'ROI', 'Stoploss', 'Trailing'].map((space) => (
                  <label key={space} className="flex items-center space-x-3 cursor-pointer group">
                    <div className="relative flex items-center">
                      <input type="checkbox" defaultChecked className="peer h-4 w-4 rounded border-slate-600 bg-slate-800 text-indigo-600 focus:ring-0 focus:ring-offset-0 transition-all" />
                    </div>
                    <span className="text-sm text-slate-300 group-hover:text-white transition-colors">{space}</span>
                  </label>
                ))}
              </div>
            </div>

            <div>
              <label className="text-xs font-semibold text-slate-400 uppercase mb-2 block">Epochs</label>
              <input
                type="number"
                value={epochs}
                onChange={(e) => setEpochs(Number(e.target.value))}
                className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 text-sm focus:border-indigo-500 outline-none"
              />
            </div>
          </div>

          <button
            onClick={runHyperopt}
            disabled={isRunning}
            className={`mt-6 w-full py-3 rounded-lg font-bold text-white shadow-lg flex items-center justify-center transition-all ${isRunning ? 'bg-slate-700 cursor-not-allowed' : 'bg-gradient-to-r from-pink-600 to-rose-600 hover:from-pink-500 hover:to-rose-500 shadow-rose-900/20'}`}
          >
            {isRunning ? (
              <><Loader size={18} className="animate-spin mr-2" /> Optimizing...</>
            ) : (
              <><Zap size={18} className="mr-2 fill-current" /> Start Hyperopt</>
            )}
          </button>
        </div>

        {/* Results Area */}
        <div className="flex-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-6 flex flex-col min-h-[500px] relative overflow-hidden">
          {!isRunning && results.length === 0 && (
            <div className="absolute inset-0 flex flex-col items-center justify-center text-slate-500 opacity-40">
              <Cpu size={64} className="mb-4" />
              <p className="text-lg">Ready to optimize strategy parameters</p>
            </div>
          )}

          {isRunning && (
             <div className="mb-6">
               <div className="flex justify-between text-sm text-slate-400 mb-2">
                 <span>Progress</span>
                 <span>{Math.round(progress)}%</span>
               </div>
               <div className="w-full bg-slate-700 rounded-full h-2 overflow-hidden">
                 <div className="bg-gradient-to-r from-pink-500 to-rose-500 h-full transition-all duration-100 ease-out" style={{ width: `${progress}%` }}></div>
               </div>
             </div>
          )}

          {(results.length > 0 || isRunning) && (
            <div className="flex-1 overflow-hidden flex flex-col animate-in fade-in slide-in-from-bottom-4 duration-500">
               <div className="flex justify-between items-center mb-4">
                 <h3 className="text-lg font-bold text-white">Best Epochs</h3>
                 <button className="flex items-center space-x-1 px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-white text-xs rounded transition-colors">
                    <Download size={14} /> <span>Export JSON</span>
                 </button>
               </div>

               <div className="overflow-auto custom-scrollbar flex-1 rounded-lg border border-slate-700/50">
                 <table className="w-full text-left border-collapse">
                   <thead className="sticky top-0 bg-slate-900/90 backdrop-blur-sm z-10">
                     <tr className="text-xs text-slate-400 uppercase tracking-wider border-b border-slate-700">
                       <th className="py-3 px-4 font-medium">Epoch</th>
                       <th className="py-3 px-4 font-medium">Total Profit</th>
                       <th className="py-3 px-4 font-medium">Trades</th>
                       <th className="py-3 px-4 font-medium">Win Rate</th>
                       <th className="py-3 px-4 font-medium">Avg Duration</th>
                     </tr>
                   </thead>
                   <tbody>
                     {results.map((res, idx) => (
                       <tr key={idx} className={`border-b border-slate-700/30 hover:bg-slate-700/20 transition-colors ${res.best ? 'bg-emerald-500/10' : ''}`}>
                         <td className="py-3 px-4 text-slate-300 font-mono">
                           {res.best && <Sparkles size={12} className="inline mr-1 text-yellow-400" />}
                           {res.epoch}
                         </td>
                         <td className={`py-3 px-4 font-mono font-bold ${Number(res.profit) > 0 ? 'text-emerald-400' : 'text-rose-400'}`}>
                           {Number(res.profit) > 0 ? '+' : ''}{res.profit}%
                         </td>
                         <td className="py-3 px-4 text-slate-300 font-mono">{res.trades}</td>
                         <td className="py-3 px-4 text-indigo-400 font-mono">{res.winRate}%</td>
                         <td className="py-3 px-4 text-slate-400 text-sm">{res.avgDuration}</td>
                       </tr>
                     ))}
                   </tbody>
                 </table>
               </div>
            </div>
          )}
        </div>
      </div>
    );
  };

  export default HyperoptView;