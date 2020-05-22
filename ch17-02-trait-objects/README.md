# 为使用不同类型的值而设计的 trait 对象
## 定义通用行为的 trait

trait 对象指向一个实现了我们指定 trait 的类型的实例，以及一个用于在运行时查找该类型的 trait 方法的表。我们通过指定某种指针来创建 trait 对象，例如 `&` 引用或 `Box<T>` 智能指针，还有 `dyn` keyword， 以及指定相关的 trait。我们可以使用 trait 对象代替泛型或具体类型。任何使用 trait 对象的位置，Rust 的类型系统会在编译时确保任何在此上下文中使用的值会实现其 trait 对象的 trait。如此便无需在编译时就知晓所有可能的类型。

Rust 刻意不将结构体与枚举称为 “对象”，以便与其他语言中的对象相区别。在结构体或枚举中，结构体字段中的数据和 impl 块中的行为是分开的，不同于其他语言中将数据和行为组合进一个称为对象的概念中。trait 对象将数据和行为两者相结合，从这种意义上说**则**其更类似其他语言中的对象。不过 trait 对象不同于传统的对象，因为不能向 trait 对象增加数据。trait 对象并不像其他语言中的对象那么通用：其 (trait 对象) 具体的作用是允许对通用行为进行抽象。

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

泛型类型参数一次只能替代一个具体类型，而 trait 对象则允许在运行时替代多种具体类型。

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

## 实现 trait

```rust
use gui::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

```

```rust
use gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

只关心值所反映的信息而不是其具体类型 —— 类似于动态类型语言中称为**鸭子类型** (duck typing) 的概念：如果它走起来像一只鸭子，叫起来像一只鸭子，那么它就是一只鸭子！

## trait 对象执行动态分发

单态化所产生的代码进行**静态分发** (static dispatch)。静态分发发生于编译器在编译时就知晓调用了什么方法的时候。这与**动态分发**  (dynamic dispatch) 相对，这时编译器在编译时无法知晓调用了什么方法。在动态分发的情况下，编译器会生成在运行时确定调用了什么方法的代码。


当使用 trait 对象时，Rust 必须使用动态分发。使用 trait 对象可以获得额外的灵活性，但代价是性能不如静态分发。在使用时仍需要权衡取舍。

## Trait 对象要求对象安全

只有**对象安全** (object safe)的 trait 才可以组成 trait 对象。

如果一个 trait 中所有的方法有如下属性时，则该 trait 是对象安全的：

- 返回值类型不为 `Self`
- 方法没有任何泛型类型参数

[更多细节 (Rust RFC 255)](https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md)