# 使用线程同时运行代码

编程语言有一些不同的方法来实现线程。很多操作系统提供了创建新线程的 API。这种由编程语言调用操作系统 API 创建线程的模型有时被称为 _1:1_，一个 OS 线程对应一个语言线程。

很多编程语言提供了自己特殊的线程实现。编程语言提供的线程被称为 **_绿色_** (green) 线程，使用绿色线程的语言会在不同数量的 OS 线程的上下文中执行它们。为此，绿色线程模式被称为 _M:N_ 模型：`M` 个绿色线程对应 `N` 个 OS 线程，这里 `M` 和 `N` 不必相同。

绿色线程的 M:N 模型需要更大的语言运行时来管理这些线程。因此，Rust 标准库只提供了 1:1 线程模型实现。由于 Rust 是较为底层的语言，如果你愿意牺牲性能来换取抽象，以获得对线程运行更精细的控制及更低的上下文切换成本，你可以使用实现了 M:N 线程模型的 crate。

## 使用 `spawn` 创建新线程
## 使用 `join` 等待所有线程结束

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

## 线程与 `move` 闭包

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

如果为闭包增加 `move`，将会把 v 移动进闭包的环境中