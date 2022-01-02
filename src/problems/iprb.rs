pub fn solve(input: &str) -> String {
    let mut kmn = input.split_whitespace();

    let k = kmn.next().unwrap().parse::<f64>().unwrap();
    let m = kmn.next().unwrap().parse::<f64>().unwrap();
    let n = kmn.next().unwrap().parse::<f64>().unwrap();

    let total = k + m + n;

    // always has dominant
    let kk = k * (k - 1.);
    let km = k * m * 2.;
    let kn = k * n * 2.;

    // dominant 3/4 times
    let mm = (m * (m - 1.)) * 0.75;

    // dominant 1/2 times
    let mn = m * n;

    let prob = (kk + km + kn + mm + mn) / (total * (total - 1.));

    format!("{:.5}", prob)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("2 2 2"), "0.78333")
    }
}
