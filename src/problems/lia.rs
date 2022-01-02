pub fn solve(input: &str) -> String {
    let mut input = input.split_whitespace();

    let k = input.next().unwrap().parse::<u32>().unwrap();
    let n = input.next().unwrap().parse::<u32>().unwrap();

    format!("{}", k * n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("2 1"), "0.684")
    }
}
