# Result 与可恢复的错误

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)

定义：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## `unwrap` 和 `expect`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

如果程序执行成功 `unwrap` 或 `expect` 方法会将正确的值取出来，如果出错就直接让程序挂掉。其中 `unwrap` 方法在挂掉时会打印出标准库内置的错误信息，而 `exprct` 则让我们可以自己定义一个字符串在程序挂掉时显示。

## 使用 `match` 匹配 `Result`：

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

### 匹配不同的错误

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
```

如果觉得 `match` 太多也可以使用**闭包** (closure)：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}
```

[更多关于闭包的知识](https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html)

## 传播错误

Rust 允许程序像别的语言处理“异常”一样的将错误扔给更上一层的调用者，这被称为**传播** (propagating)错误。

如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

使用 `?` 简写：

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

`match` 表达式与问号运算符所做的有一点不同：`?` 所使用的错误值被传递给了 `from` 函数，它定义于标准库的 `From` trait 中，其用来将错误从一种类型转换为另一种类型。当 `?` 调用 `from` 函数时，收到的错误类型被转换为定义为当前函数返回的错误类型。这在当一个函数返回一个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。只要每一个错误类型都实现了 `from` 函数来定义如将其转换为返回的错误类型，`?` 会自动处理这些转换。

**`?` 只能被用于返回 `Result` 的函数**
