# 宏

**宏** (_Macro_) 指的是 Rust 中一系列的功能：

- **声明** (_Declarative_) 宏
- 使用 `macro_rules!`
- 三种**过程** (_Procedural_) 宏：
  - 自定义 `#[derive]` 宏在结构体和枚举上指定通过 `derive` 属性添加的代码
  - 类属性 (Attribute) 宏定义可用于任意项的自定义属性
  - 类函数宏看起来像函数不过作用于作为参数传递的 token。

## 宏和函数的区别

从根本上来说，宏是一种为写其他代码而写代码的方式，即所谓的**元编程** (_metaprogramming_)。

所有的这些宏以**展开**的方式来生成比你所手写出的更多的代码。

一个函数标签必须声明函数参数个数和类型。相比之下，宏只接受一个可变参数：用一个参数调用 `println!("hello")` 或用两个参数调用 `println!("hello {}", name)`。而且，宏可以在编译器翻译代码前展开，例如，宏可以在一个给定类型上实现 trait 。而函数则不行，因为函数是在运行时被调用，同时 trait 需要在编译时实现。

实现一个宏而不是函数的消极面是宏定义要比函数定义更复杂，因为你正在编写生成 Rust 代码的 Rust 代码。由于这样的间接性，宏定义通常要比函数定义更难阅读、理解以及维护。

宏和函数的最后一个重要的区别是：在调用宏**之前**必须定义并将其引入作用域，而函数则可以在任何地方定义和调用。

## 使用 `macro_rules!` 的声明宏用于通用元编程

Rust 最常用的宏形式是**声明宏** (_declarative macros_)。

其核心概念是，声明宏允许我们编写一些类似 Rust `match` 表达式的代码。宏将一个值和包含相关代码的模式进行比较；此种情况下，该值是传递给宏的 Rust 源代码字面值，模式用于和传递给宏的源代码进行比较，同时每个模式的相关代码则用于替换传递给宏的代码。所有这一切都发生于编译时。

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

`vec!` 稍微简化的定义：

```rust
#[macro_export]
macro_rules! vec {
    // 此处有一个单边模式
    // 更复杂的宏会有多个单边模式。
    // 首先，一对括号包含了全部模式。
    // 接下来是后跟一对括号的美元符号（$），其通过替代代码捕获了符合括号内模式的值。
    // $() 内则是 $x:expr ，其匹配 Rust 的任意表达式或给定 $x 名字的表达式。
    // $() 之后的逗号说明一个逗号分隔符可以有选择的出现代码之后，这段代码与在 $() 中所捕获的代码相匹配。
    // 紧随逗号之后的 * 说明该模式匹配零个或多个 * 之前的任何模式。
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

> 注意：标准库中实际定义的 `vec!` 包括预分配适当量的内存的代码。这部分为代码优化，为了让示例简化，此处并没有包含在内。

无论何时导入定义了宏的包，`#[macro_export]` 注解说明宏应该是可用的。 如果没有该注解，这个宏不能被引入作用域。

`macro_rules!` 中有一些奇怪的地方。在将来，会有第二种采用 `macro` 关键字的声明宏，其工作方式类似但修复了这些极端情况。在此之后，`macro_rules!` 实际上就过时 (deprecated) 了。

## 用于从属性生成代码的过程宏

第二种形式的宏被称为**过程宏** (procedural macros)，因为它们更像函数 (一种过程类型)。

过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出，而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码。

当创建过程宏时，其定义必须位于一种特殊类型的属于它们自己的 crate 中 (该限制最终可能被取消)。

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

过程宏的函数接受一个 `TokenStream` 作为输入并产生一个 `TokenStream` 作为输出。

这也就是宏的核心：宏所处理的源代码组成了输入 `TokenStream`，同时宏生成的代码是输出 `TokenStream`。

最后，函数上有一个属性 (`#[some_attribute]`)；这个属性表明过程宏的类型。

在同一 crate 中可以有多种的过程宏。

## 如何编写自定义 derive 宏

让我们创建一个 `hello_macro` crate，其包含名为 `HelloMacro` 的 trait 和关联函数 `hello_macro`。不同于让 crate 的用户为其每一个类型实现 `HelloMacro` trait，我们将会提供一个过程式宏以便用户可以使用 `#[derive(HelloMacro)]` 注解他们的类型来得到 `hello_macro` 函数的默认实现。该默认实现会打印 `Hello, Macro! My name is TypeName!`，其中 `TypeName` 为定义了 trait 的类型名。

我们会创建一个 crate，使程序员能够写以下的代码：

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```

首先新建一个库 crate：

```rust
$ cargo new hello_macro --lib
```

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

在 `hello_macro` 项目中新建名为 `hello_macro_derive` 的包：

```rust
$ cargo new hello_macro_derive --lib
```

由于两个 crate 紧密相关，因此在 `hello_macro` 包的目录下创建过程式宏的 crate。如果改变在 `hello_macro` 中定义的 trait ，同时也必须改变在 `hello_macro_derive` 中实现的过程式宏。这两个包需要分别发布，编程人员如果使用这些包，则需要同时添加这两个依赖并将其引入作用域。我们也可以只用 `hello_macro` 包而将 `hello_macro_derive` 作为一个依赖，并重新导出过程式宏的代码。

需要将 `hello_macro_derive` 声明为一个过程宏的 crate。同时也需要 `syn` 和 `quote` crate 中的功能，正如注释中所说，需要将其加到依赖中。

hello_macro_derive/Cargo.toml：
```toml
[lib]
proc-macro = true

[dependencies]
syn = "0.14.4"
quote = "0.6.3"
```

```rust
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

// 外部函数中的代码 (在这里是 hello_macro_derive) 几乎在所有你能看到或创建的过程宏 crate 中都一样。
// 内部函数 (在这里是 impl_hello_macro) 的函数体中所指定的代码则依过程宏的目的而各有不同。

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 构建 Rust 代码所代表的语法树
    // 以便可以进行操作
    let ast = syn::parse(input).unwrap();

    // 构建 trait 实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // quote! 宏让我们可以编写希望返回的 Rust 代码。
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
                                                        // stringify! 为 Rust 内置宏。
                                                        // 其接收一个 Rust 表达式，然后在
                                                        // 编译时将表达式转换为一个字符串常量
            }
        }
    };
    // 消费这个中间表示 (intermediate representation，IR)
    // 并返回所需的 TokenStream 类型值。
    gen.into()
}

```

## 类属性宏

类属性宏与自定义派生宏相似，不同于为 `derive` 属性生成代码，它们允许你创建新的属性。

```rust
#[route(GET, "/")]
fn index() {
```

`#[route]` 属性将由框架本身定义为一个过程宏。其宏定义的函数签名看起来像这样：

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

这里有两个 `TokenStream` 类型的参数；第一个用于属性内容本身，也就是 `GET, "/"` 部分。第二个是属性所标记的项，在本例中，是 `fn index() {` 和剩下的函数体。

## 类函数宏

类函数宏定义看起来像函数调用的宏。类似于 `macro_rules!`，它们比函数更灵活；例如，可以接受未知数量的参数。

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

这个宏会解析其中的 SQL 语句并检查其是否是句法正确的，这是比 `macro_rules!` 可以做到的更为复杂的处理。`sql!` 宏应该被定义为如此：

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```
