# 编写 猜猜看 游戏

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html)

## 对 Result 的理解

```rust
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

// --snip--
```

> 之前提到了 `read_line` 将用户输入附加到传递给它的字符串中，不过它也返回一个值——在这个例子中是 [`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html)。Rust 标准库中有很多叫做 `Result` 的类型。一个 [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) 泛型以及对应子模块的特定版本，比如 `io::Result`。
>
> `Result` 类型是 [*枚举*（*enumerations*）](https://kaisery.github.io/trpl-zh-cn/ch06-00-enums.html)，通常也写作 *enums*。枚举类型持有固定集合的值，这些值被称为枚举的 **成员**（*variants*）。第六章将介绍枚举的更多细节。
>
> `Result` 的成员是 `Ok` 和 `Err`，`Ok` 成员表示操作成功，内部包含成功时产生的值。`Err` 成员则意味着操作失败，并且包含失败的前因后果。

我们都知道"薛定谔的猫"——它处于一种活与不活的叠加态——`Result` 就处于这么一种类似的状态，它即可能是活的 (`Ok`) 也可能是死的 (`Err`)。

我们可以通过 `match` 或者 `.expect()` 来"观测"它，并根据具体的状态做进一步的操作。

不光只有 `Result` 如此，其他的枚举也可以这么理解：处于一种叠加态

[关于枚举的更多细节](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

## 对 match 的理解

**Example 1**

```rust
// --snip--

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}

// --snip--
```

> `cmp` 方法用来比较两个值并可以在任何可比较的值上调用。它获取一个被比较值的引用：这里是把 `guess` 与 `secret_number` 做比较。 然后它会返回一个刚才通过 `use` 引入作用域的 `Ordering` 枚举的成员。使用一个 [`match`](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html) 表达式，根据对 `guess` 和 `secret_number` 调用 `cmp` 返回的 `Ordering` 成员来决定接下来做什么。
>
> 一个 `match` 表达式由 **分支（arms）** 构成。一个分支包含一个 **模式**（*pattern*）和表达式开头的值与分支模式相匹配时应该执行的代码。Rust 获取提供给 `match` 的值并挨个检查每个分支的模式。`match` 结构和模式是 Rust 中强大的功能，它体现了代码可能遇到的多种情形，并帮助你确保没有遗漏处理。这些功能将分别在第六章和第十八章详细介绍。

**Example 2**

```rust
// --snip--

io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

println!("You guessed: {}", guess);

// --snip--
```

> 将 `expect` 调用换成 `match` 语句，是从遇到错误就崩溃转换到真正处理错误的惯用方法。须知 `parse` 返回一个 `Result` 类型，而 `Result` 是一个拥有 `Ok` 或 `Err` 成员的枚举。这里使用的 `match` 表达式，和之前处理 `cmp` 方法返回 `Ordering` 时用的一样。
>
> 如果 `parse` 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 `Ok`。这个 `Ok` 值与 `match` 第一个分支的模式相匹配，该分支对应的动作返回 `Ok` 值中的数字 `num`，最后如愿变成新创建的 `guess` 变量。
>
> 如果 `parse` *不* 能将字符串转换为一个数字，它会返回一个包含更多错误信息的 `Err`。`Err` 值不能匹配第一个 `match` 分支的 `Ok(num)` 模式，但是会匹配第二个分支的 `Err(_)` 模式：`_` 是一个通配符值，本例中用来匹配所有 `Err` 值，不管其中有何种信息。所以程序会执行第二个分支的动作，`continue` 意味着进入 `loop` 的下一次循环，请求另一个猜测。这样程序就有效的忽略了 `parse` 可能遇到的所有错误！

`match` 将"观测"一个"处于叠加态"的枚举成员，将它的具体状态与 `{}` 内的状态做比较，如果匹配就执行 `=>` 后的语句。

**Example 1** 中 `match` 接收的枚举来自于 `guess.cmp(&secret_number)`

**Example 1** 中 `match` 接收的枚举来自于 `guess.trim().parse()`，如果匹配到 `Ok` 则会将其中的值 (`num`) 赋值给 `guess: u32` 否则执行 `continue` 跳过这一轮循环。

关于 `match` 的更多细节：[第六章](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)&emsp;&emsp;[第十八章](https://kaisery.github.io/trpl-zh-cn/ch18-00-patterns.html)