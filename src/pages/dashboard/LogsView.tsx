import { useState, useEffect, useRef } from 'react';
import { Search, Terminal, ArrowRight } from 'lucide-react';

const fullLogsMock = Array.from({ length: 50 }).map((_, i) => {
    const type = i % 15 === 0 ? 'ERROR' : i % 5 === 0 ? 'WARNING' : 'INFO';
    return {
      id: i,
      timestamp: `2023-10-27 10:${String(i).padStart(2, '0')}:00`,
      level: type,
      component: i % 2 === 0 ? 'Strategy' : 'Exchange',
      message: type === 'ERROR'
        ? 'Connection timed out while fetching order book.'
        : type === 'WARNING'
        ? 'Signal ignored: Min ROI not met.'
        : `Successfully processed candle data for epoch ${i}.`
    };
  }).reverse();


const LogsView = () => {
    const [filter, setFilter] = useState('ALL');
    const [search, setSearch] = useState('');
    const [autoScroll, setAutoScroll] = useState(true);
    const scrollRef = useRef<HTMLDivElement>(null);

    const filteredLogs = fullLogsMock.filter(log => {
      const matchesFilter = filter === 'ALL' || log.level === filter;
      const matchesSearch = log.message.toLowerCase().includes(search.toLowerCase()) || log.component.toLowerCase().includes(search.toLowerCase());
      return matchesFilter && matchesSearch;
    });

    useEffect(() => {
      if (autoScroll && scrollRef.current) {
        scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
      }
    }, [filteredLogs, autoScroll]);

    return (
      <div className="h-[calc(100vh-140px)] flex flex-col bg-slate-950 border border-slate-800 rounded-xl overflow-hidden animate-in fade-in duration-500 shadow-2xl">
        {/* Logs Toolbar */}
        <div className="h-14 bg-slate-900 border-b border-slate-800 flex items-center justify-between px-4 shrink-0">
          <div className="flex items-center space-x-4">
             <div className="flex items-center space-x-2 text-slate-400">
               <Terminal size={18} />
               <span className="font-mono text-sm font-bold text-slate-200">system.log</span>
             </div>
             <div className="h-4 w-px bg-slate-700"></div>
             <div className="flex space-x-1 bg-slate-800 p-0.5 rounded-lg">
               {['ALL', 'INFO', 'WARNING', 'ERROR'].map(lvl => (
                 <button
                   key={lvl}
                   onClick={() => setFilter(lvl)}
                   className={`px-3 py-1 text-xs font-bold rounded-md transition-all ${
                     filter === lvl
                     ? lvl === 'ERROR' ? 'bg-rose-900/50 text-rose-400' : lvl === 'WARNING' ? 'bg-yellow-900/50 text-yellow-400' : 'bg-slate-600 text-white'
                     : 'text-slate-500 hover:text-slate-300'
                   }`}
                 >
                   {lvl}
                 </button>
               ))}
             </div>
          </div>

          <div className="flex items-center space-x-3">
            <div className="relative">
              <Search size={14} className="absolute left-3 top-2 text-slate-500" />
              <input
                type="text"
                placeholder="Search logs..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="bg-slate-800 border border-slate-700 text-slate-200 text-xs rounded-full pl-8 pr-4 py-1.5 focus:border-indigo-500 outline-none w-48 transition-all focus:w-64"
              />
            </div>
            <button
              onClick={() => setAutoScroll(!autoScroll)}
              className={`p-1.5 rounded-md transition-colors ${autoScroll ? 'bg-indigo-600 text-white' : 'text-slate-500 hover:bg-slate-800'}`}
              title="Auto-scroll"
            >
              <ArrowRight size={16} className="rotate-90" />
            </button>
          </div>
        </div>

        {/* Logs Content */}
        <div
          ref={scrollRef}
          className="flex-1 overflow-y-auto p-4 font-mono text-xs space-y-1 custom-scrollbar bg-[#0d1117]"
        >
          {filteredLogs.length > 0 ? filteredLogs.map((log) => (
            <div key={log.id} className="flex hover:bg-slate-800/50 px-2 py-0.5 rounded -mx-2 transition-colors">
              <span className="text-slate-500 w-36 shrink-0 select-none">{log.timestamp}</span>
              <span className={`w-20 shrink-0 font-bold ${
                log.level === 'ERROR' ? 'text-rose-500' : log.level === 'WARNING' ? 'text-yellow-500' : 'text-blue-400'
              }`}>
                {log.level}
              </span>
              <span className="text-slate-400 w-24 shrink-0 hidden sm:block">[{log.component}]</span>
              <span className={`flex-1 break-all ${
                 log.level === 'ERROR' ? 'text-rose-300' : log.level === 'WARNING' ? 'text-yellow-200' : 'text-slate-300'
              }`}>
                {log.message}
              </span>
            </div>
          )) : (
            <div className="flex flex-col items-center justify-center h-full text-slate-600">
              <Search size={32} className="mb-2 opacity-50" />
              <p>No logs found matching your criteria.</p>
            </div>
          )}
        </div>
      </div>
    );
  };

  export default LogsView;