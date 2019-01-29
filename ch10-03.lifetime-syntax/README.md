# 生命周期与引用有效性

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)

Rust 中的每一个引用都有其**生命周期** (*lifetime*)，也就是引用保持有效的作用域。

大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样。

生命周期注解并不改变任何引用的生命周期的长短。生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。

## 函数签名中的生命周期注解

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len > y.len {
        x
    } else {
        y
    }
}
```

通过在函数签名中指定生命周期参数时，我们并没有改变任何传入后返回的值的生命周期。

当具体的引用被传递给 `longest` 时，被 `'a` 所替代的具体生命周期是 `x` 的作用域与 `y` 的作用域相重叠的那一部分。换一种说法就是泛型生命周期 `'a` 的具体生命周期等同于 `x` 和 `y` 的生命周期中较小的那一个。

## 深入理解生命周期

指定生命周期参数的正确方式依赖函数实现的具体功能。

当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。如果返回的引用**没有**指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域。

## 结构体定义中的生命周期注解

在结构体中使用引用也必须为每个引用添加生命周期注释。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

`ImportantExcerpt` 的实例不能比其 `part` 字段中的引用存在的更久。

## 生命周期省略 (Lifetime Elision)

函数或方法的参数的生命周期被称为**输入生命周期** (*input lifetimes*)，而返回值的生命周期被称为**输出生命周期** (*output lifetimes*)。

**生命周期省略规则** (*lifetime elision rules*):

1. 每一个是引用的参数都有它自己的生命周期参数：`fn foo<'a>(x: &'a i32)`，有两个引用参数的函数有两个不同的生命周期参数，`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，依此类推。

2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。

3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 `&self` 或 `&mut self`，那么 `self` 的生命周期被赋给所有输出生命周期参数。

## 方法定义中的生命周期注解

```rust
// impl 之后和类型名称之后的生命周期参数是必要的
impl<'a> ImportantExcerpt<'a> {
    // &self 与 announcement 拥有各自的生命周期，
    // (输出) &str 的生命周期等于 &self 的生命周期 (规则3)
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## 静态生命周期

Rust 中有一种特殊的生命周期：`'static`，其生命周期存活于整个程序期间。

所有的字符串字面值都拥有 `'static` 生命周期。

## 结合泛型类型参数、trait bounds 和生命周期

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(a: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```