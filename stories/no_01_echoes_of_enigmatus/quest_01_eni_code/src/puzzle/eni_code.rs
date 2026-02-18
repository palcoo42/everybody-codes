use std::collections::VecDeque;

// Calculate EniCode
pub fn eni_code(number: i64, exponent: i64, modulo: i64) -> i64 {
    let mut code: VecDeque<i64> = VecDeque::new();
    let mut score = 1;

    for _ in 0..exponent {
        // Calculate new score
        score *= number;

        // Add remainder to the code
        code.push_front(score % modulo);
    }

    // Convert code into number string
    let number: String = code.iter().map(|e| e.to_string()).collect();

    // Convert to i64 to be easily comparable for maximum value
    number
        .parse::<i64>()
        .unwrap_or_else(|_| panic!("Failed to parse '{number}' to i64"))
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_eni_code() {
        assert_eq!(eni_code(2, 4, 5), 1342);
        assert_eq!(eni_code(3, 5, 16), 311193);
    }
}
