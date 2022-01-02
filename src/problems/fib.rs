struct Fib {
    kids: u64,
    adults: u64,
    growth: u64,
}

impl Fib {
    fn new(growth: u64) -> Self {
        Self {
            kids: 1,
            adults: 0,
            growth,
        }
    }

    fn next(&mut self) {
        let new_kids = self.adults * self.growth;
        self.adults += self.kids;
        self.kids = new_kids;
    }

    fn total(&self) -> u64 {
        self.kids + self.adults
    }
}

pub fn solve(input: &str) -> String {
    let mut input = input.split_whitespace();
    let n = input.next().unwrap().parse::<u64>().unwrap();
    let k = input.next().unwrap().parse::<u64>().unwrap();

    let mut fib = Fib::new(k);

    for _ in 1..n {
        fib.next();
    }
    format!("{}", fib.total())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("5 3"), "19")
    }
}
