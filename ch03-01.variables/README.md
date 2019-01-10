# 变量和可变性

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html)

**Rust 中变量默认是不可变的**

常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。

```rust
let a = 100_000;                 // 一个不可变的变量
let mut b = 100_000;             // 一个可变的变量
const MAX_POINTS: u32 = 100_000; // 一个常量
```

[更多细节](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html#a%E5%8F%98%E9%87%8F%E5%92%8C%E5%B8%B8%E9%87%8F%E7%9A%84%E5%8C%BA%E5%88%AB)

## 隐藏 (Shadowing)

我们可以定义一个与原变量名字相同的新变量，此时原变量被新变量隐藏。

隐藏的变量与被隐藏的变量不一定类型相同。

使用 `mut` 修饰的 (可变的) 变量可以被隐藏但不能改变类型。

```rust
let x = 5;
let x = x + 1; // 此时 x (右值) 被新的 x (左值) 隐藏

let spaces = "  ";
let spaces = spaces.len(); // 隐藏不必类型相同
```

[更多细节](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html#a%E9%9A%90%E8%97%8Fshadowing)
