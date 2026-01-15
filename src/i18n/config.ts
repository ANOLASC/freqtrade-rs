import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

const resources = {
  en: {
    translation: {
      // Sidebar
      dashboard: 'Dashboard',
      trade: 'Trade',
      backtesting: 'Backtesting',
      hyperopt: 'Hyperopt',
      settings: 'Settings',
      logs: 'Logs',
      startBot: 'Start Bot',
      stopBot: 'Stop Bot',
      config: 'Config',
      
      // Dashboard
      totalProfit: 'Total Profit',
      winRate: 'Win Rate',
      openTrades: 'Open Trades',
      maxDrawdown: 'Max Drawdown',
      sinceStart: 'Since start',
      allTrades: 'All trades',
      equityCurve: 'Equity Curve',
      realTimePerformance: 'Real-time performance',
      botLogs: 'Bot Logs',
      activeTrades: 'Active Trades',
      refresh: 'Refresh',
      noActiveTrades: 'No active trades',
      pairStrategy: 'Pair / Strategy',
      entry: 'Entry',
      current: 'Current',
      profit: 'Profit',
      actions: 'Actions',
      
      // Settings
      general: 'General',
      exchange: 'Exchange',
      strategy: 'Strategy',
      generalConfiguration: 'General Configuration',
      dryRunMode: 'Dry Run Mode',
      dryRunDescription: 'Simulate trading without using real funds',
      maxOpenTrades: 'Max Open Trades',
      stakeCurrency: 'Stake Currency',
      stakeAmount: 'Stake Amount',
      exchangeSetup: 'Exchange Setup',
      exchangeName: 'Exchange Name',
      apiKey: 'API Key',
      apiSecret: 'API Secret',
      strategyConfiguration: 'Strategy Configuration',
      strategyName: 'Strategy Name',
      timeframe: 'Timeframe',
      saveChanges: 'Save Changes',
      saved: 'Saved!',
      language: 'Language',
      
      // Other
      loadingDashboard: 'Loading dashboard...',
      loadingConfiguration: 'Loading configuration...',
      dailyInsight: 'Daily Insight',
      inDevelopment: '(In Development)',
      totalBalance: 'Total Balance',
      dryRun: 'DRY RUN',
    }
  },
  zh: {
    translation: {
      // Sidebar
      dashboard: '仪表盘',
      trade: '交易',
      backtesting: '回测',
      hyperopt: '超参优化',
      settings: '设置',
      logs: '日志',
      startBot: '启动机器人',
      stopBot: '停止机器人',
      config: '配置',
      
      // Dashboard
      totalProfit: '总收益',
      winRate: '胜率',
      openTrades: '开仓数',
      maxDrawdown: '最大回撤',
      sinceStart: '开始至今',
      allTrades: '所有交易',
      equityCurve: '资金曲线',
      realTimePerformance: '实时表现',
      botLogs: '机器人日志',
      activeTrades: '活跃交易',
      refresh: '刷新',
      noActiveTrades: '暂无活跃交易',
      pairStrategy: '交易对 / 策略',
      entry: '入场价',
      current: '当前价',
      profit: '收益率',
      actions: '操作',
      
      // Settings
      general: '常规',
      exchange: '交易所',
      strategy: '策略',
      generalConfiguration: '常规配置',
      dryRunMode: '模拟交易模式',
      dryRunDescription: '在不使用真实资金的情况下模拟交易',
      maxOpenTrades: '最大开仓数',
      stakeCurrency: '基础货币',
      stakeAmount: '每单投入',
      exchangeSetup: '交易所设置',
      exchangeName: '交易所名称',
      apiKey: 'API 密钥',
      apiSecret: 'API 密钥',
      strategyConfiguration: '策略配置',
      strategyName: '策略名称',
      timeframe: '时间周期',
      saveChanges: '保存更改',
      saved: '已保存！',
      language: '语言',
      
      // Other
      loadingDashboard: '加载仪表盘...',
      loadingConfiguration: '加载配置...',
      dailyInsight: '每日洞察',
      inDevelopment: '（开发中）',
      totalBalance: '总余额',
      dryRun: '模拟运行',
    }
  }
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: 'zh', // 默认中文
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false
    },
    react: {
      useSuspense: false
    }
  });

export default i18n;
