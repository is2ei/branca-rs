pub mod branca {
    pub fn encode<'a>(data: &'a str) -> &'a str {
        data
    }

    pub fn decode<'a>(data: &'a str) -> &'a str {
        data
    }
}

#[cfg(test)]
mod tests {
    use branca;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        assert_eq!(branca::encode("encode"), "encode");
        assert_eq!(branca::decode("decode"), "decode");
    }
}
