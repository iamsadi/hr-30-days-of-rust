use super::util;

pub fn solution() {
    let (_, lines) = util::read_input_lines_with_n("14");
    let expected = util::read_expected_results("14");
    let numbers: Vec<isize> = (lines.get(0).unwrap() as &String)
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut diff = Difference::from(numbers);
    diff.compute_diff();

    let expected_diff = (expected.get(0).unwrap() as &String).parse().unwrap();
    assert_eq!(diff.max_diff, expected_diff);
}

struct Difference {
    elements: Vec<isize>,
    pub max_diff: isize
}

impl Difference {

    fn from(elements: Vec<isize>) -> Difference {
        Difference { elements, max_diff: 0 }
    }

    fn compute_diff(&mut self) {
        for i in self.elements.iter() {
            for j in self.elements.iter() {
                let diff: isize = (i - j).abs();
                if diff > self.max_diff {
                    self.max_diff = diff;
                }
            }
        }
    }
}