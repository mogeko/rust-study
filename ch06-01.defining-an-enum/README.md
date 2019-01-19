# 定义枚举

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

```rust
enum Message {
    Quit, // 没有关联任何数据
    Move { x: i32, y: i32 }, // 包含一个匿名结构体
    Write(String), // 包含单独一个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}

// 可以使用 impl 在枚举上定义方法。
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## `Option` 枚举和其相对于空值的优势

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

可以不需要 `Option::` 前缀来直接使用 `Some` 和 `None`

Rust 中并没有**空值 (Null)**，因为处理空值是很重要同时又是很容易被忽略的一件事，而 Rust 使用 `Option` 强制你必须处理为空的情况。

`Option` 中内置了很多的成员方法：[Option 的文档](https://doc.rust-lang.org/std/option/enum.Option.html)

