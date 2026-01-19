use num_integer::gcd;
use std::env;
use std::num::ParseIntError;
use text_io::read;

fn get_value() -> Vec<u128> {
    loop {
        print!("Please input integer value(s):\t");

        let value: String = read!("{}\n");

        let result: Result<Vec<u128>, _> = value
            .split_whitespace()
            .map(|x| x.parse::<u128>())
            .collect();

        match result {
            Ok(v) => {
                if !v.is_empty() {
                    return v;
                }

                println!("No values provided");
            }
            Err(_) => {
                println!("Invalid input: {}", value);
            }
        }
    }
}

fn get_args() -> Result<Vec<u128>, ParseIntError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Ok(get_value());
    }

    args[1..].iter().map(|arg| arg.parse()).collect()
}

fn totient_sieve(n: u128) -> Vec<u128> {
    let mut phi = vec![0; (n + 1) as usize];
    for i in 1..=n {
        phi[i as usize] = i;
    }

    for i in 2..=n {
        if phi[i as usize] == i {
            for j in (i..=n).step_by(i as usize) {
                phi[j as usize] -= phi[j as usize] / i;
            }
        }
    }

    phi
}

fn sums(phi: &[u128]) -> (Vec<u128>, Vec<u128>) {
    let n = phi.len() - 1;
    let mut sum_phi = vec![0; n + 1];
    let mut sum_k_phi = vec![0; n + 1];

    for i in 1..=n {
        sum_phi[i] = sum_phi[i - 1] + phi[i];
        sum_k_phi[i] = sum_k_phi[i - 1] + (i as u128) * phi[i];
    }

    (sum_phi, sum_k_phi)
}



fn run_calculation(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    let phi = totient_sieve(n);

    let (prefix_sum_phi, prefix_sum_k_phi) = sums(&phi);

    let mut ans = 0;
    let limit = (n - 1) / 2;

    for g in 1..=limit {
        let m = (n - 1) / g;

        if m < 2 {
            continue;
        }

        let sum_phi = prefix_sum_phi[m as usize] - prefix_sum_phi[1];
        let sum_k_phi = prefix_sum_k_phi[m as usize] - prefix_sum_k_phi[1];

        let d = gcd(g, n);
        let factor = g / d;

        let t1 = n * sum_phi;
        let t2 = g * sum_k_phi;
        if t2 > t1 {
            panic!("Negative term");
        }

        ans += (t1 - t2) * factor;
    }

    ans
}

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Invalid input, unable to parse arguments: {}", e);
            return;
        }
    };

    if args.is_empty() {
        println!("No input values provided");
        return;
    }

    if args.len() > 1 {
        println!("Number of arguments: {}", args.len());
    }
    for n in args.iter() {
        println!();
        println!("Input value:\t{}", n);
        let l: f64 = (*n as f64).log10();
        let ilog = n.ilog10();
        let diff = (l - ilog as f64).abs();
        if diff < 1e-10 {
            println!("Values:\t\t10^{}", ilog);
        }
        let result = run_calculation(*n);

        println!("Output value:\t{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_integer::lcm;

    // Reference method, scales as O(N^2)
    fn reference(n: u128) -> u128 {
        let mut sum = 0;

        for a in 1..=n {
            let lim = n - a;
            for b in 1..=lim {
                let c = n - a - b;
                if c > 0 {
                    let g = gcd(a, b);
                    let l = lcm(c, g);
                    sum += l;
                }
            }
        }

        sum
    }

    #[test]
    fn test_prefix_sums() {
        let test_values = vec![0, 1, 2, 10, 100, 111, 9, 219];

        for n in test_values {
            println!("Input value:\t\t{}", n);
            let ref_val = reference(n);
            println!("Reference Value:\t{}", ref_val);
            let val = run_calculation(n);
            println!("Output value:\t\t{}", val);

            assert_eq!(val, ref_val);
        }
    }
}
