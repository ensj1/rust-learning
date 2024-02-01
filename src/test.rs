fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;
    #[test]
    fn test_add() {
        let result: i32 = add(3, 2);

        assert_eq!(result, 4);
    }
}