# 共享状态并发

## 互斥器一次只允许一个线程访问数据

**互斥器** (mutex) 是 _mutual exclusion_ 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。为了访问互斥器中的数据，线程首先需要通过获取互斥器的**锁** (lock) 来表明其希望访问数据。锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。因此，我们描述互斥器为通过锁系统**保护** (guarding) 其数据。

**使用互斥器时务必记住**：

1. 在使用数据之前尝试获取锁。
2. 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

不过在 Rust 中，得益于类型系统和所有权，我们不会在锁和解锁上出错。

## `Mutex<T>` 的 API

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

`Mutex<T>` 是一个智能指针。更准确的说，lock 调用**返回**一个叫做 `MutexGuard` 的智能指针。

这个智能指针实现了 `Deref` 来指向其内部数据；其也提供了一个 `Drop` 实现当 `MutexGuard` 离开作用域时自动释放锁

## 在线程间共享 `Mutex<T>`

### 多线程和多所有权

`Rc<T>` 不能安全的在线程间共享。

### 原子引用计数 `Arc<T>`

`Arc<T>` 是一个类似 `Rc<T>` 并可以安全的用于并发环境的类型。

字母 “a” 代表 原子性 (atomic)，所以这是一个原子引用计数 (atomically reference counted) 类型。

原子性类型工作起来类似原始类型，不过可以安全的在线程间共享。

```rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## `RefCell<T>`/`Rc<T>` 与 `Mutex<T>`/`Arc<T>` 的相似性

`Mutex<T>` 提供了内部可变性，就像 `Cell` 系列类型那样。

同 `RefCell<T>` 一样，Rust 不能避免使用 `Mutex<T>` 的全部逻辑错误。使用 `RefCell<T>`/`Rc<T>` 可能造成引用循环，`Mutex<T>` 也有造成**死锁** (deadlock) 的风险。这发生于当一个操作需要锁住两个资源而两个线程各持一个锁，这会造成它们永远相互等待。