import { useState, useEffect } from 'react';
import {
    Activity, Box, Settings, Play, Pause, Menu, X, Shield, TrendingUp, Clock, Cpu, Terminal
} from 'lucide-react';
import { BrowserRouter as Router, Routes, Route, Link, useLocation } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import SidebarItem from './components/SidebarItem';
import DashboardView from './pages/dashboard/DashboardView';
import TradeView from './pages/dashboard/TradeView';
import BacktestView from './pages/dashboard/BacktestView';
import HyperoptView from './pages/dashboard/HyperoptView';
import SettingsView from './pages/dashboard/SettingsView';
import LogsView from './pages/dashboard/LogsView';
import { useAppStore } from './stores/appStore';
import { Trade } from './types';

const MainLayout = () => {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const [botStatus, setBotStatus] = useState('stopped');
  const location = useLocation();
  const { actions } = useAppStore();
  
  useEffect(() => {
    actions.fetchConfig();
    actions.fetchDashboardStats();
  }, []);
  
  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const status = await invoke('get_bot_status');
        const statusMap: Record<string, string> = {
          'Running': 'running',
          'Stopped': 'stopped',
          'Paused': 'paused',
          'Error': 'error'
        };
        setBotStatus(statusMap[status as string] || 'stopped');
      } catch (error) {
        console.error('Failed to fetch bot status:', error);
      }
    }, 5000);
    return () => clearInterval(interval);
  }, []);
  
  const handleAnalyzeTrade = async (trade: Trade) => {
    alert('AI 分析功能开发中\n\n交易对: ' + trade.pair + '\n当前价格: ' + (trade.current || trade.close_rate || 'N/A'));
  };
  
  const handleDailyInsight = () => {
    alert('每日市场洞察功能开发中');
  };
  
  const handleBotToggle = async () => {
    try {
      if (botStatus === 'running') {
        await invoke('stop_bot');
        setBotStatus('stopped');
      } else {
        await invoke('start_bot');
        setBotStatus('running');
      }
    } catch (error) {
      console.error('Failed to toggle bot:', error);
      alert('操作失败: ' + error);
    }
  };
  
  return (
    <div className="flex h-screen bg-slate-900 text-slate-200 font-sans overflow-hidden selection:bg-indigo-500/30">
      <aside className={`${isSidebarOpen ? 'w-64' : 'w-20'} bg-slate-900 border-r border-slate-800 flex flex-col transition-all duration-300 z-20`}>
        <div className={`h-16 flex items-center border-b border-slate-800 transition-all duration-300 ${isSidebarOpen ? 'px-6' : 'justify-center px-0'}`}>
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 rounded bg-gradient-to-tr from-indigo-500 to-cyan-400 flex items-center justify-center shrink-0 shadow-lg shadow-indigo-500/30">
              <Activity size={18} className="text-white" />
            </div>
            {isSidebarOpen && <span className="font-bold text-lg tracking-tight text-white whitespace-nowrap animate-in fade-in duration-300">FreqUI</span>}
          </div>
        </div>

        <div className="flex-1 py-6 px-3 space-y-2 overflow-y-auto overflow-x-hidden">
          <Link to="/dashboard"><SidebarItem icon={Box} label="Dashboard" active={location.pathname.startsWith('/dashboard')} isSidebarOpen={isSidebarOpen} /></Link>
          <Link to="/trade"><SidebarItem icon={TrendingUp} label="Trade" active={location.pathname.startsWith('/trade')} isSidebarOpen={isSidebarOpen} /></Link>
          <Link to="/backtest"><SidebarItem icon={Clock} label="Backtesting" active={location.pathname.startsWith('/backtest')} isSidebarOpen={isSidebarOpen} /></Link>
          <Link to="/hyperopt"><SidebarItem icon={Cpu} label="Hyperopt" active={location.pathname.startsWith('/hyperopt')} isSidebarOpen={isSidebarOpen} /></Link>
          <div className="pt-4 pb-2">
            <div className={`h-px bg-slate-800 mb-4 ${isSidebarOpen ? 'mx-2' : 'mx-1'}`}></div>
            <p className={`px-4 text-xs font-semibold text-slate-500 uppercase tracking-wider mb-2 transition-opacity duration-200 ${!isSidebarOpen && 'hidden'}`}>Config</p>
          </div>
          <Link to="/settings"><SidebarItem icon={Settings} label="Settings" active={location.pathname.startsWith('/settings')} isSidebarOpen={isSidebarOpen} /></Link>
          <Link to="/logs"><SidebarItem icon={Terminal} label="Logs" active={location.pathname.startsWith('/logs')} isSidebarOpen={isSidebarOpen} /></Link>
        </div>

        <div className="p-4 border-t border-slate-800">
          <button onClick={handleBotToggle} className={`w-full flex items-center justify-center space-x-2 py-2.5 rounded-lg transition-all ${botStatus === 'running' ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 hover:bg-emerald-500/20' : 'bg-rose-500/10 text-rose-400 border border-rose-500/20 hover:bg-rose-500/20'}`}>
            {botStatus === 'running' ? <Pause size={18} className="shrink-0" /> : <Play size={18} className="shrink-0" />}
            {isSidebarOpen && <span className="whitespace-nowrap">{botStatus === 'running' ? 'Stop Bot' : 'Start Bot'}</span>}
          </button>
        </div>
      </aside>

      <main className="flex-1 flex flex-col h-full overflow-hidden bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-slate-800 via-slate-900 to-slate-900">
        <header className="h-16 border-b border-slate-800/50 flex items-center justify-between px-8 bg-slate-900/50 backdrop-blur-sm z-10 shrink-0">
          <div className="flex items-center space-x-4">
            <button onClick={() => setIsSidebarOpen(!isSidebarOpen)} className="p-2 text-slate-400 hover:text-white rounded-lg hover:bg-slate-800 transition-colors">
              <Menu size={20} />
            </button>
            <div className="h-4 w-px bg-slate-700"></div>
            <div className="flex items-center space-x-2 bg-slate-800/50 px-3 py-1.5 rounded-full border border-slate-700/50">
              <div className={`w-2 h-2 rounded-full ${botStatus === 'running' ? 'bg-emerald-400 animate-pulse' : 'bg-rose-400'}`}></div>
              <span className="text-xs font-mono text-slate-300">{botStatus.toUpperCase()}</span>
            </div>
            
            <button onClick={handleDailyInsight} className="flex items-center space-x-2 bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white px-4 py-1.5 rounded-full text-xs font-semibold shadow-lg shadow-indigo-900/20 transition-all transform hover:scale-105">
              <X size={12} className="text-yellow-200" />
              <span>Daily Insight</span>
              <span className="text-[10px] opacity-70">(开发中)</span>
            </button>
          </div>

          <div className="flex items-center space-x-6">
            <div className="hidden md:flex flex-col items-end">
              <span className="text-xs text-slate-500">Total Balance</span>
              <span className="text-sm font-bold text-white font-mono">2,450.50 USDT</span>
            </div>
            <div className="w-8 h-8 rounded-full bg-slate-700 flex items-center justify-center border border-slate-600 text-slate-300">
               <Shield size={16} />
            </div>
          </div>
        </header>

        <div className="flex-1 overflow-y-auto p-8 custom-scrollbar relative">
            <Routes>
                <Route path="/" element={<DashboardView onAnalyze={handleAnalyzeTrade} />} />
                <Route path="/dashboard" element={<DashboardView onAnalyze={handleAnalyzeTrade} />} />
                <Route path="/trade" element={<TradeView />} />
                <Route path="/backtest" element={<BacktestView />} />
                <Route path="/hyperopt" element={<HyperoptView />} />
                <Route path="/settings" element={<SettingsView />} />
                <Route path="/logs" element={<LogsView />} />
            </Routes>
        </div>

      </main>
      
      <style>{`
        .custom-scrollbar::-webkit-scrollbar { width: 6px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: rgba(30, 41, 59, 0.5); }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(71, 85, 105, 0.8); border-radius: 10px; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(99, 102, 241, 0.5); }
        .animate-in { animation: animateIn 0.2s ease-out; }
        @keyframes animateIn { from { opacity: 0; transform: scale(0.98); } to { opacity: 1; transform: scale(1); } }
      `}</style>
    </div>
  );
};

const App = () => {
    return (
        <Router>
            <MainLayout />
        </Router>
    )
}

export default App;