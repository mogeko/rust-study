# 无畏并发

如下是本章将要涉及到的内容：

- 何创建线程来同时运行多段代码。
- **消息传递** (Message passing) 并发，其中通道 (channel) 被用来在线程间传递消息。
- **共享状态** (Shared state) 并发，其中多个线程可以访问同一片数据。
- `Sync` 和 `Send` trait，将 Rust 的并发保证扩展到用户定义的以及标准库提供的类型中。