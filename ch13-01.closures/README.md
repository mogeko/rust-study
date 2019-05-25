# 闭包：可以捕获环境的匿名函数

[原文地址](https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html)

我们使用以下语法定义一个闭包

```rust
let plus_one = |x| x + 1;
```

闭包与普通函数最大的区别在于闭包可以捕获环境中的变量

```rust
let num = 5;
let plus_num = |x: i32| x + num;

assert_eq!(10, plus_num(5));
```

以上例子中 `num` 是以指针的形式在闭包中被调用的

我们可以强制我们的闭包用 move 关键字获取环境移所有权：

```rust
let num = 5;
let owns_num = move |x: i32| x + num;
```

## 闭包实现

```rust
pub trait Fn<Args> : FnMut<Args> {
extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
type Output;

extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

## 使用闭包作为参数

回调函数也是这样实现的

```rust
fn call_with_one<F>(some_closure: F) -> i32
where F : Fn(i32) -> i32 {

some_closure(1)
}

let answer = call_with_one(|x| x + 2);

assert_eq!(3, answer);
```

以上代码中，函数 `call_with_one` 将会获取闭包 `|x| x + 2` 的所有权，以下代码则不会

```rust
fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
some_closure(1)
}

let answer = call_with_one(&|x| x + 2);

assert_eq!(3, answer);
```

现在我们把一个特征对象 `&Fn`。当我们传递这个特征到 `call_with_one` 时我们必须参考我们的闭包,所以我们使用 `&| |`。