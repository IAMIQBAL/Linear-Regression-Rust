mod linear_regression;
use std::io;

fn main() {

    // Input X values
    let mut size = String::new();

    println!("Enter Dataset size: ");
    io::stdin().read_line(&mut size).expect("Error occured");
    let size = match size.trim().parse(){
        Ok(size) => size,
        Err(_) => 1,
    };

    let mut x: Vec<f64> = Vec::with_capacity(size);
    let mut i = 0;

    println!("Enter X Values: ");

    while i < size{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error occured");

        let input:f64 = match input.trim().parse(){
            Ok(inp) => inp,
            Err(_) => -1.0,
        };
        x.push(input);
        i = i + 1;
    }

    i = 0;

    // Input Y values
    let mut y: Vec<f64> = Vec::with_capacity(size);

    println!("Enter Y Values: ");
    while i < size{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error occured");

        let input: f64 = match input.trim().parse(){
            Ok(inp) => inp,
            Err(_) => -1.0,
        };
        y.push(input);
        i = i + 1;
    }

    println!("X Values: {:?}", x);
    println!("Y Values: {:?}", y);

    println!("X Mean: {}", linear_regression::mean(&x));
    println!("Y Mean: {}", linear_regression::mean(&y));

    let mut pred_x = String::new();
    io::stdin().read_line(&mut pred_x).expect("Error");
    let pred_x: f64 = match pred_x.trim().parse() {
        Ok(pred_x) => pred_x,
        Err(_) => -1.0,
    };

    println!("Predicted Value for : {}", linear_regression::predict(&x, &y, pred_x));
}