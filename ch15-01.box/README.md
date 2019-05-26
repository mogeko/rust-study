# `Box<T>` 在堆上存储数据，并且可确定大小

[原文地址](https://kaisery.github.io/trpl-zh-cn/ch15-01-box.html)

`Box<T>` 类型是一个智能指针，因为它实现了 `Deref` trait，它允许 `Box<T>` 值被当作引用对待。当 `Box<T>` 值离开作用域时，由于 `Box<T>` 类型 `Drop` trait 的实现，box 所指向的堆数据也会被清除。

它多用于如下场景：

- 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
- 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
- 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

## 使用 `Box<T>` 在堆上储存数据

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    assert_eq!(5, *b); // b 是 Box<T> 类型的指针，所以需要解引用
}
```

## box 允许创建递归类型

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

