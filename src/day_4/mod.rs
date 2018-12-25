struct Person {
    age: i32
}

impl Person {
    fn of(initial_age: i32) -> Person {
        if initial_age < 0 {
            println!("Age is not valid, setting age to 0.");
            Person { age: 0 }
        } else {
            Person { age: initial_age }
        }
    }

    fn am_i_old(&self) {
        if self.age < 13 {
            println!("You are young.");
        } else if self.age >= 13 && self.age < 18 {
            println!("You are a teenager.");
        } else {
            println!("You are old.");
        }
    }

    fn year_passes(&mut self) {
        self.age = self.age + 1;
    }
}

pub fn solution() {
    let p1 = Person::of(1);
    let p2 = Person::of(13);
    let p3 = Person::of(18);
    let p4 = Person::of(19);
    let mut p5 = Person::of(12);

    p1.am_i_old();
    p2.am_i_old();
    p3.am_i_old();
    p4.am_i_old();
    p5.am_i_old();
    p5.year_passes();
    p5.am_i_old();

    /*
    Output:

    You are young.
    You are a teenager.
    You are old.
    You are old.
    You are young.
    You are a teenager.
    */
}
