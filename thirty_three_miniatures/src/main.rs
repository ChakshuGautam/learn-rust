// Define a 2x2 matrix as a tuple of tuples
type Matrix2x2 = ((u64, u64), (u64, u64));

// Function to perform matrix multiplication
fn matrix_multiply(m1: Matrix2x2, m2: Matrix2x2) -> Matrix2x2 {
    (
        (
            m1.0 .0 * m2.0 .0 + m1.0 .1 * m2.1 .0,
            m1.0 .0 * m2.0 .1 + m1.0 .1 * m2.1 .1,
        ),
        (
            m1.1 .0 * m2.0 .0 + m1.1 .1 * m2.1 .0,
            m1.1 .0 * m2.0 .1 + m1.1 .1 * m2.1 .1,
        ),
    )
}

// Function to compute matrix to the power of n using exponentiation by squaring
fn matrix_power(mut base: Matrix2x2, mut exponent: u64) -> Matrix2x2 {
    let mut result: Matrix2x2 = ((1, 0), (0, 1)); // Identity matrix
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = matrix_multiply(result, base);
        }
        base = matrix_multiply(base, base);
        exponent /= 2;
    }
    result
}

// Function to compute the nth Fibonacci number using matrix exponentiation
fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let base: Matrix2x2 = ((1, 1), (1, 0));
    let result = matrix_power(base, n - 1);
    result.0 .1 // The Fibonacci number is at position (0, 1) in the resulting matrix
}

fn main() {
    let n = 70;
    println!("Fibonacci number F {} is {}", n, fibonacci(n));
}


// Attempt 1
// thread 'main' panicked at src/main.rs:8:13:
// attempt to multiply with overflow