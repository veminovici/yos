/// Takes an u16 and returns the u8 values for it.
pub fn u16_bytes(u: &u16) -> [u8; 2] {
    [*u as u8, (u >> 8) as u8]
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn test_u16_bytes() {
        let u: u16 = 0xabcd;
        let xs = u16_bytes(&u);
        assert_eq!(xs[0], 0xcd);
        assert_eq!(xs[1], 0xab);
    }
}
