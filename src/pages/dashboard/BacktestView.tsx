import { useState } from 'react';
import { LineChart, Line, CartesianGrid, XAxis, YAxis, Tooltip as RechartsTooltip, ResponsiveContainer, Legend } from 'recharts';
import { Clock, Play, Loader, Save, Trash2, Calendar, ChevronDown, Activity } from 'lucide-react';

const strategies = ['NostalgiaForInfinity', 'BinanceCluc', 'DoesNothingStrategy', 'SMAOffset'];
const backtestResultsData = [
    { date: 'Jan', strategy: 10, buyHold: 5 },
    { date: 'Feb', strategy: 15, buyHold: 8 },
    { date: 'Mar', strategy: 12, buyHold: 4 },
    { date: 'Apr', strategy: 25, buyHold: 15 },
    { date: 'May', strategy: 32, buyHold: 12 },
    { date: 'Jun', strategy: 45, buyHold: 20 },
  ];


const BacktestView = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [progress, setProgress] = useState(0);
    const [hasResult, setHasResult] = useState(false);

    const runBacktest = () => {
      setIsRunning(true);
      setHasResult(false);
      setProgress(0);
      // Simulate backtest process
      let p = 0;
      const interval = setInterval(() => {
        p += 5;
        setProgress(p);
        if (p >= 100) {
          clearInterval(interval);
          setIsRunning(false);
          setHasResult(true);
        }
      }, 100);
    };

    return (
      <div className="h-[calc(100vh-140px)] flex flex-col lg:flex-row gap-6 animate-in fade-in duration-500">

        {/* Config Panel */}
        <div className="w-full lg:w-80 bg-slate-800/50 border border-slate-700/50 rounded-xl p-6 flex flex-col h-full overflow-y-auto custom-scrollbar">
          <h2 className="text-xl font-bold text-white mb-6 flex items-center">
            <Clock size={20} className="mr-2 text-indigo-400" /> Backtest Config
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
              <label className="text-xs font-semibold text-slate-400 uppercase mb-2 block">Timerange</label>
              <div className="flex items-center space-x-2 bg-slate-900 border border-slate-700 rounded-lg px-3 py-2">
                <Calendar size={14} className="text-slate-500" />
                <input type="text" placeholder="20230101-20231231" className="bg-transparent text-sm text-white w-full outline-none placeholder-slate-600" />
              </div>
            </div>

            <div>
              <label className="text-xs font-semibold text-slate-400 uppercase mb-2 block">Timeframe</label>
              <div className="grid grid-cols-4 gap-2">
                {['1m', '5m', '15m', '1h', '4h', '1d'].map(tf => (
                  <button key={tf} className={`py-1.5 text-xs rounded border ${tf === '5m' ? 'bg-indigo-600 border-indigo-600 text-white' : 'bg-slate-900 border-slate-700 text-slate-400 hover:border-slate-500'}`}>
                    {tf}
                  </button>
                ))}
              </div>
            </div>

            <div>
              <label className="text-xs font-semibold text-slate-400 uppercase mb-2 block">Starting Balance</label>
              <div className="relative">
                <span className="absolute left-3 top-2.5 text-slate-500 text-sm">$</span>
                <input type="number" defaultValue={1000} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg pl-7 pr-4 py-2 text-sm focus:border-indigo-500 outline-none" />
              </div>
            </div>
          </div>

          <button
            onClick={runBacktest}
            disabled={isRunning}
            className={`mt-6 w-full py-3 rounded-lg font-bold text-white shadow-lg flex items-center justify-center transition-all ${isRunning ? 'bg-slate-700 cursor-not-allowed' : 'bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 shadow-indigo-900/20'}`}
          >
            {isRunning ? (
              <><Loader size={18} className="animate-spin mr-2" /> Running...</>
            ) : (
              <><Play size={18} className="mr-2 fill-current" /> Start Backtest</>
            )}
          </button>
        </div>

        {/* Results Area */}
        <div className="flex-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-6 flex flex-col min-h-[500px] relative overflow-hidden">
          {!hasResult && !isRunning && (
            <div className="absolute inset-0 flex flex-col items-center justify-center text-slate-500 opacity-40">
              <Activity size={64} className="mb-4" />
              <p className="text-lg">Ready to backtest</p>
            </div>
          )}

          {isRunning && (
            <div className="absolute inset-0 flex flex-col items-center justify-center bg-slate-900/50 backdrop-blur-sm z-10">
              <div className="w-64 bg-slate-700 rounded-full h-2 mb-4 overflow-hidden">
                <div className="bg-indigo-500 h-full transition-all duration-100 ease-out" style={{ width: `${progress}%` }}></div>
              </div>
              <p className="text-indigo-300 font-mono text-sm">Processing candles... {progress}%</p>
              <div className="mt-4 p-4 bg-black/40 rounded text-xs font-mono text-slate-400 w-96 h-32 overflow-hidden border border-slate-700/50">
                <div className="text-emerald-400">{'>'} Loading data...</div>
                <div className="text-slate-300">{'>'} Strategy init: {strategies[0]}</div>
                <div className="text-slate-500">{'>'} Warmup period: 100 candles</div>
                {progress > 30 && <div className="text-indigo-300">{'>'} Processing 2023/04...</div>}
                {progress > 60 && <div className="text-indigo-300">{'>'} Processing 2023/08...</div>}
                {progress > 90 && <div className="text-yellow-300">{'>'} Calculating metrics...</div>}
              </div>
            </div>
          )}

          {hasResult && (
            <div className="flex flex-col h-full animate-in fade-in slide-in-from-bottom-4 duration-500">
               {/* Result Header */}
               <div className="flex justify-between items-center mb-6">
                  <div>
                    <h3 className="text-xl font-bold text-white">Backtest Results</h3>
                    <p className="text-sm text-slate-400">{strategies[0]} • 5m • 2023</p>
                  </div>
                  <div className="flex space-x-2">
                    <button className="flex items-center space-x-1 px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-white text-xs rounded transition-colors">
                      <Save size={14} /> <span>Save Report</span>
                    </button>
                    <button onClick={() => setHasResult(false)} className="p-1.5 hover:bg-rose-500/20 hover:text-rose-400 text-slate-400 rounded transition-colors">
                      <Trash2 size={16} />
                    </button>
                  </div>
               </div>

               {/* Metrics Grid */}
               <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                 <div className="bg-slate-700/30 p-4 rounded-lg border border-emerald-500/20">
                   <p className="text-slate-400 text-xs uppercase font-semibold">Total Profit</p>
                   <p className="text-2xl font-bold text-emerald-400 font-mono">+45.2%</p>
                 </div>
                 <div className="bg-slate-700/30 p-4 rounded-lg border border-slate-600/30">
                   <p className="text-slate-400 text-xs uppercase font-semibold">Total Trades</p>
                   <p className="text-2xl font-bold text-white font-mono">342</p>
                 </div>
                 <div className="bg-slate-700/30 p-4 rounded-lg border border-slate-600/30">
                   <p className="text-slate-400 text-xs uppercase font-semibold">Win Rate</p>
                   <p className="text-2xl font-bold text-indigo-400 font-mono">62.5%</p>
                 </div>
                 <div className="bg-slate-700/30 p-4 rounded-lg border border-rose-500/20">
                   <p className="text-slate-400 text-xs uppercase font-semibold">Max Drawdown</p>
                   <p className="text-2xl font-bold text-rose-400 font-mono">-12.8%</p>
                 </div>
               </div>

               {/* Comparison Chart */}
               <div className="flex-1 min-h-0 bg-slate-900/50 rounded-lg p-4 border border-slate-700/30">
                 <ResponsiveContainer width="100%" height="100%">
                   <LineChart data={backtestResultsData}>
                      <CartesianGrid strokeDasharray="3 3" stroke="#334155" vertical={false} />
                      <XAxis dataKey="date" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} />
                      <YAxis stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} unit="%" />
                      <RechartsTooltip contentStyle={{ backgroundColor: '#1e293b', borderColor: '#334155', color: '#f8fafc' }} />
                      <Legend />
                      <Line type="monotone" name="Strategy" dataKey="strategy" stroke="#6366f1" strokeWidth={3} dot={{r: 4}} activeDot={{r: 6}} />
                      <Line type="monotone" name="Buy & Hold" dataKey="buyHold" stroke="#64748b" strokeWidth={2} strokeDasharray="5 5" dot={false} />
                   </LineChart>
                 </ResponsiveContainer>
               </div>
            </div>
          )}
        </div>
      </div>
    );
  };

  export default BacktestView;