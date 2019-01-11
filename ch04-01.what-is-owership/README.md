# 什么是所有权

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html)

## 所有权规则

Rust 中的每一个值都有一个被称为其 **所有者 (owner)** 的变量。

值有且只有一个所有者。

当所有者（变量）离开作用域，这个值将被丢弃。

## 字符串

**Rust 中有两种字符串**

```rust
let s = "hello, world!" // 不能被修改
println("{}", s); // 将打印 "hello, world!"
```

第一种字符串使用 `""` 定义

存放在栈上，编译时硬编码到可执行文件中，**不能被修改**

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // 可以被修改
println("{}", s); // 将打印 "hello, world!"
```

第二种字符串类型名为 `String`，通过 `from`、`new` 等函数定义 

存放在堆上，运行时向操作系统申请内存，**可以被修改**

## Rust 的内存管理方式

程序会在变量离开作用域时自动调用 `drop` 函数回收内存。(类似于 C++ 中的 RAII)

### 移动

```rust
let s1 = String::from("hello");
let s2 = s1;
```

这段代码中，`s1` 被**移动**到了 `s2`，`s1` 本身失效。

![trpl04-04](https://kaisery.github.io/trpl-zh-cn/img/trpl04-04.svg)

### 克隆

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

这段代码中 `s2` 深度复制了 `s1` 的数据，包括在堆上和栈上的数据

![trpl04-03](https://kaisery.github.io/trpl-zh-cn/img/trpl04-03.svg)

### 拷贝

有的数据类型 (只在堆上) 使用**拷贝**而非**移动**

- 所有整数类型，比如 u32。
- 布尔类型，bool，它的值是 true 和 false。
- 所有浮点数类型，比如 f64。
- 字符类型，char。
- 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

## 所有权与函数

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

将值传递给函数在语义上与给变量赋值相似，即可能是**移动**也可能是**复制**。

## 返回值与作用域

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中, 
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
```

返回值也可以转移所有权。
