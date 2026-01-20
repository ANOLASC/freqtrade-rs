#!/usr/bin/env python3
"""
测试用例提取脚本
分析所有测试文件，提取测试函数名和行号
"""
import os
import re
from pathlib import Path
from typing import Dict, List, Tuple

TEST_DIR = "/d/code/trade/freqtrade/tests"

def find_all_test_files() -> List[Path]:
    """查找所有测试文件"""
    test_files = []
    for root, dirs, files in os.walk(TEST_DIR):
        # 排除某些目录
        dirs[:] = [d for d in dirs if d not in ['__pycache__', '.git', 'node_modules']]
        for file in files:
            if file.startswith('test_') and file.endswith('.py'):
                test_files.append(Path(root) / file)
    return sorted(test_files)

def extract_test_functions(file_path: Path) -> List[Tuple[str, int, str]]:
    """
    提取测试函数
    返回: [(函数名, 行号, 装饰器), ...]
    """
    tests = []
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # 匹配测试函数定义
    func_pattern = re.compile(r'^(\s*)def\s+(test_\w+)\s*\(', re.MULTILINE)
    
    # 匹配装饰器
    decorator_patterns = [
        re.compile(r'^(\s*)@pytest\.mark\.parametrize\([^)]+\)', re.MULTILINE),
        re.compile(r'^(\s*)@pytest\.mark\.(skip|skipif|xfail)\b', re.MULTILINE),
        re.compile(r'^(\s*)@pytest\.mark\.(longrun)', re.MULTILINE),
    ]
    
    for i, line in enumerate(lines, 1):
        # 检查是否是测试函数
        match = func_pattern.match(line)
        if match:
            func_name = match.group(2)
            indent = match.group(1)
            
            # 检查是否有skip标记
            is_skipped = False
            is_parametrized = False
            
            # 向前查找装饰器
            for j in range(max(0, i-5), i):
                prev_line = lines[j]
                if '@pytest.mark.skip' in prev_line or '@pytest.mark.skipif' in prev_line:
                    is_skipped = True
                    break
                if '@pytest.mark.parametrize' in prev_line:
                    is_parametrized = True
                    break
            
            # 确定优先级
            priority = "P0" if "persistence" in str(file_path) or "trade" in func_name.lower() else \
                      "P1" if "exchange" in str(file_path) or "strategy" in str(file_path) else "P2"
            
            # 确定模块
            module = determine_module(str(file_path))
            
            tests.append({
                'name': func_name,
                'line': i,
                'file': str(file_path).replace(TEST_DIR + '/', ''),
                'skipped': is_skipped,
                'parametrized': is_parametrized,
                'priority': priority,
                'module': module
            })
    
    return tests

def determine_module(file_path: str) -> str:
    """确定测试模块"""
    if 'persistence' in file_path:
        return 'persistence'
    elif 'exchange' in file_path and 'online' not in file_path:
        return 'exchange'
    elif 'strategy' in file_path:
        return 'strategy'
    elif 'freqtradebot' in file_path:
        return 'freqtradebot'
    elif 'optimize' in file_path:
        return 'optimize'
    elif 'leverage' in file_path:
        return 'leverage'
    elif 'data' in file_path:
        return 'data'
    elif 'rpc' in file_path:
        return 'rpc'
    elif 'freqai' in file_path:
        return 'freqai'
    elif 'util' in file_path:
        return 'util'
    else:
        return 'core'

def analyze_test_file(file_path: Path) -> Dict:
    """分析单个测试文件"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    lines = content.split('\n')
    
    result = {
        'file': str(file_path).replace(TEST_DIR + '/', ''),
        'total_lines': len(lines),
        'test_count': 0,
        'tests': [],
        'parametrized_count': 0,
        'skipped_count': 0,
        'classes': []
    }
    
    # 查找测试类
    class_pattern = re.compile(r'^class\s+(Test\w+)\s*\(', re.MULTILINE)
    for i, line in enumerate(lines, 1):
        match = class_pattern.match(line)
        if match:
            result['classes'].append({
                'name': match.group(1),
                'line': i
            })
    
    # 提取测试函数
    tests = extract_test_functions(file_path)
    result['tests'] = tests
    result['test_count'] = len(tests)
    result['parametrized_count'] = sum(1 for t in tests if t['parametrized'])
    result['skipped_count'] = sum(1 for t in tests if t['skipped'])
    
    return result

def main():
    """主函数"""
    test_files = find_all_test_files()
    
    print(f"找到 {len(test_files)} 个测试文件\n")
    
    all_results = []
    module_summary = {}
    
    for file_path in test_files:
        result = analyze_test_file(file_path)
        all_results.append(result)
        
        # 汇总模块信息
        module = result['tests'][0]['module'] if result['tests'] else 'unknown'
        if module not in module_summary:
            module_summary[module] = {'files': 0, 'tests': 0, 'total_lines': 0}
        module_summary[module]['files'] += 1
        module_summary[module]['tests'] += result['test_count']
        module_summary[module]['total_lines'] += result['total_lines']
    
    # 打印汇总
    print("=" * 80)
    print("模块汇总")
    print("=" * 80)
    for module, stats in sorted(module_summary.items(), key=lambda x: -x[1]['tests']):
        print(f"{module:20} | 文件: {stats['files']:3} | 测试: {stats['tests']:4} | 行数: {stats['total_lines']:6}")
    
    print("\n" + "=" * 80)
    print("详细测试列表 (按模块和优先级排序)")
    print("=" * 80)
    
    # 按模块和优先级排序
    sorted_results = sorted(all_results, key=lambda x: (
        ['persistence', 'exchange', 'strategy', 'freqtradebot', 'optimize', 'leverage', 'data', 'rpc', 'freqai', 'util', 'core'].index(x['tests'][0]['module']) if x['tests'] else 99,
        ['P0', 'P1', 'P2'].index(x['tests'][0]['priority']) if x['tests'] else 99
    ))
    
    for result in sorted_results:
        if not result['tests']:
            continue
            
        print(f"\n文件: {result['file']}")
        print(f"  总行数: {result['total_lines']} | 测试数: {result['test_count']} | 参数化: {result['parametrized_count']} | 跳过: {result['skipped_count']}")
        
        if result['classes']:
            print(f"  测试类: {', '.join(c['name'] for c in result['classes'])}")
        
        print("  测试函数:")
        for test in result['tests']:
            skip_mark = " [SKIP]" if test['skipped'] else ""
            param_mark = " [PARAM]" if test['parametrized'] else ""
            print(f"    Line {test['line']:4} | [{test['priority']}] | {test['name']}{skip_mark}{param_mark}")
    
    # 保存详细结果到JSON
    import json
    output = {
        'summary': {
            'total_files': len(test_files),
            'total_tests': sum(r['test_count'] for r in all_results),
            'total_lines': sum(r['total_lines'] for r in all_results),
            'module_summary': module_summary
        },
        'files': all_results
    }
    
    with open('/tmp/test_analysis.json', 'w', encoding='utf-8') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)
    
    print(f"\n详细结果已保存到 /tmp/test_analysis.json")

if __name__ == '__main__':
    main()
