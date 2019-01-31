# 控制测试如何运行

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

```shell
cargo test [OPTIONS] [TESTNAME] [-- <args>...]
```

可以将一部分命令行参数传递给 `cargo test`，而将另外一部分传递给生成的测试二进制文件。

为了分隔这两种参数，需要先列出传递给 `cargo test` 的参数，接着是分隔符 `--`，再之后是传递给测试二进制文件的参数。

运行 `cargo test --help` 可以查看与 `cargo test` 相关的帮助文档。而运行 `cargo test -- --help` 可以提示在分隔符 `--` 之后使用的有关参数。

## 记几个在 `--` 之后使用的参数

记几个比较重要的在 `--` 之后使用的参数：

- `--test-threads=<int>`: 跑测试时并发的线程数。
- `--nocapture`: 显示 (通过测试的程序的) 标准输出信息。
- `--ignored`: 只运行被标记为*忽略* (`#[ignore]`) 的测试。

## 通过指定名字来运行部分测试

可以通过向 `cargo test` 传递某个具体测试的名称来运行特定的测试：

```shell
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

也可以传递一部分名称，此时 rust 会运行包含此名称的所以测试：

```shell
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```


