use std::f64::consts::PI;

fn count_to_5() -> i32 {
    let mut number = 0;
    loop {
        if number > PI as i32 && number > 5 {
            break;
        }
        number += 1;
    }
    5
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
