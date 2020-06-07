# Refutability (可反驳性): 模式是否会匹配失效

模式有两种形式：refutable (可反驳的) 和 irrefutable (不可反驳的)。能匹配任何传递的可能值的模式被称为是**不可反驳的** (_irrefutable_)。一个例子就是 `let x = 5`; 语句中的 `x`，因为 `x` 可以匹配任何值所以不可能会失败。对某些可能的值进行匹配会失败的模式被称为是**可反驳的** (_refutable_)。一个这样的例子便是 `if let Some(x) = a_value` 表达式中的 `Some(x`)；如果变量 `a_value` 中的值是 `None` 而不是 `Some`，那么 `Some(x)` 模式不能匹配。

函数参数、`let` 语句和 `for` 循环只能接受不可反驳的模式

`if let` 和 `while let` 表达式被限制为只能接受可反驳的模式
