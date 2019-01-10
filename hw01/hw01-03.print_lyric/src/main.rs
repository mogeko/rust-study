fn main() {
    let lyric = [
        "and a partridge in a pear tree",
        "two turtle doves, ",
        "three French hens, ",
        "four calling birds, ",
        "five gold rings, ",
        "six geese a laying, ",
        "seven swans a swimming, ",
        "eight maids a milking, ",
        "nine, drummers drumming, ",
        "ten pipers piping, ",
        "eleven ladies dancing, ",
        "twelve Lords a leaping, "
    ];

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let mut x = 1;
    while x <= 12 {
        println!(
            "On the {} day of Christmas my true love sent to me", days[x - 1]
        );
        let mut y = x;
        if x == 1 {
            println!("a partridge in a pear tree");
        } else {
            while y > 0 {
                print!("{}", lyric[y - 1]);
                y = y - 1;
            }
        }
        println!("");
        println!("");
        x = x + 1;
    }
}
