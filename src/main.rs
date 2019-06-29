fn main() {
    let input = String::from("22164224441");
    do_step(40, input);
}

fn do_step(iterations: u32, start: String) {
    let mut result = to_digits(&start).unwrap();

    for _ in 0..iterations {
        result = step(result);
    }
    println!("Result is {} digits long", result.len())
}

fn to_digits(text: &str) -> Option<Vec<u32>> {
    text.chars().map(|ch| ch.to_digit(10)).collect()
}

fn step(number: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut last_digit = 0;
    let mut count = 1;

    for digit in number {
        if digit == last_digit {
            count += 1;
        } else {
            if last_digit > 0 {
                result.push(count);
                result.push(last_digit);
                count = 1;
            }
            last_digit = digit;
        }
    }

    result.push(count);
    result.push(last_digit);

    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_case() {
        assert_eq!(step(vec![1]), vec![1, 1]);
        assert_eq!(step(vec![2]), vec![1, 2]);
        assert_eq!(step(vec![1, 1]), vec![2, 1]);
        assert_eq!(step(vec![3, 1]), vec![1, 3, 1, 1]);
        assert_eq!(step(vec![3, 2, 1, 1]), vec![1, 3, 1, 2, 2, 1]);
        assert_eq!(step(vec![1, 1, 1, 2, 2, 3]), vec![3, 1, 2, 2, 1, 3]);
    }
}
