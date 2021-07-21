use num::complex::Complex;
use std::time::{Duration, Instant};
use std::io;


fn main() {
    print_complex();
    benchmark();

    let a = (String::new(), true);

    println!("{:?}", a);

    let mut input = String::new();    
    io::stdin().read_line(&mut input).expect("msg");
}

fn print_complex(){
 
    for real in 0 ..3  {
        
        for img in 0 .. 3 {
            let a = Complex::new(real as f32, img as f32); 
            let b = a.sin();
        
            println!("Hello, world! {} {}", a, b);
        }        
    }
}

fn benchmark() {
        let mut count = 0;
        let time_limit = Duration::new(0,1000);
        let start = Instant::now();
    
        while (Instant::now() - start) < time_limit {
            count += 1;
        }
        println!("{}", count);
    }

   

