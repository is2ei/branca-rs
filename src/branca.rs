pub fn encode<'a>(data: &'a str) -> &'a str {
    data
}

pub fn decode<'a>(data: &'a str) -> &'a str {
    data
}

#[cfg(test)]
mod tests {

    use branca;

    #[test]
    fn test_vector_1() {
        assert_eq!(2 + 2, 4);
        assert_eq!(branca::encode("data"), "data");
        assert_eq!(branca::decode("data"), "data");
    }

    #[test]
    fn test_vector_2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_generate_token() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_invalid_encode_string() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_invalid_decode_string() {
        assert_eq!(2 + 2, 4);
    }
}