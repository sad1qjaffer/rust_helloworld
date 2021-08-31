#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!("Hello, World!", hello() );
    }
}
pub fn hello() -> &'static str{
    "Hello, World!"
} 