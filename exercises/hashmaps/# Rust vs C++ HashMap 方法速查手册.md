### 🤖 Assistant

# Rust vs C++ HashMap 方法速查手册

结合你完成的 Rustlings 习题，以下是关键方法的分类对比：

## 一、基础操作类

| 操作         | C++ (std::unordered_map)       | Rust (std::collections::HashMap) | 对应习题 |
|--------------|---------------------------------|-----------------------------------|----------|
| **创建空map** | `std::unordered_map<K, V> map;` | `HashMap::new()`                 | iter5    |
| **插入元素**  | `insert({key, value})`          | `insert(key, value)`             | iter5    |
| **获取元素**  | `at(key)`                       | `get(&key)` → `Option<&V>`       | iter5    |
| **删除元素**  | `erase(key)`                    | `remove(&key)` → `Option<V>`     | -        |

## 二、迭代器相关

| 操作                | C++ 风格                          | Rust 迭代器链                          | 对应习题 |
|---------------------|-----------------------------------|----------------------------------------|----------|
| **遍历所有键值对**  | `for (const auto& [k, v] : map)`  | `for (k, v) in &map`                  | iter1    |
| **仅遍历值**        | 手动获取second值                  | `map.values()` → `Iterator<Item=&V>`  | iter5    |
| **过滤元素**        | 手动if判断                        | `.filter(|(k, v)| condition)`         | iter5    |
| **元素转换**        | 创建新容器手动转换                | `.map(|(k, v)| transform)`            | iter2    |
| **嵌套结构处理**    | 多重嵌套循环                      | `.flat_map(|inner| inner.iter())`     | iter5    |

## 三、特殊方法对比

### 1. 存在性检查
```cpp
// C++
if (map.contains(key)) { ... }
```
```rust
// Rust (来自习题iter5)
if map.contains_key(&key) { ... }  // 固有方法
```

### 2. 条目处理 (Entry API)
```cpp
// C++ (需手动检查)
auto it = map.find(key);
if (it == map.end()) {
    map.insert({key, default});
}
```
```rust
// Rust (来自标准库)
map.entry(key).or_insert(default);  // 固有方法
```

### 3. 容量管理
| 操作              | C++                  | Rust                    |
|-------------------|----------------------|-------------------------|
| 预分配空间        | `reserve(n)`         | `reserve(n)` (固有方法) |
| 缩减容量          | `shrink_to_fit()`    | `shrink_to_fit()`       |

## 四、习题方法映射表

### iterators2
```rust
// 字符串处理
vec!["a", "b"].iter().map(|s| capitalize_first(s)).collect()
```
对应 C++ 实现：
```cpp
std::vector<std::string> result;
for (const auto& s : vec) {
    result.push_back(capitalize_first(s));
}
```

### iterators3
```rust
// 错误处理模式
numbers.into_iter().map(|n| divide(n, 27)).collect::<Result<Vec<_>, _>>()
```
C++ 无直接对应，需手动创建结果容器：
```cpp
std::vector<Result<int>> results;
for (auto n : numbers) {
    results.push_back(divide(n, 27));
}
```

### iterators4
```rust
// 阶乘实现
(1..=num).product()  // 使用Iterator的product方法
```
对应 C++ 实现：
```cpp
int product = 1;
for (int i = 1; i <= num; ++i) {
    product *= i;
}
```

## 五、性能与安全注意事项

1. **迭代器失效问题**：
   - C++：修改容器会使其迭代器失效
   - Rust：编译器通过借用检查阻止危险操作

2. **内存预分配**：
   ```rust
   let mut map = HashMap::with_capacity(100); // 类似C++的reserve
   ```

3. **查找性能**：
   | 操作          | C++ 复杂度       | Rust 复杂度       |
   |---------------|-----------------|-------------------|
   | 插入/查找     | 平均O(1)        | 平均O(1)          |
   | 遍历          | O(n)            | O(n)              |

## 六、Rust 特有方法速查

### 来自习题的关键方法
```rust
// iterators5 核心方法
collection.iter()
    .flat_map(|map| map.values())  // 展平嵌套结构
    .filter(|&p| *p == target)     // 过滤目标值
    .count()                       // 计数
```

### 实用方法扩展
| 方法                 | 作用                           | C++近似实现             |
|----------------------|--------------------------------|-------------------------|
| `entry(key)`         | 安全地处理存在/不存在的键       | `find` + `insert`       |
| `retain(|k, v| cond)`| 就地过滤元素                   | 手动创建新容器           |
| `drain()`            | 消费迭代并清空容器             | `std::move_iterator`    |

建议打印此表放在手边，遇到方法选择困难时可快速对照。需要我解释某个方法的底层实现细节吗？