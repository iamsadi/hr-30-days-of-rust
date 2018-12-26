use super::util;

pub fn solution() {
    let expected_results = util::read_expected_results("10");
    let (n, lines) = util::read_input_lines_with_n("10");

    let mut max: usize = 0;
    let mut tmp: usize = 0;
    for i in 1..65 {
        // Left shift 1 by i-1 to create a mask only with single bit set.
        let mask = shift_bit(1, i);
        if is_bit_set(n, mask) {
            tmp += 1;
        } else {
            tmp = 0;
        }

        if tmp > max {
            max = tmp;
        }
    }

    assert_eq!(&max.to_string(), expected_results.get(0).unwrap());
}

fn shift_bit(number: usize, bit_idx: usize) -> usize {
    return number << bit_idx - 1;
}

// If bitwise AND of number and mask is non-zero, then bit is set.
fn is_bit_set(number: usize, mask: usize) -> bool {
    return (number & mask) != 0;
}