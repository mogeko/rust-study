# 使用消息传递在线程间传送数据

一个日益流行的确保安全并发的方式是**消息传递** (message passing)，这里线程或 actor 通过发送包含数据的消息来相互沟通。这个思想来源于 [Go 编程语言文档](http://golang.org/doc/effective_go.html)中的口号：“不要通过共享内存来通讯；而是通过通讯来共享内存。” (“Do not communicate by sharing memory; instead, share memory by communicating.”)

Rust 中一个实现消息传递并发的主要工具是**通道**(channel)，Rust 标准库提供了其实现的编程概念。你可以将其想象为一个水流的通道，比如河流或小溪。如果你将诸如橡皮鸭或小船之类的东西放入其中，它们会顺流而下到达下游。

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

这里使用 `mpsc::channel` 函数创建一个新的通道；`mpsc` 是**多个生产者，单个消费者** (multiple producer, single consumer) 的缩写。简而言之，Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的**发送** (sending) 端，但只能有一个消费这些值的**接收** (receiving) 端。想象一下多条小河小溪最终汇聚成大河：所有通过这些小河发出的东西最后都会来到下游的大河。

`mpsc::channel` 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。

> 由于历史原因，`tx` 和 `rx` 通常作为**发送者** (transmitter) 和**接收者** (receiver) 的缩写，所以这就是我们将用来绑定这两端变量的名字。

通道的接收端有两个有用的方法：`recv` 和 `try_recv`。

`recv` 会阻塞主线程执行直到从通道中接收一个值。一旦发送了一个值，`recv` 会在一个 `Result<T, E>` 中返回它。当通道发送端关闭，`recv` 会返回一个错误表明不会再有新的值到来了。

`try_recv` 不会阻塞，相反它立刻返回一个 `Result<T, E>`：`Ok` 值包含可用的信息，而 `Err` 值代表此时没有任何消息。

## 通道与所有权转移

在新建线程中的通道中发送完 `val` 值**之后**就**无法再使用**它了。

## 发送多个值并观察接收者的等待

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

在主线程中，不再显式调用 `recv` 函数：而是将 `rx` 当作一个迭代器。

对于每一个接收到的值，我们将其打印出来。当通道被关闭时，迭代器也将结束。

## 通过克隆发送者来创建多个生产者

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
// --snip--

let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--
}
```

这一次，在创建新线程之前，我们对通道的发送端调用了 `clone` 方法。这会给我们一个可以传递给第一个新建线程的发送端句柄。我们会将原始的通道发送端传递给第二个新建线程。这样就会有两个线程，每个线程将向通道的接收端发送不同的消息。