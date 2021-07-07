use num::complex::Complex;
use std::time::{Duration, Instant};


fn main() {

    for real in 0 .. 3  {
        
        for img in 0 .. 3 {
            let a = Complex::new(real as f32, img as f32); 
            let b = a.sin();
        
            println!("Hello, world! {} {}", a, b);
        }        
    }

    benchmark();
}

fn benchmark() {
        let mut count = 0;
        let time_limit = Duration::new(1,0);
        let start = Instant::now();
    
        while (Instant::now() - start) < time_limit {
            count += 1;
        }
        println!("{}", count);
    }

   

