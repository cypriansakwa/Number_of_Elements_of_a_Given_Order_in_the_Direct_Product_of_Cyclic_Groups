/// Compute the greatest common divisor (GCD) of two numbers.
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Compute the least common multiple (LCM) of two numbers.
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

/// Euler's Totient function (phi function) to count numbers less than `n` that are coprime with `n`.
fn euler_totient(n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;
    let mut num = n;
    
    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 {
        result -= result / num;
    }
    
    result
}

/// Function to compute the number of elements of a specific order in a cyclic group of order `n`.
fn num_elements_of_order_in_cyclic_group(n: u64, ord: u64) -> u64 {
    if n % ord == 0 {
        euler_totient(ord)
    } else {
        0
    }
}

/// Function to compute the number of elements of a given order in the product group Z_n x Z_m.
fn num_elements_of_order_in_product_group(n: u64, m: u64, order: u64) -> u64 {
    let mut count = 0;

    // Loop through possible orders in Z_n and Z_m
    for ord_a in 1..=n {
        if n % ord_a == 0 {
            for ord_b in 1..=m {
                if m % ord_b == 0 {
                    if lcm(ord_a, ord_b) == order {
                        count += num_elements_of_order_in_cyclic_group(n, ord_a) *
                                 num_elements_of_order_in_cyclic_group(m, ord_b);
                    }
                }
            }
        }
    }
    
    count
}

fn main() {
    let n = 84;
    let m = 126;
    let order = 14;
    
    let num_elements = num_elements_of_order_in_product_group(n, m, order);
    
    println!("The number of elements of order {} in Z_{} x Z_{} is: {}", order, n, m, num_elements);
}
