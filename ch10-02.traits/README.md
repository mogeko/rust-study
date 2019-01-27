# trait：定义共享的行为

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)

*trait* 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 *trait bounds* 指定泛型是任何拥有特定行为的类型。

> 注意：trait 类似于其他语言中的常被称为 接口 (interfaces)的功能，虽然有一些不同。

## 定义 trait

一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## 为类型实现 trait

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

如果别人想要利用我们 crate 的功能为其自己的库作用域中的结构体实现 `Summary` trait。首先他们需要将 trait (通过 `use`) 引入作用域。`Summary` 还必须是公有 trait 使得其他 crate 可以实现它。

只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。

不能为外部类型实现外部 trait。例如，不能在本 crate 中为 `Vec<T>` 实现 `Display` trait。这是因为 `Display` 和 `Vec<T>` 都定义于标准库中，它们并不位于本 crate 本地作用域中。这个限制是被称为**相干性** (*coherence*) 的程序属性的一部分，或者更具体的说是**孤儿规则** (*orphan rule*)，其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。

## 默认实现

有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

为了使用这个版本的 `Summary`，只需在实现 trait 时定义 `summarize_author` 即可：

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

## trait 作为参数

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

在 `notify` 函数体中，可以调用任何来自 `Summary` trait 的方法，比如 `summarize`。

### Trait Bounds

`impl Trait` 语法是一个 *trait bound* 的语法糖。它的真正写法是这样的：

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

trait bound 与泛型参数声明在一起，位于尖括号中分号的后面。因为 `T` 的 trait bound，我们可以传递任何 `NewsArticle` 或 `Tweet` 的实例调用 `notify`。用任何其他类型，比如 `String` 或 `i32`，调用该函数的代码将不能编译，因为这些类型没有实现 `Summary`。

### 通过 `+` 指定多个 trait

```rust
pub fn notify(item: impl Summary + Display) {
```

```rust
pub fn notify<T: Summary + Display>(item: T) {
```

### 通过 `where` 简化代码

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

```rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

## 返回 trait

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

## 使用 trait bound 有条件地实现方法

通过使用带有 trait bound 的泛型参数的 `impl` 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

// 只有实现了 Display 和 PartialOrd 的 Pair<T> 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

也可以对任何实现了特定 trait 的类型有条件地实现 trait。对任何满足特定 trait bound 的类型实现 trait 被称为 *blanket implementations*，他们被广泛的用于 Rust 标准库中。

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```
