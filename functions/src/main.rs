
fn five() -> i32{
    5 //This is the functions return value
}

fn main() {
    println!("Hello, world!");
    let y = {
        let x = 3;
        x+1 // value gets bounded to y, not the no semicolon: expressions do not include ending semicolons
    };
    println!("The value of y is {y}");

    another_function(5);
    print_labeled_measurement(5, 'm');

    let x = five();
    println!("The value of x is {x}")

}

fn another_function(x: i32) { //i32 means integer, 32 bits
    println!("The value of x is {x}");
}
fn print_labeled_measurement(value: i32, unit_label:char){
    println!("The measurement is {value} {unit_label}");
}

//Expressions evaluate to a resultant value
//Statements are instructions that perform some action and do not return a value