

fn true_rng() {
    // Might to this at some point!
    return;
}

/// Generates a pseudo random number (or numbers).
/// 
/// # Description
/// This is an Implementation of a multiplicative linear 
/// congruential generator with bounds and a seed made from
/// the current system clock time.
/// * There aren't default params in Rust inherently
/// 
/// # Arguments
/// * 'n' - The length of desired vector
/// * 'lb' - Lower bound
/// * 'ub' - Upper Bound
/// 
/// # Hardcoded Algo Parameters (In Order)
/// * 'a' - The multiplier
/// * 'seed' - The start value, and for multiple numbers generated the next seed
/// * 'c' - The increment
/// * 'm' - The modulus
fn pseudo_rng(n:u128, lb:u128, ub:u128) -> Vec<u128> {
    let mut results : Vec<u128> = Vec::new();
    for _ in 0..n {
        let seed = lb + ((22695477 * std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .ok().unwrap().as_nanos() + 1) % 4294967296) % (ub-lb+1);
        results.push(seed);
    }
    return results;
    }

fn main() {
    true_rng();

    let v : Vec<u128> = pseudo_rng(1, 0, 10); // m is 2^32
    println!("{:?}", v);
}