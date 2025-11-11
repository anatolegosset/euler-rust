use std::collections::{HashMap, HashSet};

/// Returns a Vec<usize> result of size n such that result[i] is the smallest prime divisor of i
/// for all i such that 2 <= i <= n.
fn smallest_prime_factors(n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = (0..(n + 1)).collect();
    for i in 2..((n as f64).sqrt() as usize + 1) {
        if result[i] == i {
            for j in ((i * i)..(n + 1)).step_by(i) {
                if result[j] == j {
                    result[j] = i;
                }
            }
        }
    }
    result
}

/// Returns a HashMap<usize, usize> result such that p is in result if p is a prime divisor of k
/// and result[p] is the exponent of p in the prime decomposition of k.
/// spf is a vec containing the smallest prime factors (see fn smallest_prime_factors()) and
/// must have spf.len() >= k
fn prime_factorization(mut k: usize, spf: &Vec<usize>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();

    while k > 1 {
        let p = spf[k];
        *result.entry(p).or_insert(0) += 1;
        k /= p;
    }

    result
}

/// From an usize k, and a vec of smallest prime factors spf, returns a Vec<usize> result that
/// contains all divisors of k ** 2 that are < k.
fn small_square_divisors(k: usize, spf: &Vec<usize>) -> Vec<usize> {
    let mut divisors: Vec<usize> = vec![1];
    let factors = prime_factorization(k, spf);

    for (prime, exponent) in factors.into_iter() {
        let mut new_divisors: Vec<usize> = Vec::new();

        for divisor in divisors.into_iter() {
            for i in 0..=(2 * exponent) {
                let new_div = divisor * (prime.pow(i as u32));
                if new_div < k {
                    new_divisors.push(new_div);
                }
            }
        }

        divisors = new_divisors
    }
    divisors
}

fn main() {
    const N: usize = 1_000_000_000_000;

    let root = (N as f64).sqrt() as usize;
    let spf = smallest_prime_factors(root);

    let mut relevant_squares: HashSet<usize> = HashSet::new();

    for i in 1..=root {
        relevant_squares.insert(i * i);
    }

    let mut total: usize = 0;

    for i in 1..=root {
        let square = i * i;

        let divisors = small_square_divisors(i, &spf);
        for divisor in divisors {
            let quotient = square / divisor;
            let to_check = divisor + i * quotient;
            if relevant_squares.contains(&to_check) {
                total += to_check;
                relevant_squares.remove(&to_check);
            }
            let to_check = i + divisor * quotient;
            if relevant_squares.contains(&to_check) {
                total += to_check;
                relevant_squares.remove(&to_check);
            }
        }
    }

    println!("{}", total)
}

