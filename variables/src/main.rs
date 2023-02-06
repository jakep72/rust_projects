use std::io;

fn main() {
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // let mut spaces = "     ";
    // spaces = spaces.len()

    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("The guess is: {guess}");

    // let x: f64 = 2.0; // f64

    // let y: f64 = 3.0; // f32

    // let z = x*y;

    // println!("The product is: {z}");

    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;

    // let product = 4*30;

    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;

    // let remainder = 43 % 5;

    // println!("{sum},{difference},{product},{quotient},{truncated},{remainder}");

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // println!("{c},{z},{heart_eyed_cat}");

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let (x,y,z) = tup;

    // let x0 = tup.0;

    // let x1 = tup.1;

    // let x2 = tup.2;

    // println!("{x0},{x1},{x2}");
    
    // let a = [1, 2, 3, 4, 5];



    // let months = ["January", "February", "March", "April", "May", "June", "July",
    //           "August", "September", "October", "November", "December"];

    
    // let b = [3;5];

    // let a0 = a[0];

    // let a1 = a[1];

    let a = [1, 2, 3, 4, 5];

    let size = a.len();

    loop {
        println!("Please enter an array index less than {size}.");

        let mut index = String::new();

        io::stdin().read_line(&mut index).expect("Failed to read line");

        // let index: usize = index.trim().parse().expect("Index entered was not a number");
        
        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if index == 100{
            println!("Goodbye!");
            break;
        };

        if index >= size{
            println!("Index out of bounds!");
            continue;
        };


        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }
}
