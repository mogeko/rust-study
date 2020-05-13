# `Rc<T>` 引用计数智能指针

Rust 有一个叫做 `Rc<T>` 的类型。其名称为 **引用计数** (reference counting) 的缩写。引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。

![trpl15-03](https://kaisery.github.io/trpl-zh-cn/img/trpl15-03.svg)

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

`a.clone()` 与 `Rc::clone(&a)` 等效，但更推荐后者，因为方便重构

`Rc::clone` 的实现并不像大部分类型的 `clone` 实现那样对所有数据进行深拷贝。`Rc::clone` 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。通过使用 `Rc::clone` 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 `Rc::clone` 调用。

引用计数的值可以通过调用 `Rc::strong_count` 函数获得。这个函数叫做 `strong_count` 而不是 `count` 是因为 `Rc<T>` 也有 `weak_count`；在 [“避免引用循环：将 `Rc<T>` 变为 `Weak<T>`”](https://kaisery.github.io/trpl-zh-cn/preventing-ref-cycles) 部分会讲解 `weak_count` 的用途。

通过不可变引用，`Rc<T>` 允许在程序的多个部分之间只读地共享数据。如果 `Rc<T>` 也允许多个可变引用，则会违反第四章讨论的借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致。

如果想改变数据需要用到 `RefCell<T>`