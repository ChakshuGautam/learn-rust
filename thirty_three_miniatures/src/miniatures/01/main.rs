extern crate num_bigint;
extern crate num_traits;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem::replace;
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

// Function to compute the nth Fibonacci number using the iterative method
fn fibonacci_iterative(n: u64) -> BigUint {
    if n == 0 {
        return Zero::zero();
    }
    if n == 1 {
        return One::one();
    }

    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let mut c: BigUint;

    for _ in 2..=n {
        c = &a + &b;
        a = replace(&mut b, c);
    }

    b
}

fn main() {
    // Array of numbers for which to calculate Fibonacci numbers
    let numbers = vec![10, 100, 500, 1000, 5000, 10_000, 50_000, 100_000, 500_000];
    println!(
        "{:<10} {:<20} {:<30}",
        "F(n)", "Time Taken Matrix (µs)", "Time Taken Iterative (µs)"
    );

    let file_name = "./src/miniatures/01/results.csv";
    let mut file = BufWriter::new(File::create(file_name).expect("Could not create file"));

    // Write the CSV headers
    writeln!(file, "n,Time Taken Matrix (µs),Time Taken Iterative (µs)")
        .expect("Could not write to file");

    for &n in &numbers {
        // Time the matrix method
        let start_matrix = Instant::now();
        let _fib_number_matrix = fibonacci(n + 1);
        let duration_matrix = start_matrix.elapsed();

        // Time the iterative method
        let start_iterative = Instant::now();
        let _fib_number_iterative = fibonacci_iterative(n);
        let duration_iterative = start_iterative.elapsed();

        assert_eq!(
            _fib_number_matrix, _fib_number_iterative,
            "The numbers are not equal"
        );

        // Write the time taken by both methods to the CSV file
        writeln!(
            file,
            "{},{},{}",
            n,
            duration_matrix.as_micros(),
            duration_iterative.as_micros()
        )
        .expect("Could not write to file");

        // Print the results in a table format
        println!(
            "{:<10} {:<20}   {:<30}",
            n,
            duration_matrix.as_micros(),
            duration_iterative.as_micros()
        );
    }

    // Ensure all data is flushed to the file
    file.flush().expect("Could not flush data to file");
}

// u128 - 340282366920938463463374607431768211455

/*
F(n)       Time Taken Matrix (µs) Time Taken Iterative (µs)
10         45                     2
100        32                     23
500        72                     131
1000       112                    298
5000       615                    2695
10000      1780                   8145
50000      19085                  142166
100000     55499                  536884
500000     543486                 13128766
*/
