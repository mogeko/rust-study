# 使用 `Drop` Trait 运行清理代码


`Drop` trait 要求实现一个叫做 `drop` 的方法，它获取一个 `self` 的可变引用。

`drop` 方法在实例离开作用域时被自动调用，不允许显式调用

`drop` 方法是**析构函数** (destructor)，这是一个清理实例的函数的通用编程概念。**析构函数**对应创建实例的**构造函数**。

Rust 不允许我们显式调用 `drop` 因为 Rust 仍然会在实例离开作用域时对值自动调用 `drop`，这会导致一个 **double free** 错误，因为 Rust 会尝试清理相同的值两次。

如果希望提前调用 `drop`，可以使用 `std::mem::drop` 函数：

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

顺便一提，`std::mem::drop` 是这么实现的：

```rust
pub fn drop<T>(_x: T) { }
```

就是一个空函数！！

利用泛型获取实例的所有权，利用作用域规则自动调用 `drop` 方法，完美符合零抽象原则