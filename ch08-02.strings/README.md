# 使用字符串存储 UTF-8 编码的文本

[教程原文](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html)

符串是新晋 Rustacean 们通常会被困住的领域，这是由于三方面理由的结合：
- Rust 倾向于确保暴露出可能的错误
- 字符串是比很多程序员所想象的要更为复杂的数据结构
- UTF-8

Rust 的核心语言中只有一种字符串类型：`str`，字符串 slice，它通常以被借用的形式出现，`&str`。

> **字符串 slice：**它们是一些储存在别处的 UTF-8 编码字符串数据的引用。比如字符串字面值被储存在程序的二进制输出中，字符串 slice 也是如此。

称作 `String` 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。

## 新建字符串

```rust
// 新建一个空的字符串
let mut s = String::new();

// 初始化一个字符串
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

## 更新字符串

### 使用 `push_str` 和 `push` 附加字符串

```rust
let mut s = String::from("foo");
s.push_str("bar"); // 使用 push_str 方法向 String 附加字符串 slice

let mut s = String::from("lo");
s.push('l'); // 使用 push 将一个字符加入 String 值中
```

### 使用 `+` 运算符或 `format!` 宏拼接字符串

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");

let s3 = s1 + &s2; // 使用 + 运算符将两个 String 值合并到一个新的 String 值中
                   // 注意 s1 被移动了，不能继续使用

let s = format!("{}{}", s1, s2); // 使用 format! 宏
                                 // format! 宏不会获取任何参数的所有权
```

## 索引字符串

Rust 中的字符串不支持索引。

## 遍历字符串的方法

如果你需要操作单独的 Unicode 标量值，最好的选择是使用 `chars` 方法。

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

运行结果：

```shell
न
म
स
्
त
े
```

`bytes` 方法返回每一个原始字节。

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

运行结果：

```shell
224
164
// --snip--
165
135
```