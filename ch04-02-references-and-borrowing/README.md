# 引用与借用

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
```

**引用**允许你使用值但不获取其所有权

我们将获取引用作为函数参数称为**借用** (borrowing)

**（默认）不允许修改引用的值。**

## 可变引用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

可变的变量可以创建可变的引用，但是
- **在特定作用域中的特定数据有且只有一个可变引用**
- **我们也不能在拥有不可变引用的同时拥有可变引用**
