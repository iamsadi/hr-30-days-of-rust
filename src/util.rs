use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_expected_results(day: &str) -> Vec<String> {
    let mut expected_results: Vec<String> = Vec::new();
    let tc_out_filename = format!("{}{}{}", "src/day_", day, "/tc.out");
    let tc_out_file = File::open(tc_out_filename).unwrap();
    let out_reader = BufReader::new(tc_out_file);
    for line in out_reader.lines() {
        expected_results.push(line.unwrap());
    }
    return expected_results;
}

pub fn read_input_lines_with_n(day: &str) -> (usize, Vec<String>) {
    let tc_in_filename = format!("{}{}{}", "src/day_", day, "/tc.in");
    let tc_in_file = File::open(tc_in_filename).unwrap();
    let in_reader = BufReader::new(tc_in_file);

    let mut n: usize = 0;
    let mut lines = Vec::new();

    for (idx, line) in in_reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if idx == 0 {
            n = line.parse().unwrap(); // first line is the size of the test data input
        } else {
            lines.push(line);
        }
    }

    return (n, lines);
}

pub fn read_input_lines(day: &str) -> Vec<String> {
    let tc_in_filename = format!("{}{}{}", "src/day_", day, "/tc.in");
    let tc_in_file = File::open(tc_in_filename).unwrap();
    let in_reader = BufReader::new(tc_in_file);

    let mut lines = Vec::new();

    for line in in_reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        lines.push(line);
    }

    return lines;
}