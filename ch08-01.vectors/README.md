# vector 用来储存一系列的值

[教程原文](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)

vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。

vector 只能储存相同类型的值。

vector 是用泛型实现的。

## 新建 vector

```rust
let v1: vec<i32> = Vec::new(); // Rust 无法推断出放入值的类型，需要手动注解出来

let v2 = vec![1, 2, 3];
```

## 更新 vector

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

使用 `mut` 让 `v` 可变。

使用 `push` 方法在 vector ，中增加一个新的元素。移除 vector 中最后一个元素用 `pop` 方法。

Rust 能够推断出放入的值都是 `i32` 类型，所以不需要 `Vec<i32>` 注解。



## 读取 vector 的元素

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // 使用引用读取 vector 中的元素
println!("The third element is {}", third);

match v.get(2) { // 使用 get 方法读取 vector 中的元素
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

使用 `&` 与 `[]` 返回一个引用

使用 `get` 方法返回一个 `Option<&T>`


> 不能在相同作用域中同时存在可变和不可变引用

当我们获取了 vector 的第一个元素的不可变引用并尝试在 vector 末尾增加一个元素的时候，这是行不通的

不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

## 遍历 vector

```rust
let mut v = vec![100, 32, 57];


for i in &mut v {
    *i += 50;
}
```

使用 `+=` 运算符之前必须使用解引用运算符 (`*`) 获取 `i` 中的值。

[ 关于 `*` 的更多细节](https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html)


## 使用枚举来储存多种类型

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

