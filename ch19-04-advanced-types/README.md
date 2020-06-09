# 高级类型

## 为了类型安全和抽象而使用 newtype 模式

newtype 模式可以用于一些其他我们还未讨论的功能，包括静态的确保某值不被混淆，和用来表示一个值的单元。

例如 `Millimeters` 和 `Meters` 结构体都在 newtype 中封装了 `u32` 值。如果编写了一个有 `Millimeters` 类型参数的函数，不小心使用 `Meters` 或普通的 `u32` 值来调用该函数的程序是不能编译的。

```rust
struct Millimeters(u32);
struct Meters(u32);
```

另一个 newtype 模式的应用在于抽象掉一些类型的实现细节：例如，封装类型可以暴露出与直接使用其内部私有类型时所不同的公有 API，以便限制其功能。newtype 模式是一种实现第十七章 ["封装隐藏了实现细节"](https://kaisery.github.io/trpl-zh-cn/ch17-01-what-is-oo.html#encapsulation-that-hides-implementation-details) 部分所讨论的隐藏实现细节的封装的轻量级方法。

## 类型别名用来创建类型同义词

与 newtype 模式类似，Rust 还提供了声明**类型别名** (type alias) 的能力，使用 `type` 关键字来给予现有类型另一个名字。

```rust
type Kilometers = i32;
```

这意味着 `Kilometers` 是 i32 的**同义词** (synonym)；`Kilometers` 不是一个新的、单独的类型。`Kilometers` 类型的值将被完全当作 `i32` 类型值来对待：

```rust
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

类型别名的主要用途是减少重复。

```rust

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

类型别名也经常与 `Result<T, E>` 结合使用来减少重复。

```rust
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}
```

因为这是一个别名，它只是另一个 `Result<T, E>`，这意味着可以在其上使用 `Result<T, E>` 的任何方法，以及像 `?` 这样的特殊语法。

## 从不返回的 never type

Rust 有一个叫做 `!` 的特殊类型。在类型理论术语中，它被称为 _empty type_，因为它没有值。我们更倾向于称之为 _never type_。

这个名字描述了它的作用：在函数从不返回的时候充当返回值。

```rust
fn bar() -> ! {
    // --snip--
}
```

这读做 "函数 `bar` 从不返回"，而从不返回的函数被称为**发散函数** (diverging functions)。

它有什么用呢？

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

众所周知，`match` 的分支必须返回相同的类型。

这里 `continue` 的值就是 `!`。也就是说，当 Rust 要计算 `guess` 的类型时，它查看这两个分支。前者是 `u32` 值，而后者是 `!` 值。因为 `!` 并没有一个值，Rust 决定 `guess` 的类型是 `u32`。

描述 `!` 的行为的正式方式是 never type 可以强转为任何其他类型。允许 `match` 的分支以 `continue` 结束是因为 `continue` 并不真正返回一个值；相反它把控制权交回上层循环，所以在 `Err` 的情况，事实上并未对 `guess` 赋值。

never type 的另一个用途是 `panic!`。

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Rust 知道 `val` 是 `T` 类型，`panic!` 是 `!` 类型，所以整个 `match` 表达式的结果是 `T` 类型。

最后一个有着 `!` 类型的表达式是 `loop`：

```rust
print!("forever ");

loop {
    print!("and ever ");
}
```

这里，循环永远也不结束，所以此表达式的值是 `!`。但是如果引入 `break` 这就不为真了，因为循环在执行到 `break` 后就会终止。

## 动态大小类型和 Sized trait

因为 Rust 需要知道例如应该为特定类型的值分配多少空间这样的信息其类型系统的一个特定的角落可能令人迷惑：这就是**动态大小类型** (_dynamically sized types_) 的概念。这有时被称为 "DST" 或 "unsized types"，这些类型允许我们处理只有在运行时才知道大小的类型。

`str` 是一个 DST；直到运行时我们都不知道字符串有多长。

因为直到运行时都不能知道大其小，也就意味着不能创建 `str` 类型的变量，也不能获取 `str` 类型的参数。

所以虽然 `&T` 是一个储存了 `T` 所在的内存位置的单个值，`&str` 则是**两个**值：`str` 的地址和其长度。

里是 Rust 中动态大小类型的常规用法：他们有一些额外的元信息来储存动态信息的大小。

这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。

可以将 `str` 与所有类型的指针结合：比如 `Box<str>` 或 `Rc<str>`。

另一个动态大小类型是 trait。

将 trait 用于 trait 对象，必须将他们放入指针之后，比如 `&Trait` 或 `Box<Trait>` (`Rc<Trait>` 也可以)。

为了处理 DST，Rust 有一个特定的 trait 来决定一个类型的大小是否在编译时可知：这就是 `Sized` trait。这个 trait 自动为编译器在编译时就知道大小的类型实现。另外，Rust 隐式的为每一个泛型函数增加了 `Sized` bound。

也就是说，对于如下泛型函数定义：

```rust
fn generic<T>(t: T) {
    // --snip--
}
```

实际上被当作如下处理：

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

可以使用如下特殊语法来放宽这个限制：

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

`?Sized` trait bound 与 `Sized` 相对；这个语法只能用于 `Sized`，而不能用于其他 trait。