fn main() {
    let s = String::from("hello world 1 2");

    println!("{}", first_world(&s[..]));
}

fn first_world(s: &str) -> &str {
    let byte = s.as_bytes();

    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    } // iter 方法返回集合中的每一个元素
      // 而 enumerate 包装 iter 的结果并返回一个元组
      // 其中元组中的 i 是索引而元组中的 &item 是单个字节。

    &s[..]
}
