import { useState, useEffect } from 'react';
import { LucideIcon, Settings, Globe, Cpu, Save, CheckCircle } from 'lucide-react';
import { useAppStore } from '../../stores/appStore';
import type { AppConfig, BotConfig, ExchangeConfig, StrategyConfig } from '../../types';

interface NavButtonProps {
  id: string;
  icon: LucideIcon;
  label: string;
  active: string;
  onClick: (id: string) => void;
}

const SettingsView = () => {
  const { config, actions } = useAppStore();
  const [activeSection, setActiveSection] = useState('general');
  const [localConfig, setLocalConfig] = useState<Partial<AppConfig>>({});
  const [isSaved, setIsSaved] = useState(false);
  
  useEffect(() => {
    actions.fetchConfig();
  }, []);
  
  useEffect(() => {
    if (config) {
      setLocalConfig(config);
    }
  }, [config]);
  
  const handleSave = async () => {
    await actions.updateConfig(localConfig);
    setIsSaved(true);
    setTimeout(() => setIsSaved(false), 2000);
  };
  
  const updateField = (section: keyof AppConfig, field: string, value: any) => {
    setLocalConfig(prev => ({
      ...prev,
      [section]: {
        ...(prev[section] as any || {}),
        [field]: value
      }
    }));
  };
  
  if (!config) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-slate-400">Loading configuration...</div>
      </div>
    );
  }
  
  return (
    <div className="h-[calc(100vh-140px)] flex flex-col lg:flex-row gap-6 animate-in fade-in duration-500">
      <div className="w-full lg:w-64 bg-slate-800/50 border border-slate-700/50 rounded-xl p-4 flex flex-col h-full">
        <h2 className="text-xl font-bold text-white mb-6 px-2 flex items-center">
          <Settings size={20} className="mr-2 text-indigo-400" /> Settings
        </h2>
        <div className="space-y-1">
          <NavButton id="general" icon={Settings} label="General" active={activeSection} onClick={setActiveSection} />
          <NavButton id="exchange" icon={Globe} label="Exchange" active={activeSection} onClick={setActiveSection} />
          <NavButton id="strategy" icon={Cpu} label="Strategy" active={activeSection} onClick={setActiveSection} />
        </div>
      </div>
      
      <div className="flex-1 bg-slate-800/50 border border-slate-700/50 rounded-xl p-8 flex flex-col overflow-y-auto custom-scrollbar">
        <div className="flex-1 max-w-2xl">
          {activeSection === 'general' && <GeneralSettings config={localConfig.bot || config.bot} onChange={(field, value) => updateField('bot', field, value)} />}
          {activeSection === 'exchange' && <ExchangeSettings config={localConfig.exchange || config.exchange} onChange={(field, value) => updateField('exchange', field, value)} />}
          {activeSection === 'strategy' && <StrategySettings config={localConfig.strategy || config.strategy} onChange={(field, value) => updateField('strategy', field, value)} />}
        </div>
        
        <div className="mt-8 pt-6 border-t border-slate-700 flex justify-end">
          <button onClick={handleSave} className={`flex items-center space-x-2 px-6 py-2.5 rounded-lg font-medium transition-all duration-300 transform active:scale-95 ${isSaved ? 'bg-emerald-600 text-white' : 'bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg shadow-indigo-900/20'}`}>
            {isSaved ? <CheckCircle size={18} /> : <Save size={18} />}
            <span>{isSaved ? 'Saved!' : 'Save Changes'}</span>
          </button>
        </div>
      </div>
    </div>
  );
};

const NavButton = ({ id, icon: Icon, label, active, onClick }: NavButtonProps) => (
  <button onClick={() => onClick(id)} className={`w-full flex items-center space-x-3 px-4 py-3 rounded-lg transition-all ${active === id ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-500/20' : 'text-slate-400 hover:bg-slate-700 hover:text-slate-200'}`}>
    <Icon size={18} />
    <span className="font-medium text-sm">{label}</span>
  </button>
);

