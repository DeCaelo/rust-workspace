pub mod add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add::add_numbers(2, 2);
        assert_eq!(result, 4);
    }
}
