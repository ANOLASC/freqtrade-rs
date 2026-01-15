import { LucideIcon } from 'lucide-react';

interface SidebarItemProps {
  icon: LucideIcon;
  label: string;
  active: boolean;
  isSidebarOpen: boolean;
}

const SidebarItem = ({ icon: Icon, label, active, isSidebarOpen }: SidebarItemProps) => (
    <div
      className={`w-full flex items-center ${isSidebarOpen ? 'justify-start px-4 space-x-3' : 'justify-center px-2'} py-3 rounded-lg mb-1 transition-all duration-200 ${active ? 'bg-indigo-600 text-white shadow-lg shadow-indigo-500/20' : 'text-slate-400 hover:bg-slate-800 hover:text-slate-200'}`}
    >
      <Icon size={20} className="shrink-0" />
      {isSidebarOpen && <span className="font-medium text-sm whitespace-nowrap overflow-hidden transition-all duration-200">{label}</span>}
    </div>
  );

  export default SidebarItem;
