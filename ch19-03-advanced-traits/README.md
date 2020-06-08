# 高级 trait

## 关联类型在 trait 定义中指定占位符类型

**关联类型** (_associated types_) 是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。如此可以定义一个使用多种类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么。

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```

那么为什么不用泛型？

如果使用泛型，我们会这么定义 `Iterator`：

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

着意味着我们可以同时实现 `impl Iterator<u32> for Counter` 和 `impl Iterator<String> for Counter`。

而对于关联类型，我们只能选择一次 `Item` 会是什么类型，因为只能有一个 `impl Iterator for Counter`。

## 默认泛型类型参数和运算符重载

当使用泛型类型参数时，可以为泛型指定一个默认的具体类型。

使用默认泛型类型的语法是： `<PlaceholderType=ConcreteType>`。

这种情况的一个非常好的例子是用于运算符重载。

**运算符重载** (Operator overloading) 是指在特定情况下自定义运算符 (比如 `+`) 行为的操作。

```rust
trait Add<RHS=Self> { // RHS 是一个泛型类型参数 ("right hand side" 的缩写)
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

如果被相加的是同一类型，则在实现时不需要声明泛型的具体类型，因为默认类型是 `Self`

```rust
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

但如果被相加的不是同一类型，则需要声明具体是什么类型：

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```
默认参数类型主要用于如下两个方面：

- 扩展类型而不破坏现有代码。
- 在大部分用户都不需要的特定情况进行自定义。

标准库的 `Add` trait 就是一个第二个目的例子：大部分时候你会将两个相似的类型相加，不过它提供了自定义额外行为的能力。在 `Add` trait 定义中使用默认类型参数意味着大部分时候无需指定额外的参数。换句话说，一小部分实现的样板代码是不必要的，这样使用 trait 就更容易了。

第一个目的是相似的，但过程是反过来的：如果需要为现有 trait 增加类型参数，为其提供一个默认类型将允许我们在不破坏现有实现代码的基础上扩展 trait 的功能。

## 完全限定语法与消歧义：调用相同名称的方法

Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait。甚至直接在类型上实现开始已经有的同名方法也是可能的！

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

在 `main` 调用了 `Dog::baby_name` 函数，它直接调用了定义于 `Dog` 之上的关联函数。这段代码会打印出：

```
A baby dog is called a Spot
```

为了调用 `Animal` trait 上的 `baby_name()` 我们需要使用**完全限定语法**：

```rust
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

通常，完全限定语法定义为：

```
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

只有当存在多个同名实现而 Rust 需要帮助以便知道我们希望调用哪个实现时，才需要使用这个较为冗长的语法。

## 父 trait 用于在另一个 trait 中使用某 trait 的功能

有时我们可能会需要某个 trait 使用另一个 trait 的功能。在这种情况下，需要能够依赖相关的 trait 也被实现。这个所需的 trait 是我们实现的 trait 的**父 (超) trait** (_supertrait_)。

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

因为指定了 `OutlinePrint` 需要 `Display` trait，则可以在 `outline_print` 中使用 `to_string`， 其会为任何实现 `Display` 的类型自动实现。如果不在 `trait` 名后增加 `: Display` 并尝试在 `outline_print` 中使用 `to_string`，则会得到一个错误说在当前作用域中没有找到用于 `&Self` 类型的方法 `to_string`。

如果想实现 `OutlinePrint` trait，则需要先实现 `Display` trait：

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}
```

## newtype 模式用以在外部类型上实现外部 trait

在第十章的 ["为类型实现 trait"](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html#implementing-a-trait-on-a-type) 部分，我们提到了孤儿规则 (orphan rule)，它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。一个绕开这个限制的方法是使用 **newtype 模式** (_newtype pattern_)，它涉及到在一个元组结构体 (第五章 ["用没有命名字段的元组结构体来创建不同的类型"](https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) 部分介绍了元组结构体) 中创建一个新类型。这个元组结构体带有一个字段作为希望实现 trait 的类型的简单封装。接着这个封装类型对于 crate 是本地的，这样就可以在这个封装上实现 trait。_Newtype_ 是一个源自 (U.C.0079，逃) Haskell 编程语言的概念。使用这个模式没有运行时性能惩罚，这个封装类型在编译时就被省略了。

例如，如果想要在 `Vec<T>` 上实现 `Display`，而孤儿规则阻止我们直接这么做，因为 `Display` trait 和 `Vec<T>` 都定义于我们的 crate 之外。可以创建一个包含 `Vec<T>` 实例的 `Wrapper` 结构体，接着在 `Wrapper` 上实现 `Display` 并使用 `Vec<T>` 的值：

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

此方法的缺点是，因为 `Wrapper` 是一个新类型，它没有定义于其值之上的方法；必须直接在 `Wrapper` 上实现 `Vec<T>` 的所有方法，这样就可以代理到 `self.0` 上 —— 这就允许我们完全像 `Vec<T>` 那样对待 `Wrapper`。如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 `Deref` trait （第十五章 ["通过 `Deref` trait 将智能指针当作常规引用处理"](https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait) 部分讨论过) 并返回其内部类型是一种解决方案。如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。
