# Slines

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html)

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..=10];
```

![trpl04-06](https://kaisery.github.io/trpl-zh-cn/img/trpl04-06.svg)

**字符串字面值 ("") 就是 slines**

## 其他类型的 slines

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

这个 slice 的类型是 `&[i32]`。

它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。

[更多关于集合的细节](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)
