pub fn sum_u8s(a: u8, b: u8) -> u8 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        assert_eq!(0, sum_u8s(255, 1));
    }
}
