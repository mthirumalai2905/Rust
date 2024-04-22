fn main() {
    for i in 1..20{
        if is_prime(i) {
            print!("{}\t", i)
        }
    }
}

fn is_prime(n:usize) -> bool{
    let mut check = true;

    for i in 2..n{
        if n % i == 0 {
            check = false;
        }
    }

    return check;
}
