import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ProtectionConfig {
  name: string;
  enabled: boolean;
  config: any;
}

export default function Protections() {
  const [protections, setProtections] = useState<ProtectionConfig[]>([]);
  const [loading, setLoading] = useState(true);
  const [showAddModal, setShowAddModal] = useState(false);
  const [selectedProtection, setSelectedProtection] = useState<string>('');
  const [newProtectionConfig, setNewProtectionConfig] = useState<any>({});

  useEffect(() => {
    loadProtections();
  }, []);

  const loadProtections = async () => {
    try {
      const result = await invoke<string[]>('list_protections');
      const protectionConfigs: ProtectionConfig[] = result.map(name => ({
        name,
        enabled: true,
        config: {}
      }));
      setProtections(protectionConfigs);
    } catch (error) {
      console.error('Failed to load protections:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleAddProtection = async () => {
    try {
      switch (selectedProtection) {
        case 'cooldown':
          await invoke('add_cooldown_protection', { config: newProtectionConfig });
          break;
        case 'low_profit':
          await invoke('add_low_profit_protection', { config: newProtectionConfig });
          break;
        case 'max_drawdown':
          await invoke('add_max_drawdown_protection', { config: newProtectionConfig });
          break;
        case 'stoploss_guard':
          await invoke('add_stoploss_guard', { config: newProtectionConfig });
          break;
      }
      await loadProtections();
      setShowAddModal(false);
      alert('Protection added successfully!');
    } catch (error) {
      console.error('Failed to add protection:', error);
      alert('Failed to add protection: ' + error);
    }
  };

  const handleRemoveProtection = async (name: string) => {
    if (!confirm('Are you sure you want to remove ' + name + '?')) {
      return;
    }
    try {
      await invoke('remove_protection', { name });
      await loadProtections();
      alert('Protection removed successfully!');
    } catch (error) {
      console.error('Failed to remove protection:', error);
      alert('Failed to remove protection: ' + error);
    }
  };

  const handleCheckGlobalStop = async () => {
    try {
      const stopReason = await invoke<any>('check_global_stop');
      if (stopReason) {
        alert('Global stop triggered!\\n\\nReason: ' + stopReason.reason + '\\nUntil: ' + stopReason.until + '\\nProtection: ' + stopReason.protection);
      } else {
        alert('No global stop active');
      }
    } catch (error) {
      console.error('Failed to check global stop:', error);
      alert('Failed to check global stop: ' + error);
    }
  };

  const handleCheckPairStop = async () => {
    const pair = prompt('Enter pair to check (e.g., BTCUSDT):');
    if (!pair) return;
    
    try {
      const stopReason = await invoke<any>('check_pair_stop', { pair });
      if (stopReason) {
        alert('Pair stop triggered for ' + pair + '!\\n\\nReason: ' + stopReason.reason + '\\nUntil: ' + stopReason.until + '\\nProtection: ' + stopReason.protection);
      } else {
        alert('No pair stop active for ' + pair);
      }
    } catch (error) {
      console.error('Failed to check pair stop:', error);
      alert('Failed to check pair stop: ' + error);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-xl">Loading...</div>
      </div>
    );
  }

  return (
    <div className="container mx-auto p-6">
      <h1 className="text-3xl font-bold mb-6">风险管理 - 保护机制</h1>
      
      <div className="flex gap-4 mb-6">
        <button
          onClick={() => setShowAddModal(true)}
          className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded"
        >
          添加保护机制
        </button>
        <button
          onClick={handleCheckGlobalStop}
          className="bg-yellow-500 hover:bg-yellow-600 text-white px-4 py-2 rounded"
        >
          检查全局停止
        </button>
        <button
          onClick={handleCheckPairStop}
          className="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded"
        >
          检查单对停止
        </button>
      </div>

      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">保护名称</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {protections.length === 0 ? (
              <tr>
                <td colSpan={3} className="px-6 py-4 text-center text-gray-500">
                  没有配置的保护机制
                </td>
              </tr>
            ) : (
              protections.map((protection) => (
                <tr key={protection.name}>
                  <td className="px-6 py-4 whitespace-nowrap">{protection.name}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={'px-2 inline-flex text-xs leading-5 font-semibold rounded-full ' + (protection.enabled ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800')}>
                      {protection.enabled ? '已启用' : '已禁用'}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <button
                      onClick={() => handleRemoveProtection(protection.name)}
                      className="text-red-600 hover:text-red-900"
                    >
                      移除
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {showAddModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <h2 className="text-xl font-bold mb-4">添加保护机制</h2>
            
            <div className="mb-4">
              <label className="block text-sm font-medium text-gray-700 mb-2">保护类型</label>
              <select
                value={selectedProtection}
                onChange={(e) => setSelectedProtection(e.target.value)}
                className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="">选择保护类型...</option>
                <option value="cooldown">冷却期保护</option>
                <option value="low_profit">低利润对保护</option>
                <option value="max_drawdown">最大回撤保护</option>
                <option value="stoploss_guard">止损保护</option>
              </select>
            </div>

            {selectedProtection === 'cooldown' && (
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">停止持续时间（分钟）</label>
                  <input
                    type="number"
                    defaultValue={60}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, stop_duration: parseInt(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">回顾周期（分钟）</label>
                  <input
                    type="number"
                    defaultValue={1000}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, lookback_period: parseInt(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
              </div>
            )}

            {selectedProtection === 'low_profit' && (
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">最小利润率（%）</label>
                  <input
                    type="number"
                    defaultValue={0.05}
                    step={0.01}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, min_profit: parseFloat(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">回顾周期（分钟）</label>
                  <input
                    type="number"
                    defaultValue={1000}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, lookback_period: parseInt(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
              </div>
            )}

            {selectedProtection === 'max_drawdown' && (
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">最大回撤（%）</label>
                  <input
                    type="number"
                    defaultValue={0.2}
                    step={0.01}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, max_drawdown: parseFloat(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">回顾周期（分钟）</label>
                  <input
                    type="number"
                    defaultValue={1440}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, lookback_period: parseInt(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
              </div>
            )}

            {selectedProtection === 'stoploss_guard' && (
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">最大止损触发次数</label>
                  <input
                    type="number"
                    defaultValue={4}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, max_stoploss_count: parseInt(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">回顾周期（分钟）</label>
                  <input
                    type="number"
                    defaultValue={1000}
                    onChange={(e) => setNewProtectionConfig({ ...newProtectionConfig, lookback_period: parseInt(e.target.value) })}
                    className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  />
                </div>
              </div>
            )}

            <div className="flex gap-4 mt-6">
              <button
                onClick={() => setShowAddModal(false)}
                className="flex-1 bg-gray-300 hover:bg-gray-400 text-gray-800 px-4 py-2 rounded"
              >
                取消
              </button>
              <button
                onClick={handleAddProtection}
                disabled={!selectedProtection}
                className="flex-1 bg-blue-500 hover:bg-blue-600 disabled:bg-gray-300 text-white px-4 py-2 rounded"
              >
                添加
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