const GeneralSettings = ({ config, onChange }: { config: BotConfig; onChange: (field: string, value: any) => void }) => (
  <div className="space-y-6 animate-in fade-in slide-in-from-bottom-2 duration-300">
    <h3 className="text-lg font-bold text-white border-b border-slate-700 pb-2 mb-6">General Configuration</h3>
    <div className="flex items-center justify-between p-4 bg-slate-700/30 rounded-lg border border-slate-700">
      <div>
        <h4 className="text-sm font-medium text-white">Dry Run Mode</h4>
        <p className="text-xs text-slate-400">Simulate trading without using real funds</p>
      </div>
      <button onClick={() => onChange('dry_run', !config.dry_run)} className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${config.dry_run ? 'bg-indigo-600' : 'bg-slate-600'}`}>
        <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${config.dry_run ? 'translate-x-6' : 'translate-x-1'}`} />
      </button>
    </div>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">Max Open Trades</label>
      <input type="number" value={config.max_open_trades} onChange={(e) => onChange('max_open_trades', parseInt(e.target.value))} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none" />
    </div>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">Stake Currency</label>
      <select value={config.stake_currency} onChange={(e) => onChange('stake_currency', e.target.value)} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none">
        <option>USDT</option>
        <option>BUSD</option>
        <option>BTC</option>
        <option>ETH</option>
      </select>
    </div>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">Stake Amount</label>
      <input type="number" value={config.stake_amount} onChange={(e) => onChange('stake_amount', parseFloat(e.target.value))} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none" />
    </div>
  </div>
);

const ExchangeSettings = ({ config, onChange }: { config: ExchangeConfig; onChange: (field: string, value: any) => void }) => (
  <div className="space-y-6 animate-in fade-in slide-in-from-bottom-2 duration-300">
    <h3 className="text-lg font-bold text-white border-b border-slate-700 pb-2 mb-6">Exchange Setup</h3>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">Exchange Name</label>
      <select value={config.name} onChange={(e) => onChange('name', e.target.value)} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none">
        <option value="binance">Binance</option>
        <option value="kraken">Kraken</option>
        <option value="kucoin">KuCoin</option>
      </select>
    </div>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">API Key</label>
      <input type="password" value={config.key} onChange={(e) => onChange('key', e.target.value)} placeholder="•••••••••••••••••••••••" className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none font-mono" />
    </div>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">API Secret</label>
      <input type="password" value={config.secret} onChange={(e) => onChange('secret', e.target.value)} placeholder="••••••••••••••••••••••••" className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none font-mono" />
    </div>
  </div>
);

const StrategySettings = ({ config, onChange }: { config: StrategyConfig; onChange: (field: string, value: any) => void }) => (
  <div className="space-y-6 animate-in fade-in slide-in-from-bottom-2 duration-300">
    <h3 className="text-lg font-bold text-white border-b border-slate-700 pb-2 mb-6">Strategy Configuration</h3>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">Strategy Name</label>
      <select value={config.name} onChange={(e) => onChange('name', e.target.value)} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none">
        <option>SimpleStrategy</option>
        <option>NostalgiaForInfinity</option>
        <option>BinanceCluc</option>
      </select>
    </div>
    <div>
      <label className="block text-sm font-medium text-slate-400 mb-2">Timeframe</label>
      <select value={config.timeframe} onChange={(e) => onChange('timeframe', e.target.value)} className="w-full bg-slate-900 border border-slate-700 text-white rounded-lg px-4 py-2.5 focus:border-indigo-500 outline-none">
        <option value="1m">1 Minute</option>
        <option value="5m">5 Minutes</option>
        <option value="15m">15 Minutes</option>
        <option value="1h">1 Hour</option>
        <option value="4h">4 Hours</option>
        <option value="1d">1 Day</option>
      </select>
    </div>
  </div>
);

export default SettingsView;