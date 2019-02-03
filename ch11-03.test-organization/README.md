# 测试的组织结构

[教程原文](https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html)

测试的主要分为两类：**单元测试** (*unit tests*) 和 **集成测试** (*integration tests*)。


单元测试倾向于一次测试一个模块 (函数)，或者是测试私有接口，是最基本的测试。而集成测试对于你的库来说则完全是外部的，只能调用一些公有的 API，主要是测试库的多个部分能否一起正常工作。

## 单元测试

单元测试与他们要测试的代码共同存放在位于 *src* 目录下相同的文件中。规范是在每个文件中创建包含测试函数的 `tests` 模块，并使用 `#[cfg(test)]` 标注模块。

```rust
// 功能代码 (被测试的代码)
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}



// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    } // 单元测试可以测私有的函数
}
```

## 集成测试

**集成测试无法测试二进制 crate，也就是只包含 *src/main.rs* 的项目** (因为需要使用 `extern crate` 导入 crate，而二进制 crate 无法这么做)

集成测试需要在项目根目录创建一个 *tests* 目录，与 *src* 同级。接着可以随意在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译。

集成测试中不需要创建名为 `tests` 的子模块，也不需要 `#[cfg(test)]` 标注。

假设需要被测试的 crate 名为 `adder`：

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

### 运行单独的测试文件

被测文件与测试文件是 1 对 n (n >= 1) 的关系。

可以使用 `cargo test --test` 后跟文件的名称来运行某个特定集成测试文件中的所有测试：

```shell
$ cargo test --test integration_test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### 集成测试中的子模块

我们想在集成测试中创建一个名为 `common` 的子模块方便 crate 与 crate 之间共享代码。我们可以创建文件 *tests/common/mod.rs*：

```rust
pub fn setup() {
    // 编写特定库测试所需的代码
}
```

然后在需要调用的 crate 中使用 `mod common;` 调用：

```rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

## 关于涉及到 I/O 的测试

涉及到 I/O 的程序是很难进行测试的，所以应当尽量避免。

可以通过一下思路优化程序：

- 让函数输出需要进行 I/O 操作的文件的路径比让函数直接操作文件更好
- 如果需要从文件中获取数据，应当在 (集成测试的) 测试目录中设定一组用于测试的特殊文件，它们始终处于未改变的状态
- 如果不得不对文件进行操作，使用 Mock
