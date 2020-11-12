pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut current_number: u32 = 2;

    while primes.len() <= n as usize {
        let mut is_prime = true;
        current_number += 1;
        for x in 0..primes.len() - 1 {
            let p = primes[x];
            if current_number % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(current_number)
        }
    }

    return primes[n as usize];
}
