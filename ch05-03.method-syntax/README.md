# 方法语句

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch05-03-method-syntax.html)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## 自动引用和解引用

```rust
// 以下代码是等价的
p1.distance(&p2);
(&p1).distance(&p2);
```

当使用 `object.something()` 调用方法时，Rust 会自动为 `object` 添加 `&`、`&mut` 或 `*` 以便使 `object` 与方法签名匹配。

## 关联函数

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let sq = Rectangle::square(3); // 使用 :: 调用关联函数
```

允许在 `impl` 块中定义**不**以 `self` 作为参数的函数。这被称为**关联函数** (associated functions)

`::` 语法用于关联函数和模块创建的命名空间 [关于模块的更多细节](https://kaisery.github.io/trpl-zh-cn/ch07-02-modules-and-use-to-control-scope-and-privacy.html)


