use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // [1]

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // [2]

        let guess: u32 = match guess.trim().parse() { // [5]
            Ok(mun) => mun,
            Err(_) => continue,
        }; // [6]

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // [3][4]
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

/*
1.
  变量默认不可变，使用 mut 使变量可变
  new() 是 String 类型的一个 关联函数。关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。一些语言中把它称为 静态方法 (static method)。

2.
  read_line 将用户输入附加到传递给它的字符串中，不过它也返回一个值——在这个例子中是 io::Result。Rust 标准库中有很多叫做 Result 的类型。一个 Result 泛型以及对应子模块的特定版本，比如 io::Result。
  Result 类型是 枚举（enumerations），通常也写作 enums。枚举类型持有固定集合的值，这些值被称为枚举的 成员（variants）。
  Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果。
  这些 Result 类型的作用是编码错误处理信息。Result 类型的值，像其他类型一样，拥有定义于其上的方法。io::Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。如果 io::Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。

3.
  cmp 方法用来比较两个值并可以在任何可比较的值上调用。它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较。 然后它会返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员。使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么。

4.
  一个 match 表达式由 分支（arms） 构成。一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。Rust 获取提供给 match 的值并挨个检查每个分支的模式。

5.
  Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值。这个功能常用在需要转换值类型之类的场景。它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类。

6.
  如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num，最后如愿变成新创建的 guess 变量。
  如果 parse 不 能将字符串转换为一个数字，它会返回一个包含更多错误信息的 Err。Err 值不能匹配第一个 match 分支的 Ok(num) 模式，但是会匹配第二个分支的 Err(_) 模式：_ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息。所以程序会执行第二个分支的动作，continue 意味着进入 loop 的下一次循环，请求另一个猜测。这样程序就有效的忽略了 parse 可能遇到的所有错误！
*/