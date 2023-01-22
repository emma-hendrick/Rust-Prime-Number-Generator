// Anything over around 4800 primes will cause an overflow error
const NUM_PRIMES: i32 = 2500;
const MAX_STACK_DEPTH: i32 = 100;

// The main function is the entry point of the program
fn main() {

    let primes: Vec<i32> = generate_primes(NUM_PRIMES);

    for prime in primes.iter() {
        print!("{} ", prime);
    }

}

// Prime generator facade
fn generate_primes(num_primes_needed: i32) -> Vec<i32> {
    
    let mut primes: Vec<i32> = vec![2, 3];
    let mut prime_length: usize = primes.len();

    while prime_length != num_primes_needed as usize {
        primes.extend(iterate_primes(*primes.last().unwrap(), &primes, num_primes_needed, prime_length as i32, MAX_STACK_DEPTH));
        prime_length = primes.len();
    }

    return primes;

}

// Generate primes up to the square of the highest prime
fn iterate_primes(value: i32, primes_to_generate_from: &Vec<i32>, stop_count: i32, prime_length: i32, depth_remaining: i32) -> Vec<i32> {
    let max_prime: i32 = primes_to_generate_from.last().unwrap() * primes_to_generate_from.last().unwrap();

    // If we are about to leave the area we know this function will work
    if max_prime <= value || depth_remaining == 0 {
        return vec![];
    } 

    // If the current value is divisible by any of the primes
    else if primes_to_generate_from.iter().any(|x: &i32| ((value as f64) / (*x as f64)) % 1.0 == 0.0) {
        return iterate_primes(value + 2, primes_to_generate_from, stop_count, prime_length, depth_remaining - 1);
    }

    // If we have enough primes
    else if prime_length + 1 == stop_count {
        return vec![value];
    } 
    
    // Otherwise
    else {
        return [vec![value], iterate_primes(value + 2, primes_to_generate_from, stop_count, prime_length + 1, depth_remaining - 1)].concat();
    }
    
}