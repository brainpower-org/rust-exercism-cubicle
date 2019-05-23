pub fn factors(n: u64) -> Vec<u64> {
        if n <= 1 {
            vec![]
        } else {
            dosomething2(n)
        }
}

fn dosomething(n: u64, factor: u64) -> Vec<u64> {
     println!("n: {}, factor: {}", n, factor);

    let mut result: Vec<u64> = vec![];
    if n <= 1 || factor > n {
           return result;
    }
        
        if n % factor == 0 {
            //result.push(factor);
            //result.extend(&dosomething(n / factor, factor));
            //result
            dosomething(n / factor, factor)
        } else {
            dosomething(n, factor + 1)
        }
}

fn dosomething2(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut number = n;
    let mut factor = 2;

    while factor <= number || number > 1 {
        if number % factor == 0 {
            result.push(factor);
            number = number / factor;
        } else {
            factor += 1;
        }
    }
    result
}
