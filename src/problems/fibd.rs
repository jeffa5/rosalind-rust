struct Fib {
    rabbits: Vec<u64>,
}

impl Fib {
    fn new(death: usize) -> Self {
        let mut rabbits = vec![0; death];
        rabbits[0] = 1;
        Self { rabbits }
    }

    fn next(&mut self) {
        let new_kids = self.rabbits[1..].iter().sum();
        let mut new_rabbits = Vec::new();
        new_rabbits.push(new_kids);
        let len = self.rabbits.len() - 1;
        new_rabbits.append(&mut self.rabbits[0..len].to_vec());
        self.rabbits = new_rabbits;
    }

    fn total(&self) -> u64 {
        self.rabbits.iter().sum()
    }
}

pub fn solve(input: &str) -> String {
    let mut input = input.split_whitespace();
    let n = input.next().unwrap().parse::<u64>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();

    let mut fib = Fib::new(m);

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
        assert_eq!(solve("6 3"), "4")
    }
}
