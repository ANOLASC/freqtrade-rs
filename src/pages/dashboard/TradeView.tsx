import { useState } from 'react';
import { ComposedChart, Line, Bar, CartesianGrid, XAxis, YAxis, Tooltip as RechartsTooltip, ResponsiveContainer } from 'recharts';
import { Zap, Settings, Shield, LogOut, ChevronDown } from 'lucide-react';

const candleData = [
    { time: '10:00', open: 64000, high: 64500, low: 63900, close: 64200, volume: 120 },
    { time: '11:00', open: 64200, high: 64800, low: 64100, close: 64700, volume: 150 },
    { time: '12:00', open: 64700, high: 64900, low: 64500, close: 64600, volume: 100 },
    { time: '13:00', open: 64600, high: 65200, low: 64500, close: 65100, volume: 180 },
    { time: '14:00', open: 65100, high: 65500, low: 65000, close: 65300, volume: 200 },
  ];

const TradeView = () => {
    const [selectedPair] = useState('BTC/USDT');

    return (
      <div className="space-y-6 animate-in fade-in duration-500 h-[calc(100vh-140px)] flex flex-col">
        {/* Top Bar: Pair Select */}
        <div className="bg-slate-800/50 border border-slate-700/50 p-4 rounded-xl flex justify-between items-center">
          <div className="flex items-center space-x-4">
             <div className="relative">
               <button className="flex items-center space-x-2 bg-slate-900 border border-slate-700 px-4 py-2 rounded-lg text-white hover:border-indigo-500 transition-colors">
                 <span className="font-bold">{selectedPair}</span>
                 <ChevronDown size={14} className="text-slate-400"/>
               </button>
             </div>
             <div className="flex space-x-4 text-sm">
               <div className="flex flex-col">
                 <span className="text-slate-500 text-xs">Last Price</span>
                 <span className="text-emerald-400 font-mono">65,300.50</span>
               </div>
               <div className="flex flex-col">
                 <span className="text-slate-500 text-xs">24h Change</span>
                 <span className="text-emerald-400 font-mono">+2.45%</span>
               </div>
               <div className="flex flex-col">
                 <span className="text-slate-500 text-xs">24h Volume</span>
                 <span className="text-slate-300 font-mono">1.2B</span>
               </div>
             </div>
          </div>
          <div className="flex space-x-2">
             <button className="px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-xs rounded text-white transition-colors">15m</button>
             <button className="px-3 py-1.5 bg-indigo-600 hover:bg-indigo-500 text-xs rounded text-white transition-colors">1h</button>
             <button className="px-3 py-1.5 bg-slate-700 hover:bg-slate-600 text-xs rounded text-white transition-colors">4h</button>
          </div>
        </div>

        <div className="flex-1 grid grid-cols-1 lg:grid-cols-3 gap-6 min-h-0">
          {/* Left: Chart */}
          <div className="lg:col-span-2 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 flex flex-col">
             <div className="flex-1 w-full min-h-0">
               <ResponsiveContainer width="100%" height="100%">
                  <ComposedChart data={candleData}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#334155" vertical={false} />
                    <XAxis dataKey="time" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} />
                    <YAxis yAxisId="right" orientation="right" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} domain={['auto', 'auto']} />
                    <RechartsTooltip contentStyle={{ backgroundColor: '#1e293b', borderColor: '#334155', color: '#f8fafc' }} />
                    <Line yAxisId="right" type="monotone" dataKey="close" stroke="#22c55e" strokeWidth={2} dot={true} />
                    <Bar yAxisId="right" dataKey="volume" fill="#3b82f6" opacity={0.3} barSize={20} />
                  </ComposedChart>
               </ResponsiveContainer>
             </div>
          </div>

          {/* Right: Manual Controls */}
          <div className="bg-slate-800/50 border border-slate-700/50 rounded-xl p-6 flex flex-col space-y-6 overflow-y-auto custom-scrollbar">

             {/* Force Entry */}
             <div>
               <h3 className="text-white font-bold mb-4 flex items-center">
                 <Zap size={16} className="text-yellow-400 mr-2" /> Force Entry
               </h3>
               <div className="space-y-3">
                 <div className="flex space-x-2 bg-slate-900 p-1 rounded-lg border border-slate-700">
                   <button className="flex-1 py-1.5 bg-emerald-600 text-white rounded text-sm font-medium shadow-lg">Buy</button>
                   <button className="flex-1 py-1.5 text-slate-400 hover:text-white rounded text-sm font-medium transition-colors">Sell (Short)</button>
                 </div>
                 <div>
                   <label className="text-xs text-slate-400 mb-1 block">Order Type</label>
                   <select className="w-full bg-slate-900 border border-slate-700 text-slate-300 rounded p-2 text-sm">
                     <option>Limit</option>
                     <option>Market</option>
                   </select>
                 </div>
                 <div>
                   <label className="text-xs text-slate-400 mb-1 block">Price (USDT)</label>
                   <input type="number" defaultValue={65300} className="w-full bg-slate-900 border border-slate-700 text-white rounded p-2 text-sm font-mono" />
                 </div>
                 <button className="w-full bg-emerald-600 hover:bg-emerald-500 text-white py-2.5 rounded-lg font-medium transition-colors shadow-lg shadow-emerald-900/20">
                   Place Buy Order
                 </button>
               </div>
             </div>

             <div className="h-px bg-slate-700/50"></div>

             {/* Quick Actions */}
             <div>
                <h3 className="text-white font-bold mb-4 flex items-center">
                  <Settings size={16} className="text-slate-400 mr-2" /> Quick Actions
                </h3>
                <div className="grid grid-cols-2 gap-3">
                  <button className="flex flex-col items-center justify-center p-3 bg-slate-900 border border-slate-700 rounded-lg hover:border-slate-500 transition-all">
                    <Shield size={18} className="text-slate-400 mb-1" />
                    <span className="text-xs text-slate-300">Blacklist Pair</span>
                  </button>
                  <button className="flex flex-col items-center justify-center p-3 bg-slate-900 border border-slate-700 rounded-lg hover:border-slate-500 transition-all">
                    <LogOut size={18} className="text-rose-400 mb-1" />
                    <span className="text-xs text-slate-300">Force Exit All</span>
                  </button>
                </div>
             </div>

          </div>
        </div>
      </div>
    );
  };

  export default TradeView;