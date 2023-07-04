#[derive(Debug)]
struct Triplet(u64, u64, u64);

impl Triplet {
    fn area(&self) -> u64 {
        self.0 * self.1 / 2
    }
}
// Find pitagorean triplet with a given area
fn pythagorean_triplets(area : u64) -> Option<Triplet>
{
    let mut n : u64 = 1;
    let mut m : u64 = 2;

    while m * m + n * n < area
    {
        // Check if its prime and has the correct area
        if fast_gcd(n, m) == 1 && Triplet(m * m - n * n, 2 * m * n, m * m + n * n).area() == area {
            return Some(Triplet(m * m - n * n, 2 * m * n, m * m + n * n))
        }

        n += 1;
        if n == m
        {
            n = 1;
            m += 1;
        }
    }

    // Not found
    None
}

// Fast iterative version of Euclid's GCD algorithm
fn fast_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    return a;
}

fn solve(area: i32) -> String {
    format!("{:?}", pythagorean_triplets(area as u64))
}

fn main() {
    println!("{}", solve(666_666));
}
