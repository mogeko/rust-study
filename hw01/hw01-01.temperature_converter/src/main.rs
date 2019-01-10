use std::io;

fn main() {
    println!("Do you want to turn °C to °F or °F to °C?");
    println!("  1. °C to °F");
    println!("  2. °F to °C");

    let cover_mode = get_input();
    if cover_mode == 1f64 {
        println!("What is the temperature?(°C)");
        let temperature = (get_input() * 5f64) / 9f64 + 32f64;
        println!("The converted temperature is {:.2} °F", temperature);
    } else if cover_mode == 2f64 {
        println!("What is the temperature?(°F)");
        let temperature = (5f64 / 9f64) * (get_input() - 32f64);
        println!("The converted temperature is {:.2} °C", temperature);        
    } else {
        println!("This is not a valid input");
    }
}

fn get_input() -> f64 {
    // 接收用户输入
    let mut usr_input = String::new();
    io::stdin().read_line(&mut usr_input)
        .expect("This is not a valid input");
    let usr_input: f64 = usr_input.trim().parse()
        .expect("This is not a valid input");
    usr_input
}
