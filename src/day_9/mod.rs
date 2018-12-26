use super::util;

pub fn solution() {
    let expected_results = util::read_expected_results("9");
    let (n, lines) = util::read_input_lines_with_n("9");

    let num: usize = expected_results.get(0).unwrap().parse().unwrap();
    assert_eq!(num, factorial(n));
}

fn factorial(n: usize) -> usize {
    if n == 1 {
        return 1;
    }
    n * factorial(n-1)
}