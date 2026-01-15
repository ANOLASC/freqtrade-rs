import { Loader, X, Sparkles } from 'lucide-react';

interface AIModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  content: string;
  isLoading: boolean;
}

const AIModal = ({ isOpen, onClose, title, content, isLoading }: AIModalProps) => {
    if (!isOpen) return null;
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center px-4">
        <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={onClose}></div>
        <div className="relative bg-slate-800 border border-slate-700 rounded-xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in zoom-in duration-200">
          <div className="bg-gradient-to-r from-indigo-900/50 to-slate-800 p-4 border-b border-slate-700 flex justify-between items-center">
            <div className="flex items-center space-x-2 text-indigo-300">
              <Sparkles size={18} />
              <h3 className="font-semibold">{title}</h3>
            </div>
            <button onClick={onClose} className="text-slate-400 hover:text-white transition-colors"><X size={18} /></button>
          </div>
          <div className="p-6 text-slate-300 leading-relaxed min-h-[120px]">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-8 space-y-4">
                <Loader size={32} className="animate-spin text-indigo-400" />
                <p className="text-sm text-slate-500 animate-pulse">Consulting Gemini AI...</p>
              </div>
            ) : (
              <div className="prose prose-invert prose-sm max-w-none"><p>{content}</p></div>
            )}
          </div>
          <div className="p-4 bg-slate-900/50 border-t border-slate-800/50 text-xs text-slate-500 flex justify-between items-center">
            <span>Powered by Google Gemini</span>
            {!isLoading && (
              <button onClick={onClose} className="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white rounded-lg transition-colors">Close</button>
            )}
          </div>
        </div>
      </div>
    );
  };

  export default AIModal;
