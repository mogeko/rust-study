# 高级函数与闭包

## 函数指针

函数的类型是 `fn` (使用小写的 "f" ) 以免与 `Fn` 闭包 trait 相混淆。`fn` 被称为**函数指针** (_function pointer_)。通过函数指针允许我们使用函数作为另一个函数的参数。指定参数为函数指针的语法类似于闭包

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

不同于闭包，`fn` 是一个类型而不是一个 trait，所以直接指定 `fn` 作为参数而不是声明一个带有 `Fn` 作为 trait bound 的泛型参数。

函数指针实现了所有三个闭包 trait(`Fn`、`FnMut` 和 `FnOnce`)，所以总是可以在调用期望闭包的函数时传递函数指针作为参数。倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。

```rust

#![allow(unused_variables)]
fn main() {
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
}
```

```rust

let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

一个只期望接受 `fn` 而不接受闭包的情况的例子是与不存在闭包的外部代码交互时：C 语言的函数可以接受函数作为参数，但 C 语言没有闭包。

另一个实用的模式暴露了元组结构体和元组结构体枚举成员的实现细节。这些项使用 `()` 作为初始化语法，这看起来就像函数调用，同时它们确实被实现为返回由参数构造的实例的函数。它们也被称为实现了闭包 trait 的函数指针，并可以采用类似如下的方式调用：

```rust

#![allow(unused_variables)]
fn main() {
enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> =
    (0u32..20)
    .map(Status::Value)
    .collect();
}
```

## 返回闭包

闭包表现为 trait，这意味着不能直接返回闭包。因为 trait 是 DST (动态大小类型)，Rust 并不知道需要多少空间来储存闭包。

不过我们可以通过将闭包放在指针后来解决这一问题：

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
