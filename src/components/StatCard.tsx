import { TrendingUp, TrendingDown, LucideIcon } from 'lucide-react';

interface StatCardProps {
  title: string;
  value: string;
  subtext: string;
  icon: LucideIcon;
  trend: 'up' | 'down' | 'neutral';
}

const StatCard = ({ title, value, subtext, icon: Icon, trend }: StatCardProps) => (
  <div className="bg-slate-800/50 backdrop-blur-md border border-slate-700/50 p-6 rounded-xl hover:border-slate-600 transition-all duration-300 group">
    <div className="flex justify-between items-start mb-4">
      <div>
        <p className="text-slate-400 text-sm font-medium mb-1">{title}</p>
        <h3 className="text-2xl font-bold text-white tracking-tight">{value}</h3>
      </div>
      <div className={`p-3 rounded-lg ${trend === 'up' ? 'bg-emerald-500/10 text-emerald-400' : trend === 'down' ? 'bg-rose-500/10 text-rose-400' : 'bg-blue-500/10 text-blue-400'}`}>
        <Icon size={20} />
      </div>
    </div>
    <div className="flex items-center text-xs">
      {trend === 'up' && <TrendingUp size={14} className="text-emerald-400 mr-1" />}
      {trend === 'down' && <TrendingDown size={14} className="text-rose-400 mr-1" />}
      <span className={trend === 'up' ? 'text-emerald-400' : trend === 'down' ? 'text-rose-400' : 'text-slate-400'}>
        {subtext}
      </span>
      {subtext && <span className="text-slate-500 ml-2">vs 24h ago</span>}
    </div>
  </div>
);

export default StatCard;
