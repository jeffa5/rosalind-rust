pub fn solve(input: &str) -> String {
    let mut input = input.split_whitespace();

    let aaaa = input.next().unwrap().parse::<u32>().unwrap();
    let aaab = input.next().unwrap().parse::<u32>().unwrap();
    let aabb = input.next().unwrap().parse::<u32>().unwrap();
    let abab = input.next().unwrap().parse::<u32>().unwrap();
    let abbb = input.next().unwrap().parse::<u32>().unwrap();

    let all_dom = f64::from(2 * (aaaa + aaab + aabb));
    let three_quarter_dom = (2. * f64::from(abab)) * (3. / 4.);
    let half_dom = f64::from(abbb);
    let exp = all_dom + three_quarter_dom + half_dom;

    format!("{}", exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("1 0 0 1 0 1"), "3.5")
    }
}
