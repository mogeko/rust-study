#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    } // ok

    #[test]
    fn another() {
        panic!("Make this test fail");
    } // FAILED


    // 使用 assert! 宏来检查结果
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};

        assert!(larger.can_hold(&smaller));
    } // ok

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};

        assert!(!smaller.can_hold(&larger));
    } // ok


    // 使用 assert_eq! 和 assert_ne! 宏来测试相等
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    } // ok


    // 自定义失败信息
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    } // FAILED


    // 使用 should_panic 检查 panic
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    } // ok


    // 将 Result<T, E> 用于测试
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    } // ok
}


// 使用 assert! 宏来检查结果
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


// 使用 assert_eq! 和 assert_ne! 宏来测试相等
fn add_two(a: i32) -> i32 {
    a + 2
}


// 自定义失败信息
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}


// 使用 should_panic 检查 panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}