# 数据类型

[教材原文](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html)

## 标量类型

**标量** (scalar) 类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

### 整型

|  长度  | 有符号  | 无符号  |
| :----: | :-----: | :-----: |
| 8-bit  |  `i8`   |  `u8`   |
| 16-bit |  `i16`  |  `u16`  |
| 32-bit |  `i32`  |  `u32`  |
| 64-bit |  `i64`  |  `u64`  |
|  arch  | `isize` | `usize` |

**Rust 默认的数据类型是 `i32`**

**有符号** 和 **无符号** 代表数字能否为负值

`isize` 和 `usize` 类型的长度依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

#### 整型字面值

|    数字字面值    |     例子      |
| :--------------: | :-----------: |
|     Decimal      |   `98_222`    |
|       Hex        |    `0xff`     |
|      Octal       |    `0o77`     |
|      Binary      | `0b1111_0000` |
| Byte (`u8` only) |    `b'A'`     |

除 byte 以外的所有数字字面值允许使用类型后缀，例如 `57u8`，同时也允许使用 `_` 做为分隔符以方便读数，例如`1_000`。

当在 debug 模式编译时，Rust 检查这类问题并使程序 *panic* (这个术语被 Rust 用来表明程序因错误而退出。[关于 panic 的更多细节](https://kaisery.github.io/trpl-zh-cn/ch09-00-error-handling.html))

在 release 构建中，Rust 不检测溢出，相反会进行一种被称为 “two’s complement wrapping” 的操作。简而言之，`256` 变成 `0`，`257` 变成 `1`，依此类推。

### 浮点型

`f32`: 32-bit, 单精度浮点数

`f64`: **(默认)** 64-bit, 双精度浮点数

### 布尔型

与其他大部分编程语言一样

### 字符型

Rust 的 `char` 类型代表了一个 Unicode 标量值（Unicode Scalar Value）

 `char` 由单引号指定，字符串使用双引号 ([关于字符串的更多细节](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html))

## 复合类型

**复合类型**（*Compound types*）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

### 元组类型

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1); // 定义一个元组，每一位的类型可以不同

let (x, y, z) = tup; // 解构元组

let a = tup.1; // 使用索引访问元组
```

### 数组类型

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]; // 定义一个数组，数组的数据类型必须相同
```

当你想要在栈（stack）而不是在堆（heap）上为数据分配空间 ([关于栈和堆的更多内容](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html))，或者是想要确保总是有固定数量的元素时，数组非常有用。但是数组并不如 vector 类型灵活。vector 类型是标准库提供的一个 **允许** 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 vector 的时候，你可能应该使用 vector。[关于 vector 的更多内容](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)。