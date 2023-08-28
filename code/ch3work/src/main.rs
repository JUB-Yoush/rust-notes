use std::io;
fn main() {
    carol();
}

fn f_to_c() {
    /*
    get user input
    turn store as number
    convert number from f to c using a math function
    return the value
    */
    println!("please enter the farienheit value you'd like converted:");
    let mut value: String = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("ion know what that is");

    let value: f32 = value.trim().parse().expect("enter a number");
    println!("you entered {value}");
    let value_in_c = (value - 32.0) * 5.0 / 9.0; //everything must b float
    println!("the celsius equivalent is: {value_in_c}")
}

fn fibonachi(pos: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    if pos < 0 {
        println!("your input sucks");
        0
    } else if pos == 0 {
        1
    } else if pos == 1 {
        0
    } else {
        for i in 2..pos + 1 {
            c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

fn carol() {
    const lyrics: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    const days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for i in 0..=11 {
        let day = days[i];
        println!("on the {day} day of christmas like a true hater >:) I gave em:");
        for j in (0..i + 1).rev() {
            let output = lyrics[j];
            println!("{output}");
        }
        println!("---");
    }
}
