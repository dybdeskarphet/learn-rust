pub fn add(first: i32, second: i32) -> i32 {
    first + second
}

pub fn substract(first: i32, second: i32) -> i32 {
    first - second
}

mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-2, -5), -7);
        assert_eq!(add(7, -7), 0);
        assert_eq!(add(9, -10), -1);
    }

    #[test]
    fn test_substract() {
        assert_eq!(substract(9, 3), 6);
        assert_eq!(substract(-2, -5), 3);
        assert_eq!(substract(7, -7), 14);
        assert_eq!(substract(20, 20), 0);
    }
}
