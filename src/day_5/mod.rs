pub fn solution() {
    // Some already declared and initialized variables (Task's specific)
    // Constraints: 2 <= n <= 20
    let n : i32 = 2;

    for i in 1..11 {
        println!("{} x {} = {}", n, i, n*i);
    }
}
