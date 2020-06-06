# 面向对象设计模式的实现

**状态模式** (_state pattern_)是一个面向对象设计模式。该模式的关键在于一个值有某些内部状态，体现为一系列的**状态对象**，同时值的行为随着其内部状态而改变。状态对象共享功能 (在 Rust 中使用结构体和 trait 而不是对象和继承)。每一个状态对象代表负责其自身的行为和当需要改变为另一个状态时的规则的状态。持有任何一个这种状态对象的值对于不同状态的行为以及何时状态转移毫不知情。

为了探索这个概念，我们将实现一个增量式的发布博文的工作流。这个博客的最终功能看起来像这样：

1. 博文从空白的草案开始。
2. 一旦草案完成，请求审核博文。
3. 一旦博文过审，它将被发表。
4. 只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。

`src/main.rs`:
```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

```

`src/lib.rs`:
```rust
pub struct Post {
    // 定义 Post
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self, post: &'a Post) {}
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

impl Post {
    // 新建一个草案状态的实例
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    // 存放博文内容的文本
    pub fn add_text(&mut self, text: &str) {
        self.state.as_ref().unwrap().add_text(self);
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self.state = Some(s.reject())
    }
    pub fn content(&self) -> &str {
        // 这里调用 Option 的 as_ref 方法是因为需要 Option 中值的引用而不是获取其所有权。
        // 因为 state 是一个 Option<Box<State>>，调用 as_ref 会返回一个 Option<&Box<State>>。
        // 如果不调用 as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数。
        self.state.as_ref().unwrap().content(self)
    }
}


struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self, post: &'a Post) -> &'a str {
        &post.content.push_str(text);
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Deaft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Deaft {})
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

```


## 状态模式的权衡取舍

我们展示了 Rust 是能够实现面向对象的状态模式的，以便能根据博文所处的状态来封装不同类型的行为。`Post` 的方法并不知道这些不同类型的行为。通过这种组织代码的方式，要找到所有已发布博文的不同行为只需查看一处代码：`Published` 的 `State` trait 的实现。

如果要创建一个不使用状态模式的替代实现，则可能会在 `Post` 的方法中，或者甚至于在 `main` 代码中用到 `match` 语句，来检查博文状态并在这里改变其行为。这意味着需要查看很多位置来理解处于发布状态的博文的所有逻辑！这在增加更多状态时会变得更糟：每一个 `match` 语句都会需要另一个分支。

对于状态模式来说，`Post` 的方法和使用 `Post` 的位置无需 `match` 语句，同时增加新状态只涉及到增加一个新 `struct` 和为其实现 trait 的方法。

这个实现易于扩展增加更多功能。为了体会使用此模式维护代码的简洁性，请尝试如下一些建议：

- 增加 `reject` 方法将博文的状态从 `PendingReview` 变回 `Draft`
- 在将状态变为 `Published` 之前需要两次 `approve` 调用
- 只允许博文处于 `Draft` 状态时增加文本内容。提示：让状态对象负责什么可能会修改内容而不负责修改 `Post`。

状态模式的一个缺点是因为状态实现了状态之间的转换，一些状态会相互联系。如果在 `PendingReview` 和 `Published` 之间增加另一个状态，比如 `Scheduled`，则不得不修改 `PendingReview` 中的代码来转移到 `Scheduled`。如果 `PendingReview` 无需因为新增的状态而改变就更好了，不过这意味着切换到另一种设计模式。

另一个缺点是我们会发现一些重复的逻辑。为了消除他们，可以尝试为 `State` trait 中返回 `self` 的 `request_review` 和 `approve` 方法增加默认实现，不过这会违反对象安全性，因为 trait 不知道 `self` 具体是什么。我们希望能够将 `State` 作为一个 trait 对象，所以需要其方法是对象安全的。

另一个重复是 `Post` 中 `request_review` 和 `approve` 这两个类似的实现。他们都委托调用了 `state` 字段中 `Option` 值的同一方法，并在结果中为 `state` 字段设置了新值。如果 `Post` 中的很多方法都遵循这个模式，我们可能会考虑定义一个宏来消除重复

完全按照面向对象语言的定义实现这个模式并没有尽可能地利用 Rust 的优势。

### 将状态和行为编码为类型

不同于完全封装状态和状态转移使得外部代码对其毫不知情，我们将状态编码进不同的类型。

如此，Rust 的类型检查就会将任何在只能使用发布博文的地方使用草案博文的尝试变为编译时错误。

`src/lib.rs`:
```rust
// `Post 和 DraftPost 结构体都有一个私有的 content 字段来储存博文的文本。
// 这些结构体不再有 `state` 字段因为我们将状态编码改为结构体类型。
pub struct Post {
    content: String,
}
pub struct DraftPost {
    content: String,
}
pub struct PendingReviewPost {
    content: String,
}

// Post 将代表发布的博文，它有一个返回 content 的 content 方法。
impl Post {
    pub fn new() -> DraftPost {
        // 仍然有一个 Post::new 函数，不过不同于返回 Post 实例，它返回 DraftPost 的实例。
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // DraftPost 上定义了一个 add_text 方法，这样就可以像之前那样向 content 增加文本
    // 不过注意 DraftPost 并没有定义 content 方法！
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            // request_review 方法获取 self 的所有权，因此会消费 DraftPost 实例，并转换为 PendingReviewPost
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        // 与 request_review 同理
        // 消费 PendingReviewPost 实例，并转换为发布的 Post
        Post {
            content: self.content,
        }
    }
}
```

这也意味着不得不对 `main` 做出一些小的修改。

因为 `request_review` 和 `approve` 返回新实例而不是修改被调用的结构体，所以我们需要增加更多的 `let post =` 覆盖赋值来保存返回的实例。

`src/main.rs`:
```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

```

不得不修改 `main` 来重新赋值 `post` 使得这个实现不再完全遵守面向对象的状态模式：状态间的转换不再完全封装在 `Post` 实现中。然而，得益于类型系统和编译时类型检查，我们得到了的是无效状态是不可能的！这确保了某些特定的 bug，比如显示未发布博文的内容，将在部署到生产环境之前被发现。

即便 Rust 能够实现面向对象设计模式，也有其他像将状态编码进类型这样的模式存在。这些模式有着不同的权衡取舍。虽然你可能非常熟悉面向对象模式，重新思考这些问题来利用 Rust 提供的像在编译时避免一些 bug 这样有益功能。在 Rust 中面向对象模式并不总是最好的解决方案，因为 Rust 拥有像所有权这样的面向对象语言所没有的功能。