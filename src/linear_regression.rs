pub fn predict(x: &Vec<f64>, y: &Vec<f64>, input_x: f64) -> f64{
    // Y = B0 + (B1 * X) + Error
    let b1: f64 = find_b1(&x, &y);
    let b0: f64 = find_b0(b1, &x, &y);
    let prediction = b0 + (b1 * input_x);
    prediction
}

fn find_b0(b1: f64, x: &Vec<f64>, y: &Vec<f64>) -> f64{
    let x_bar = mean(&x);
    let y_bar = mean(&y);

    let b0 = y_bar - (b1 * x_bar);
    println!("B0: {}", b0);
    b0
}

fn find_b1(x: &Vec<f64>, y: &Vec<f64>) -> f64{
    let x_bar = mean(&x);
    let y_bar = mean(&y);

    // Upper Sum
    let mut i = 0;
    let size = x.len();
    let mut sum: f64 = 0.0;
    while i < size{
        let a: f64 = x[i] - x_bar;
        let b: f64 = y[i] - y_bar;
        sum = sum + (a * b);
        i = i + 1;
    }

    // SSE
    let error: f64 = sse(&x);
    println!("B1: {}", (sum / error));
    sum / error
}

pub fn mean(x: &Vec<f64>) -> f64{
    x.iter().sum::<f64>() as f64 / x.len() as f64
}

pub fn sse(x: &Vec<f64>) -> f64{
    let x_bar = mean(&x);

    let mut sum: f64 = 0.0;
    for i in x.iter(){
        let mut a: f64 = i - x_bar;
        a = f64::powf(a, 2.0);
        sum = sum + a;
        println!("Sum at {}: {}",i, sum);
    }
    println!("SSE: {}", sum);
    sum
}