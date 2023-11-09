extern crate num_bigint;
extern crate num_traits;

use std::fs::File;
use std::io::{Write, BufWriter};
use std::time::Instant;


use num_bigint::BigUint;
use num_traits::{One, Zero};

// Define a 2x2 matrix with BigUint elements
type Matrix2x2 = ((BigUint, BigUint), (BigUint, BigUint));

// Function to perform matrix multiplication
fn matrix_multiply(m1: Matrix2x2, m2: Matrix2x2) -> Matrix2x2 {
    (
        (
            &m1.0 .0 * &m2.0 .0 + &m1.0 .1 * &m2.1 .0,
            &m1.0 .0 * &m2.0 .1 + &m1.0 .1 * &m2.1 .1,
        ),
        (
            &m1.1 .0 * &m2.0 .0 + &m1.1 .1 * &m2.1 .0,
            &m1.1 .0 * &m2.0 .1 + &m1.1 .1 * &m2.1 .1,
        ),
    )
}

// Function to compute matrix to the power of n using exponentiation by squaring
fn matrix_power(mut base: Matrix2x2, mut exponent: u64) -> Matrix2x2 {
    let mut result: Matrix2x2 = ((One::one(), Zero::zero()), (Zero::zero(), One::one())); // Identity matrix
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = matrix_multiply(result, base.clone());
        }
        base = matrix_multiply(base.clone(), base.clone());
        exponent >>= 1; // equivalent to exponent /= 2 but for integers
    }
    result
}

// Function to compute the nth Fibonacci number using matrix exponentiation
fn fibonacci(n: u64) -> BigUint {
    if n == 0 {
        return Zero::zero();
    }
    let base: Matrix2x2 = ((One::one(), One::one()), (One::one(), Zero::zero()));
    let result = matrix_power(base, n - 1);
    result.0 .1 // The Fibonacci number is at position (0, 1) in the resulting matrix
}

fn main() {
    // let n = 1_000_000; // Calculate the 1000th Fibonacci number

    // let start = Instant::now(); // Start timing
    // let fib_number = fibonacci(n+1);
    // let duration = start.elapsed(); // End timing

    // println!("Fibonacci number F{} is {}", n, fib_number);
    // let mut file = File::create("fibonacci_result.txt").expect("Could not create file");
    // writeln!(file, "Fibonacci number F{} is {}", n, fib_number).expect("Could not write to file");

    // Array of numbers for which to calculate Fibonacci numbers
    let numbers = vec![10, 100, 500, 1000, 5000, 10_000, 50_000, 100_000, 500_000, 1_000_000, 5_000_000]; 
    println!("{:<10} {:<30}", "F(n)", "Time Taken (µs)");

    let _file_name = "fibonacci_numbers.csv";
    let mut file = BufWriter::new(File::create(_file_name).expect("Could not create file"));

    // Write the CSV headers
    writeln!(file, "n,Fibonacci Number,Time Taken (µs)").expect("Could not write to file");
    
    for &n in &numbers {
        let start = Instant::now(); // Start timing
        let fib_number = fibonacci(n);
        let duration = start.elapsed(); // End timing
        
        // Write the Fibonacci number and the time taken to the CSV file
        writeln!(file, "{},{},{}", n, fib_number, duration.as_micros()).expect("Could not write to file");
        
        // Print the result in a table format
        println!("{:<10} {:<30}", n, duration.as_micros());
    }

    // Ensure all data is flushed to the file
    file.flush().expect("Could not flush data to file");
}

// u128 - 340282366920938463463374607431768211455

/* F(n)       Time Taken (µs)               
10         47                            
100        34                            
500        74                            
1000       112                           
5000       644                           
10000      1772                          
50000      19054                         
100000     54917                         
500000     541974                        
1000000    1542715                       
5000000    25179289  
*/