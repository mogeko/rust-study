# 如何编写测试

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html)

测试函数体通常执行如下三种操作：

1. 设置任何所需的数据或状态
2. 运行需要测试的代码
3. 断言其结果是我们所期望的

## 新建一个测试

在 shell 中执行 `cargo new addr --lib` 新建一个库项目。

其中的 `src/lib.rs` 文件中会自动生成一个测试模块：

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        asser_eq!(2 + 2, 4);
    }
}
```

执行 `cargo test`，输出：

```shell
$ cargo test
   Compiling addr v0.1.0 (/home/mogeko/Project/rust-study/ch11-01.writing-tests)
    Finished dev [unoptimized + debuginfo] target(s) in 6.00s                   
     Running target/debug/deps/addr-6677ec2ba2578a17

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests addr

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

简单梳理一下各个部分：

- `test result: (ok / FAILED)`: 测试是否都通过
- `(int) passed`: 通过的测试数量
- `(int) failed`: 失败的测试数量
- `(int) ignored`: (通过标记) 忽略的测试数量
- `(int) filtered out`: 被过滤的测试数量；详情：[控制测试如何运行](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)
- `Doc-tests` 开头的这一部分是所有文档测试的结果；通过[文档注释](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)编写

## 使用 assert! 宏来检查结果

我们可以向 `assert!` 宏提供一个求值为布尔值的参数。

如果值是 `true`，`assert!` 什么也不做，同时测试会通过。

如果值为 `false`，`assert!` 调用 `panic!` 宏，这会导致测试失败。

## 使用 assert_eq! 和 assert_ne! 宏来测试相等

`assert_eq!` 用来比较两个值是否相等，如果**相等**则通过测试。

`assert_ne!` 用来比较两个值是否不相等，如果**不相等**则通过测试。

`assert_eq!` 和 `assert_ne!` 宏在底层分别使用了 `==` 和 `!=`。当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 `PartialEq` 和 `Debug` trait。(通常可以通过直接在结构体或枚举上添加 `#[derive(PartialEq, Debug)]` 注解来解决)

## 自定义失败信息

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}


pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```

输出：

```shell
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:47:9

failures:
    tests::greeting_contains_name
```

## 使用 `should_panic` 检查 panic

除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的。

对函数增加属性 `should_panic` ，这个属性在函数中的代码 panic 时会通过，而在其中的代码没有 panic 时失败。

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // 为了使 should_panic 测试结果更精确，我们加了一个可选的 expected 参数。
    // 测试工具会确保错误信息中包含其提供的文本。
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## 将 Result<T, E> 用于测试

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

**不能在对这些函数使用 `#[should_panic]`。**
