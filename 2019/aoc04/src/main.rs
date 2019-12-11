#![feature(is_sorted)]
fn main() {
    let lower_bound = 124075;
    let higher_bound = 580769;

    let p1 = |x: i32| x > 1;
    let a = (lower_bound..=higher_bound).filter(|x| is_pw(*x, &p1));
    println!("Part 1: {}", a.count());

    let p2 = |x: i32| x == 2;
    let b = (lower_bound..=higher_bound).filter(|x| is_pw(*x, &p2));
    println!("Part 2: {}", b.count());
}

fn is_pw(p: i32, f: &dyn Fn(i32) -> bool) -> bool {
    let mut freqs: [i32; 10] = [0; 10];

    let digits: Vec<i32> = 
        p.to_string()
         .split("")
         .filter(|x| *x != "")
         .map(|x| x.parse::<i32>().unwrap())
         .collect();

    if digits.is_sorted() {
        for x in digits {
            freqs[x as usize] = freqs[x as usize]+1;
        }
    }

    for freq in freqs.iter() {
        if f(*freq) {
            return true;
        }
    }

    return false;
}

