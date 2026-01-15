import { useEffect, useState } from 'react';
import StatCard from '../../components/StatCard';
import TradeRow from '../../components/TradeRow';
import { DollarSign, Activity, Box, TrendingDown, RefreshCw } from 'lucide-react';
import { AreaChart, Area, ResponsiveContainer, CartesianGrid, XAxis, YAxis, Tooltip as RechartsTooltip } from 'recharts';
import { useAppStore } from '../../stores/appStore';
import type { Trade } from '../../types';

interface DashboardViewProps {
  onAnalyze: (trade: Trade) => void;
}

const DashboardView = ({ onAnalyze }: DashboardViewProps) => {
  const { activeTrades, dashboardStats, equityCurve, botState, actions } = useAppStore();
  const [timeRange, setTimeRange] = useState<'1d' | '1w'>('1d');
  
  // Load initial data
  useEffect(() => {
    actions.fetchDashboardStats();
    actions.fetchEquityCurve(timeRange);
    actions.fetchOpenTrades();
  }, []);
  
  // Update when timeRange changes
  useEffect(() => {
    actions.fetchEquityCurve(timeRange);
  }, [timeRange]);
  
  // Periodic refresh
  useEffect(() => {
    if (botState === 'running') {
      const interval = setInterval(() => {
        actions.fetchDashboardStats();
        actions.fetchEquityCurve(timeRange);
        actions.fetchOpenTrades();
      }, 30000); // Refresh every 30 seconds
      return () => clearInterval(interval);
    }
  }, [botState, timeRange]);
  
  // Mock logs (can be replaced with real log API later)
  const logs = [
    { time: '10:23:45', type: 'info', msg: 'Heartbeat: Freqtrade is running.' },
    { time: '10:24:12', type: 'buy', msg: 'Buy signal found: SOL/USDT.' },
    { time: '10:24:15', type: 'success', msg: 'Limit buy order for SOL/USDT created.' },
    { time: '10:28:00', type: 'info', msg: 'Checking for exit signals...' },
  ];
  
  if (!dashboardStats) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-slate-400">Loading dashboard...</div>
      </div>
    );
  }
  
  return (
    <div className="space-y-8 animate-in fade-in duration-500">
      {/* Stat Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard 
          title="Total Profit" 
          value={`+ ${dashboardStats.total_profit.toFixed(2)}%`} 
          subtext="Since start" 
          icon={DollarSign} 
          trend={dashboardStats.total_profit >= 0 ? 'up' : 'down'} 
        />
        <StatCard 
          title="Win Rate" 
          value={`${dashboardStats.win_rate.toFixed(1)}%`} 
          subtext="All trades" 
          icon={Activity} 
          trend={dashboardStats.win_rate >= 50 ? 'up' : 'down'} 
        />
        <StatCard 
          title="Open Trades" 
          value={dashboardStats.open_trades.toString()} 
          subtext={`Max: ${dashboardStats.open_trades}`} 
          icon={Box} 
          trend="neutral" 
        />
        <StatCard 
          title="Max Drawdown" 
          value={`-${dashboardStats.max_drawdown.toFixed(2)}%`} 
          subtext="All time" 
          icon={TrendingDown} 
          trend="down" 
        />
      </div>
      
      {/* Equity Curve and Logs */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 h-[400px]">
        <div className="lg:col-span-2 bg-slate-800/50 backdrop-blur-md border border-slate-700/50 rounded-xl p-6 flex flex-col">
          <div className="flex justify-between items-center mb-6">
            <div>
              <h3 className="text-lg font-bold text-white">Equity Curve</h3>
              <p className="text-xs text-slate-400">Real-time performance</p>
            </div>
            <div className="flex space-x-2">
              <button 
                onClick={() => setTimeRange('1d')}
                className={`px-3 py-1 text-xs rounded transition-all ${
                  timeRange === '1d' 
                  ? 'bg-slate-700 text-white' 
                  : 'bg-transparent text-slate-400 hover:text-white'
                }`}
              >
                1D
              </button>
              <button 
                onClick={() => setTimeRange('1w')}
                className={`px-3 py-1 text-xs rounded transition-all ${
                  timeRange === '1w' 
                  ? 'bg-slate-700 text-white' 
                  : 'bg-transparent text-slate-400 hover:text-white'
                }`}
              >
                1W
              </button>
            </div>
          </div>
          <div className="flex-1 w-full min-h-0">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={equityCurve}>
                <defs>
                  <linearGradient id="colorValue" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#6366f1" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#6366f1" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#334155" vertical={false} />
                <XAxis dataKey="time" stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} />
                <YAxis stroke="#64748b" fontSize={12} tickLine={false} axisLine={false} />
                <RechartsTooltip contentStyle={{ backgroundColor: '#1e293b', borderColor: '#334155', color: '#f8fafc' }} />
                <Area type="monotone" dataKey="value" stroke="#6366f1" strokeWidth={2} fillOpacity={1} fill="url(#colorValue)" />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
        
        <div className="bg-slate-800/50 backdrop-blur-md border border-slate-700/50 rounded-xl p-6 flex flex-col">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-bold text-white">Bot Logs</h3>
            <span className="px-2 py-0.5 rounded bg-slate-700 text-[10px] text-slate-300 font-mono">tail -f</span>
          </div>
          <div className="flex-1 overflow-y-auto space-y-3 pr-2 custom-scrollbar font-mono text-xs">
            {logs.map((log, idx) => (
              <div key={idx} className="flex items-start space-x-2 border-l-2 border-slate-700 pl-3 py-1">
                <span className="text-slate-500 whitespace-nowrap">{log.time}</span>
                <span className={`${
                  log.type === 'buy' ? 'text-cyan-400' : 
                  log.type === 'success' ? 'text-emerald-400' : 
                  'text-slate-300'
                }`}>{log.msg}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
      
      {/* Active Trades */}
      <div className="bg-slate-800/50 backdrop-blur-md border border-slate-700/50 rounded-xl overflow-hidden">
        <div className="p-6 border-b border-slate-700/50 flex justify-between items-center">
          <h3 className="text-lg font-bold text-white">Active Trades</h3>
          <div className="flex space-x-2">
            <button 
              onClick={() => actions.fetchOpenTrades()}
              className="p-2 text-slate-400 hover:text-indigo-400 transition-colors"
            >
              <RefreshCw size={18} />
            </button>
          </div>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="bg-slate-800/80 text-xs text-slate-400 uppercase tracking-wider">
                <th className="py-4 px-4 font-medium">Pair / Strategy</th>
                <th className="py-4 px-4 font-medium">Entry</th>
                <th className="py-4 px-4 font-medium">Current</th>
                <th className="py-4 px-4 font-medium">Profit</th>
                <th className="py-4 px-4 font-medium text-right">Actions</th>
              </tr>
            </thead>
            <tbody>
              {activeTrades.length > 0 ? (
                activeTrades.map((trade) => (
                  <TradeRow key={trade.id} trade={trade} onAnalyze={onAnalyze} />
                ))
              ) : (
                <tr>
                  <td colSpan={5} className="py-8 text-center text-slate-500">
                    No active trades
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default DashboardView;
