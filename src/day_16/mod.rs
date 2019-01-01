use super::util;

pub fn solution() {
    let expected: String = util::read_expected_results("16").get(0).unwrap().to_string();
    let lines = util::read_input_lines("16");

    let mut result: String = "".to_string();
    let i: Result<isize, _> = lines.get(0).unwrap().trim().parse();
    if i.is_err() {
        println!("Bad String");
        result = "Bad String".to_string();
    } else {
        let num = i.ok().unwrap();
        println!("{:?}", &num);
        result = String::from(format!("{}", &num));
    }

    assert_eq!(result, expected);
}