# 一个使用结构体的示例程序

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch05-02-example-structs.html)

## 通过派生 trait 增加实用功能

```rust
#[derive(Debug)] // 添加显式选择的功能
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1); // Debug 输出
    println!("rect1 is {:#?}", rect1); // 格式化的Debug 输出

}
```

如果想让 `println!` 直接打印出结构体，可以用

- `#[derive(Debug)]` 注释结构体定义，添加显式选择的功能
- 用 `{:?}` 或 `{:#?}` (格式化) 输出
