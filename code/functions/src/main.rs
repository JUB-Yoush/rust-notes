/*
fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurements(5,'h');
}

fn another_function(x:i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value:i32,unit_label:char){
    println!("the measurement is: {value}{unit_label}")
}
*/
/*
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
*/
/*
fn five() -> i32 { // binds whatever calls it to the return value
    5
}

fn main() {
    let x = five(); // bind var to five()'s return value

    println!("The value of x is: {x}");
}
*/
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
