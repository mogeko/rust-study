# match 控制流运算符

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)

> 可以把 `match` 表达式想象成某种硬币分类器：硬币滑入有着不同大小孔洞的轨道，每一个硬币都会掉入符合它大小的孔洞。同样地，值也会通过 `match` 的每一个模式，并且在遇到第一个 “符合” 的模式时，值会进入相关联的代码块并在执行中被使用。

`match` 与 `if` 的区别：

- `if` 的表达式必须返回一个**布尔值**，而 `match` 的表达式可以返回**任何类型**。
- `if` 判断的是**值**，而 `match` 匹配的是**模式**

## `_` 通配符

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

`_` 模式会匹配所有的值。

将其放置于其他分支之后，`_` 将会匹配所有之前没有指定的可能的值。

`()` 就是 unit 值，所以 `_` 的情况什么也不会发生。
