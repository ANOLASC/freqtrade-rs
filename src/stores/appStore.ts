import { create } from 'zustand';
import type { Trade, BotStatus, DashboardStats, EquityPoint, AppConfig } from '../types';
import { getDashboardStats, getEquityCurve, getConfig, updateConfig, getOpenTrades } from '../services/api';

interface AppState {
  botState: BotStatus;
  trades: Trade[];
  activeTrades: Trade[];
  equityCurve: EquityPoint[];
  dashboardStats: DashboardStats | null;
  config: AppConfig | null;
  actions: {
    setBotState: (status: BotStatus) => void;
    setTrades: (trades: Trade[]) => void;
    fetchDashboardStats: () => Promise<void>;
    fetchEquityCurve: (timeframe?: string) => Promise<void>;
    fetchConfig: () => Promise<void>;
    updateConfig: (config: Partial<AppConfig>) => Promise<void>;
    fetchOpenTrades: () => Promise<void>;
  }
}

export const useAppStore = create<AppState>((set) => ({
  botState: 'stopped',
  trades: [],
  activeTrades: [],
  equityCurve: [],
  dashboardStats: null,
  config: null,
  actions: {
    setBotState: (status) => set({ botState: status }),
    setTrades: (trades) => set({ trades }),
    
    fetchDashboardStats: async () => {
      try {
        const stats = await getDashboardStats();
        set({ dashboardStats: stats });
      } catch (error) {
        console.error('Failed to fetch dashboard stats:', error);
      }
    },
    
    fetchEquityCurve: async (timeframe = '1d') => {
      try {
        const curve = await getEquityCurve(timeframe);
        set({ equityCurve: curve });
      } catch (error) {
        console.error('Failed to fetch equity curve:', error);
      }
    },
    
    fetchConfig: async () => {
      try {
        const config = await getConfig();
        set({ config });
      } catch (error) {
        console.error('Failed to fetch config:', error);
      }
    },
    
    updateConfig: async (config) => {
      try {
        await updateConfig(config);
        const newConfig = await getConfig();
        set({ config: newConfig });
      } catch (error) {
        console.error('Failed to update config:', error);
      }
    },
    
    fetchOpenTrades: async () => {
      try {
        const trades = await getOpenTrades();
        set({ activeTrades: trades });
      } catch (error) {
        console.error('Failed to fetch open trades:', error);
      }
    },
  }
}));
