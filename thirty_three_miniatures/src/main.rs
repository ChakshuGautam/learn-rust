// Define a 2x2 matrix as a tuple of tuples
type Matrix2x2 = ((u128, u128), (u128, u128));

// Function to perform matrix multiplication with wrapping on overflow
fn matrix_multiply(m1: Matrix2x2, m2: Matrix2x2) -> Matrix2x2 {
    (
        (
            m1.0 .0.wrapping_mul(m2.0 .0).wrapping_add(m1.0 .1.wrapping_mul(m2.1 .0)),
            m1.0 .0.wrapping_mul(m2.0 .1).wrapping_add(m1.0 .1.wrapping_mul(m2.1 .1)),
        ),
        (
            m1.1 .0.wrapping_mul(m2.0 .0).wrapping_add(m1.1 .1.wrapping_mul(m2.1 .0)),
            m1.1 .0.wrapping_mul(m2.0 .1).wrapping_add(m1.1 .1.wrapping_mul(m2.1 .1)),
        ),
    )
}

// Function to compute matrix to the power of n using exponentiation by squaring
fn matrix_power(mut base: Matrix2x2, mut exponent: u128) -> Matrix2x2 {
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
fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let base: Matrix2x2 = ((1, 1), (1, 0));
    let result = matrix_power(base, n - 1);
    result.0 .1 // The Fibonacci number is at position (0, 1) in the resulting matrix
}

fn main() {
    let n = 183;
    println!("Fibonacci number F {} is {}", n, fibonacci(n+1));
}

// u128 - 340282366920938463463374607431768211455
// F180 - 18547707689471986212190138521399707760
// F182 - 48558529144435440119720805669229197641