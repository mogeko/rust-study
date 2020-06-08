# 不安全 Rust

可以通过 `unsafe` 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块。

`unsafe` 的作用包括：
- 解引用裸指针
- 调用不安全的函数或方法
- 访问或修改可变静态变量
- 实现不安全 trait
- 访问 union 的字段

## 解引用裸指针

不安全 Rust 有两个被称为**裸指针* (_raw pointers_) 的类似于引用的新类型。

和引用一样，裸指针是可变或不可变的，分别写作 `*mut T` 和 `*const T` (这里的星号不是解引用运算符)。

与引用和智能指针的区别在于，裸指针：

- 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
- 不保证指向有效的内存
- 允许为空
- 不能实现任何自动清理功能

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

这里没有引入 `unsafe` 关键字。因为可以在安全代码中**创建**裸指针，但是不能在不安全块之外**解引用**裸指针

```rust
let address = 0x012345usize;
let r = address as *const i32;
```

你甚至可以创建一个指向任意内存地址的裸指针。

```rust

let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。

## 调用不安全函数或方法

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

必须在一个单独的 `unsafe` 块中调用 `dangerous` 函数。

### 创建不安全代码的安全抽象

仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。事实上，将不安全代码封装进安全函数是一个常见的抽象。

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // as_mut_ptr 返回一个 *mut i32 类型的裸指针，储存在 ptr 变量中。
    let ptr = slice.as_mut_ptr();

    // 保证 mid 必然小于等于 len
    assert!(mid <= len);

    unsafe {
        // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
        (slice::from_raw_parts_mut(ptr, mid),
         // 在 ptr 上调用 offset 方法并使用 mid 作为参数来获取一个从 mid 开始的
         // 裸指针，使用这个裸指针并以 mid 之后项的数量为长度创建一个 slice。
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```

`slice::from_raw_parts_mut` 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。裸指针上的 `offset` 方法也是不安全的，因为其必须确信此地址偏移量也是有效的指针。因此必须将 `slice::from_raw_parts_mut` 和 `offset` 放入 `unsafe` 块中以便能调用它们。通过观察代码，和增加 `mid` 必然小于等于 `len` 的断言，我们可以说 `unsafe` 块中所有的裸指针将是有效的 `slice` 中数据的指针。这是一个可以接受的 `unsafe` 的恰当用法。

### 使用 `extern` 函数调用外部代码

有时你的 Rust 代码可能需要与其他语言编写的代码交互。为此 Rust 有一个关键字，`extern`，有助于创建和使用**外部函数接口** (_Foreign Function Interface_，FFI)。外部函数接口是一个编程语言用以定义函数的方式，其允许不同 (外部) 编程语言调用这些函数。

`extern` 块中声明的函数在 Rust 代码中总是不安全的。

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

## 访问或修改可变静态变量

Rust 支持**全局变量** (_global variables_)，不过这对于 Rust 的所有权规则来说是有问题的。如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争。

全局变量在 Rust 中被称为**静态** (_static_) 变量。

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

static 变量类似于常量。

通常静态变量的名称采用 `SCREAMING_SNAKE_CASE` 写法，并**必须**标注变量的类型。静态变量只能储存拥有 `'static` 生命周期的引用，这意味着 Rust 编译器可以自己计算出其生命周期而无需显式标注。访问不可变静态变量是安全的。

常量与不可变静态变量可能看起来很类似，不过一个微妙的区别是静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。另一方面，常量则允许在任何被用到的时候复制其数据。

常量与静态变量的另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是**不安全**的。

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

## 实现不安全 trait

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

## 何时使用不安全代码

使用 `unsafe` 来进行以上操作是没有问题的，甚至是不需要深思熟虑的，不过使得 `unsafe` 代码正确也实属不易，因为编译器不能帮助保证内存安全。当有理由使用 `unsafe` 代码时，是可以这么做的，通过使用显式的 `unsafe` 标注使得在出现错误时易于追踪问题的源头。
