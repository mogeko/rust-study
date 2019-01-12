# 定义并实例化结构体

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html)

```rust
// 定义一个结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 实例化一个结构体
let mut user1 = User {
    email: String::from("zhengjunyi@live.com"),
    username: String::from("Mogeko"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 // 其余值来自 user1 变量中实例的字段
};

// 使用 (修改) 一个结构体的字段的值
user1.email = String::from("mogeko.zheng@gmail.com");
```

## 元组结构体

元组结构体没有具体的字段名，只有字段的类型

```rust
struct Point(i32, i32, i32);

let origin = Point(0, 0, 0);

println!(
    "This is an origin point: ({}, {}, {})",
    origin.1, origin.1, origin.1
);
```

## 类单元结构体 (几乎没用)

没有任何字段的结构体被称为类单元结构体

## 结构体数据的所有权

可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上**生命周期** (lifetimes)

[更多关于生命周期的细节](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)
