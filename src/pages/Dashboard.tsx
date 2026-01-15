import { useEffect, useState } from 'react';
import { api } from '../services/api';
import { useAppStore } from '../stores/appStore';
import { formatCurrency, formatPercentage, formatDate } from '../lib/utils';
import { Button } from '../ui/button';
import { Card, CardHeader, CardTitle, CardContent } from '../ui/card';
import type { BotStatus } from '../types';

export function Dashboard() {
  const { trades, actions: { setTrades } } = useAppStore();
  const [botStatus, setBotStatus] = useState<BotStatus>('stopped');
  const [loading, setLoading] = useState(false);

  const loadStatus = async () => {
    try {
      const status = await api.getBotStatus();
      setBotStatus(status);
      const allTrades = await api.getOpenTrades();
      setTrades(allTrades);
    } catch (error) {
      console.error('Failed to load status:', error);
    }
  };

  useEffect(() => {
    loadStatus();
    const interval = setInterval(loadStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const handleStart = async () => {
    setLoading(true);
    try {
      await api.startBot();
      await loadStatus();
    } catch (error) {
      console.error('Failed to start bot:', error);
      alert('Failed to start bot');
    } finally {
      setLoading(false);
    }
  };

  const handleStop = async () => {
    setLoading(true);
    try {
      await api.stopBot();
      await loadStatus();
    } catch (error) {
      console.error('Failed to stop bot:', error);
      alert('Failed to stop bot');
    } finally {
      setLoading(false);
    }
  };

  const getStatusColor = (status: BotStatus) => {
    switch (status) {
      case 'running':
        return 'bg-green-500';
      case 'paused':
        return 'bg-yellow-500';
      case 'error':
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  };

  const getStatusText = (status: BotStatus) => {
    switch (status) {
      case 'running':
        return '运行中';
      case 'paused':
        return '已暂停';
      case 'error':
        return '错误';
      default:
        return '已停止';
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-6">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* 头部状态栏 */}
        <div className="flex items-center justify-between">
          <h1 className="text-3xl font-bold text-gray-900">交易仪表板</h1>
          <div className="flex items-center gap-4">
            <div className={`flex items-center gap-2 px-4 py-2 rounded-full ${getStatusColor(botStatus)} text-white`}>
              <div className="w-2 h-2 rounded-full bg-white animate-pulse" />
              {getStatusText(botStatus)}
            </div>
            {botStatus === 'stopped' && (
              <Button onClick={handleStart} disabled={loading}>
                {loading ? '启动中...' : '启动机器人'}
              </Button>
            )}
            {botStatus === 'running' && (
              <Button variant="destructive" onClick={handleStop} disabled={loading}>
                {loading ? '停止中...' : '停止机器人'}
              </Button>
            )}
          </div>
        </div>

        {/* 统计卡片 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-gray-600">当前交易</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">{trades.length}</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-gray-600">账户余额</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">{formatCurrency(10000.0)}</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-gray-600">今日盈亏</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold text-green-500">+{formatCurrency(0.0)}</div>
              <div className="text-sm text-gray-500">0.00%</div>
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle className="text-sm font-medium text-gray-600">胜率</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="text-3xl font-bold">0.0%</div>
            </CardContent>
          </Card>
        </div>

        {/* 交易列表 */}
        <Card>
          <CardHeader>
            <CardTitle>当前交易</CardTitle>
          </CardHeader>
          <CardContent>
            {trades.length === 0 ? (
              <div className="text-center py-12 text-gray-500">
                暂无交易
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="border-b">
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">交易对</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">策略</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">开仓价</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">数量</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">本金</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">盈亏</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">操作</th>
                      <th className="text-left py-3 px-4 text-sm font-medium text-gray-600">开仓时间</th>
                    </tr>
                  </thead>
                  <tbody>
                    {trades.map((trade) => (
                      <tr key={trade.id} className="border-b hover:bg-gray-50">
                        <td className="py-3 px-4 font-medium">{trade.pair}</td>
                        <td className="py-3 px-4">{trade.strategy}</td>
                        <td className="py-3 px-4">{formatCurrency(trade.open_rate)}</td>
                        <td className="py-3 px-4">{trade.amount.toFixed(4)}</td>
                        <td className="py-3 px-4">{formatCurrency(trade.stake_amount)}</td>
                        <td className={`py-3 px-4 ${trade.profit_abs ? (trade.profit_abs > 0 ? 'text-green-500' : 'text-red-500') : ''}`}>
                          {trade.profit_abs ? formatPercentage(trade.profit_ratio || 0) : '-'}
                        </td>
                        <td className="py-3 px-4 text-sm text-gray-500">{formatDate(trade.open_date)}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}