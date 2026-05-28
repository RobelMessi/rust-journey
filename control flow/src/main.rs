fn main() {
    let number = 7;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; //If true, the number is 5. Else, the number is 6
    println!("The value of number is {number}");


    let mut counter = 0;
    let result = loop { //result holds the value of counter
        counter+=1;

        if counter ==10 {
            break counter * 2;
        }
    }; //semi colon ends the statement that assigns the value to result
    println!("The value of the counter is {result}");

    //While Loops
    let mut number = 3;
    while number !=0{
        println!("{number}!");
        number-=1;
    }
    println!("LIFTOFF");

    //For Loops
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index<5{
        println!("The value is: {}", a[index]);
        index+=1
    }
    for element in a {
        println!("The value is: {element}");
    }
    
    //Count down loop, For
    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!")
}



