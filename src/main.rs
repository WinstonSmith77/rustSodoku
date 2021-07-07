use num::complex::Complex;


fn main() {

    for real in 0 .. 10  {
        
        for img in 0 .. 10 {
            let a = Complex::new(real as f32, img as f32); 
            let b = a.sin();
        
            println!("Hello, world! {} {}", a, b);
        }        
    }

   
}
