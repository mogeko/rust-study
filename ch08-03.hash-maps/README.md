# 哈希 map 储存键值对

[教程原文](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html)

类似于 vector，哈希 map 是同质的：**所有的键必须是相同类型，值也必须都是相同类型。**

# 新建哈希 map

使用 `new` 方法创建，使用 `insert` 方法插入数据：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
let team_name = String::from("Blue");

scores.insert(field_name, 10);
// 这里 field_name 将所有权移交给了 scores，不再有效
scores.insert(String::from("Yellow"), 50);
```

或者使用包含元组的 vector 的 `collect` 方法：

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
// 分解一下
// teams.iter() -> ["Blue", "Yellow"]
// initial_scores.iter() -> [10,50]
// teams.iter().zip(initial_scores.iter()) -> [("Blue", 10), ("Yellow", 50)]
// teams.iter().zip(initial_scores.iter()).collect() -> {"Blue": 10, "Yellow": 50}

```


如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。

[关于生命周期的更多细节](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)

## 访问哈希 map 中的值

可以通过 `get` 方法并提供对应的键来从哈希 map 中获取值，返回一个 `Option`：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

也可以使用 `for` 循环遍历哈希 map：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

这会以任意顺序打印出每一个键值对：

```shell
Yellow: 50
Blue: 10
```

## 更新哈希 map

更新值的三种策略：

- 完全无视旧值并用新值代替旧值
- 保留旧值而忽略新值，并只在键**没有**对应值时增加新值
- 结合新旧两值 (根据旧值更新一个值)

### 覆盖一个值

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10); // 旧值
scores.insert(String::from("Blue"), 25); // 新值
```

### 只在键不存在时插入

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
```

`entry` 函数的返回值是一个枚举：`Entry`，它代表了可能存在也可能不存在的值。

`Entry` 的 `or_insert` 方法在键对应的值存在时就返回这个值的 `Entry`，如果不存在则将参数作为新值插入并返回修改过的 `Entry`。

### 根据旧值更新一个值

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

`or_insert` 方法事实上会返回这个键的值的一个可变引用 (`&mut V`)。这里我们将这个可变引用储存在 `count` 变量中，所以为了赋值必须首先使用星号 (`*`) 解引用 `count`。

## 哈希函数

`HashMap` 默认使用一种[“密码学安全的” (“cryptographically strong” )](https://www.131002.net/siphash/siphash.pdf)哈希函数，它可以抵抗拒绝服务 (Denial of Service, DoS)攻击。然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 *hasher* 来切换为其它函数。hasher 是一个实现了 `BuildHasher` trait 的类型 ([更多关于 trait 的细节](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html))。你并不需要从头开始实现你自己的 hasher；[crates.io](https://crates.io/) 有其他人分享的实现了许多常用哈希算法的 hasher 的库。
