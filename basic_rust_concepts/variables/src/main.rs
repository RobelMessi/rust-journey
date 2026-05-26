
const HOW_MANY_SECONDS_IN_A_MIN: u32 = 60;
fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("{}", HOW_MANY_SECONDS_IN_A_MIN); //Need literal string b/c
    //it is a macro, the "println!"
    print_only_this();
}

fn print_only_this() {
    //Shadowing
    let x = 5;
    let x = x+1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x} ");
    }
    println!("The value of x is {x}");


}