#![feature(test)]
use std::ops::Div;


pub fn nth(n: u32) -> u32 {
    nth_solution_four(n)
}

pub fn nth_solution_one(n: u32) -> u32 {
    let mut known_primes: Vec<u32> = vec![2];

    for x in 3.. {
        if known_primes.len() == (n + 1) as usize {
            break;
        }

        if known_primes.iter().any(|y| x % y == 0) == false {
            known_primes.push(x);
        }
    }

    *known_primes.last().unwrap()
}

pub fn nth_solution_two(n: u32) -> u32 {
   let mut known_primes: Vec<u32> = vec![];
   (2..).find(|x| {
       if known_primes.iter().any(|y| x % y == 0) {
           false
       } else {
           known_primes.push(*x);
           known_primes.get(n as usize).is_some()
       }
   }).unwrap()
}

pub fn nth_solution_three(n: u32) -> u32 {
    (2..)
        .filter(|x| !(2..x - 1).any(|y| x % y == 0))
        .nth(n as usize)
        .unwrap()
}

pub fn nth_solution_four(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .unwrap()
}

pub fn is_prime(n: u32) -> bool {
    let x = (n as f32).div(2.).ceil() as u32;
    !(2..x + 1).any(|y| n % y == 0)
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_nth_solution_one(b: &mut Bencher) {
        b.iter(|| nth_solution_one(1000));
    }

    #[bench]
    fn bench_nth_solution_two(b: &mut Bencher) {
        b.iter(|| nth_solution_two(1000));
    }

    #[bench]
    fn bench_nth_solution_three(b: &mut Bencher) {
        b.iter(|| nth_solution_three(1000));
    }

    #[bench]
    fn bench_nth_solution_four(b: &mut Bencher) {
        b.iter(|| nth_solution_four(1000));
    }
}