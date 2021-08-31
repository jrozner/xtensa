mod instruction;
mod decode;

#[cfg(test)]
mod tests {
    use crate::decode::decode;

    #[test]
    fn it_works() {
        let data = [0xa1, 0x3a, 0xef];
        let decoded = decode(&data);
        assert!(decoded == None);
    }

    #[test]
    fn two_byte() {
        let data = [0x0c, 0xa2];
        let decoded = decode(&data);
        assert!(decoded == None);
    }
}
