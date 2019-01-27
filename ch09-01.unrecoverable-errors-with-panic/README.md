# panic! 与不可恢复的错误

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch09-01-unrecoverable-errors-with-panic.html)

在遭遇**不可恢复的错误**时，程序会执行 `panic!` 宏。

> ## 对应 panic 时的栈展开或终止
> 当出现 panic 时，程序默认会开始**展开** (unwinding)，这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接**终止** (abort)，这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。如果你需要项目的最终二进制文件越小越好，panic 时通过在 *Cargo.toml* 的 `[profile]` 部分增加 `panic = 'abort'`，可以由展开切换为终止。例如，如果你想要在release模式中 panic 时直接终止：
> 
> ```toml
> [profile.release]
> panic = 'abort'
> ```

展开堆栈时可以用 `std::panic::catch_unwind` 捕获 `panic` 抛出的堆栈，但是极不推荐用这种方法来处理错误。`catch_unwind` 一般是用来在多线程程序里面在将挂掉的线程 catch 住，防止一个线程挂掉导致整个进程崩掉，或者是通过外部函数接口 (FFI) 与 C 交互时将堆栈信息兜住防止 C 程序看到堆栈不知道如何处理。另外并不是所有程序都能用 `catch_unwind` 捕捉，有的嵌入式平台上的程序受限于二进制文件大小的限制，`panic` 没有使用展开，而是使用终止的方式退出程序，这就没法兜得住了。

