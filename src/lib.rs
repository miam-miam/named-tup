mod tup;
include!(concat!(env!("OUT_DIR"), "/tuple_types.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!("Hello, World!", message())
    }
}
