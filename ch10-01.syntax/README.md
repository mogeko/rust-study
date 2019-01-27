# 泛型数据类型

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html)

**泛型** (*generics*)是具体类型或其他属性的抽象替代。我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么。

## 在函数定义中使用泛型

输入一个 `i32` 或者 `char` 的数组，返回其中最大的值。不使用泛型实现：

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

使用泛型实现：

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

函数 `largest` 有泛型类型 `T`。它有一个参数 `list`，它的类型是一个 `T` 值的 slice。`largest` 函数将会返回一个与 `T` 相同类型的值。

这里使用到了 [trait bound](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html#trait-%E4%BD%9C%E4%B8%BA%E5%8F%82%E6%95%B0)。

## 结构体定义中的泛型

```rust
struct Poinr<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = point {x: 5, y: 10};
    let float = point {x: 1.0, y: 4.0};
}
```

结构体 `Point` 对于一些类型 `T` 是泛型的，而且字段 `x` 和 `y` 都是**相同**类型的。不能创建一个有不同类型值的 `Point` 的实例。

定义一个拥有多个泛型类型参数的结构体：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

## 枚举定义中的泛型

与结构体中相似。

```rust
// Option 的定义
enum Option<T> {
    Some(T),
    None,
}

// Result 的定义
enum Result<T, E> {
    OK(T),
    Err(E),
}
```

## 方法定义中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 在方法中使用泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 在方法中使用具体的类型
// 这意味着 Point<f32> 类型会有一个方法 distance_from_origin，
// 而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi() + self.y.powi()).sqrt()
    }
}

fn main() {
    let p = Point {x: 5, y: 10};

    println!("p.x = {}", p.x());
}
```

结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。结构体 `Point<T, U>` 上定义了一个方法 `mixup`。这个方法获取另一个 `Point` 作为参数，而它可能与调用 `mixup` 的 `self` 是不同的 `Point` 类型。这个方法用 `self` 的 `Point` 类型的 `x` 值 (类型 `T`) 和参数的 `Point` 类型的 `y` 值 (类型 `W`) 来创建一个新 `Point` 类型的实例：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## 泛型的性能

Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并**没有任何速度上的损失**。