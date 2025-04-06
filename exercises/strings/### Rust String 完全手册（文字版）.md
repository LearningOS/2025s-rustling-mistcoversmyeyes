### 🤖 Assistant

### Rust String 完全手册（文字版）

---

#### 一、基础概念
```rust
// String 是 UTF-8 编码的可变字符串
let mut s = String::new();       // 空字符串
s.push_str("hello");            // 追加内容
let s = "world".to_string();    // 字面量转换
```

#### 二、核心操作分类

##### 1. 创建与初始化
| 方法                 | 作用                          | 示例                        | 内存影响      |
|----------------------|-------------------------------|----------------------------|--------------|
| `new()`              | 创建空String                  | `String::new()`            | 初始容量0     |
| `with_capacity(n)`   | 预分配字节容量                | `String::with_capacity(10)`| 减少重分配    |
| `to_string()`        | &str转String                  | `"hi".to_string()`         | 堆分配        |

##### 2. 内容修改
| 方法                 | 作用                          | 时间复杂度  |
|----------------------|-------------------------------|------------|
| `push_str(&str)`     | 追加字符串                    | O(n)       |
| `push(char)`         | 追加字符                      | O(1) 分摊  |
| `insert(idx, char)`  | 插入字符                      | O(n)       |
| `replace(range, &str)` | 替换指定范围内容             | O(n)       |

##### 3. 内容访问
| 方法                 | 返回类型          | 越界处理          |
|----------------------|-------------------|-------------------|
| `len()`              | usize             | UTF-8字节数       |
| `chars()`            | 字符迭代器        | Unicode标量值     |
| `as_bytes()`         | &[u8]             | 原始字节访问      |
| `get(..n)`           | Option<&str>      | 安全子串访问      |

##### 4. 内存管理
```rust
s.reserve(20);          // 确保至少20字节容量
s.shrink_to_fit();      // 释放多余容量
s.truncate(3);          // 截断到前3个字节
```

---

#### 三、特殊用例模式

##### 1. 字符串拼接
```rust
// 高效拼接（避免临时分配）
let mut s = String::with_capacity(20);
s.push_str("Hello");
s += " world!";         // 等效于 push_str
```

##### 2. 类型转换
```rust
// String → &str（零成本）
let slice: &str = &s;

// 与其他类型互转
let path = Path::new(&s);          // 转路径
let c_str = CString::new(s).unwrap(); // 转C字符串
```

##### 3. UTF-8处理
```rust
// 遍历Unicode字符
for c in "नमस्ते".chars() {
    println!("{}", c);  // न, म, स, ्, त, े
}

// 统计字素簇（需要第三方库）
use unicode_segmentation::UnicodeSegmentation;
let count = "नमस्ते".graphemes(true).count(); // 4
```

---

#### 四、性能特性

##### 内存分配策略
- 默认初始容量：0字节
- 增长策略：通常翻倍扩容
- 容量单位：字节（非字符数）

##### 操作复杂度
| 操作               | 平均复杂度 | 最坏复杂度 |
|--------------------|------------|------------|
| push/pop           | O(1)*      | O(n)       |
| 索引访问           | O(1)       | O(1)       |
| 字符串拼接         | O(n)       | O(n)       |

*分摊时间复杂度

---

#### 五、所有权与借用

##### 1. 移动语义
```rust
let s1 = String::from("hello");
let s2 = s1;            // 所有权转移
// println!("{}", s1);  // 编译错误
```

##### 2. 借用规则
```rust
let mut s = String::from("hello");
let slice = &s[0..2];   // 不可变借用
// s.push_str("world"); // 编译错误！存在活跃借用
```

---

### Rust String 完全手册（图示版）

---

#### 一、内存布局
```text
栈内存：
+---------+---------+---------+
| pointer | capacity | length  |
| 0x5678  |    10    |    5    |  → "hello"
+---------+---------+---------+

堆内存（0x5678）：
+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
|  h  |  e  |  l  |  l  |  o  |  \0 | ??? | ??? | ??? | ??? |
+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+
         已使用字节             容量剩余
```

#### 二、拼接操作
```text
初始：s = "tea"
操作：s.push_str("cher")

内存变化：
+---+---+---+---+---+---+---+---+---+---+---+---+---+
| t | e | a | c | h | e | r |   |   |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+---+---+---+
```

#### 三、切片操作
```text
原始字符串： "🦀Rust⚙️"
字节索引： 0-3 4-7 8-11 12-16
           🦀   R   u    st⚙️

安全切片 &s[0..4] → "🦀"
危险切片 &s[0..1] → panic（无效UTF-8）
```

#### 四、所有权转移
```text
s1: [ptr: 0x1234, cap: 5, len:5] → "hello"
s2 = s1 转移后：
s1: [ptr: null, cap:0, len:0]
s2: [ptr: 0x1234, cap:5, len:5]
```

#### 五、容量增长
```text
初始：cap=3, len=3 → "abc"
push('d')触发扩容：
新容量=6，复制内容：
+---+---+---+---+---+---+
| a | b | c | d |   |   |
+---+---+---+---+---+---+
```

---

官方参考：
- [std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)
- [Rust UTF-8 处理指南](https://doc.rust-lang.org/rust-by-example/std/str.html)

（迭代器相关方法如 `chars()`, `split()` 等详见迭代器手册）