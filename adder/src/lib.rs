#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests {
    #[test]
    fn will_pass() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn will_fail() {
        panic!("Make this test fail!");
    }

    #[test]
    fn will_work_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };
        let smaller = Rectangle {
            width: 5,
            height: 1
        };

        // Asserts boolean true
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod addition_tests {
    use super::add_two;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod greeting_tests {
    use super::greeting;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::Guess;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}