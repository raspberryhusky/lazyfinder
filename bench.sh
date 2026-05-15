#!/bin/bash
echo "=== 性能与体积优化对比 ==="

# 原始版本测试 (如果有的话)
if [ -f "target/release/lazyfinder_old" ]; then
    echo "[优化前] 文件体积:"
    ls -lh target/release/lazyfinder_old | awk '{print $5}'
    echo "[优化前] 搜索性能:"
    time ./target/release/lazyfinder_old -d . -p "rs" -k "fs::read" > /dev/null
fi

echo "-------------------"
echo "[优化后] 文件体积:"
ls -lh target/release/lazyfinder | awk '{print $5}'
echo "[优化后] 搜索性能:"
time ./target/release/lazyfinder -d . -p "rs" -k "fs::read" > /dev/null

echo "=== 扩写功能测试 ==="
./target/release/lazyfinder -d src/ -p "rs" -k "FileMatcher" -c 15
