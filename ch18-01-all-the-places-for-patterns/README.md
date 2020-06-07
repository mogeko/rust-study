# 所有可能会用到模式的位置

### `match` 分支

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

`match` 表达式必须是**穷尽** (exhaustive) 的，意为 `match` 表达式所有可能的值都必须被考虑到。

有一个特定的模式 `_` 可以匹配所有情况，不过它从不绑定任何变量。

### `if let` 条件表达式

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

主要用于编写等同于只关心一个情况的 `match` 语句简写的。

### `while let` 条件循环

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### `for` 循环

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

### `let` 语句

```rust
let PATTERN = EXPRESSION;
```

变量名不过是形式特别朴素的模式。

如果希望忽略元组中一个或多个值，也可以使用 `_` 或 `..`

### 函数参数

```rust
fn foo(x: i32) {
    // 代码
}
```

函数参数也可以是模式。

---

现在我们见过了很多使用模式的方式了，不过模式在每个使用它的地方并不以相同的方式工作；在一些地方，模式必须是 _irrefutable_ 的，意味着他们必须匹配所提供的任何值。在另一些情况，他们则可以是 _refutable_ 的。