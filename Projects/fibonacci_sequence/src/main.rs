fn main() {
    let n = 21;
    let result = fibonacci_sequence(n);
    println!("The result of the {n}th sequence is {}", result);
}

fn fibonacci_sequence(a:i32) -> i32{
    if a <=1{
        a
    } else {
        fibonacci_sequence(a-1) + fibonacci_sequence(a-2) //no semi colon when evaluating expression
    }
}

