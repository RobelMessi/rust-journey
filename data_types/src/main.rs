fn main() {
    let _x = 6.2;
    //print!("{}",x);


    // addition
    let _sum= 5+10; //use underscore when you aren't using variable, compiler ignores it
    // subtraction
    let _difference = 95.5-4.3;
    // multiplication
    let _product = 4*30;
    // division
    let _quotient = 56.7/32.2;
    let _truncated = -5 / 3;
    // remainder
    let _remainder = 43 % 5;

    //println!("{}", remainder);

    // Booleans
    let _t = true;
    let _f: bool = false; //explicit type annotation


    //Chars
    let _c = 'z';
    let _z: char = 'Z'; // explicit type annotation
    let _heart_eyed_cat = '😻';
    //println!("{}", heart_eyed_cat);


    //Tuples (immutable)
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; //Breaks tuple into 3 parts
    println!("The value of y is {y}");

    let _x = (500, 6.4, 1);
    let _five_hundred = _x.0;
    let _six_point_four = _x.1;
    let _one = _x.2; //accesses each element of tuple with its indice

    //Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let a = [3; 5]; // let a = [3,3,3,3,3];
}
