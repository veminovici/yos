/// Takes an u16 and returns the u8 values for it.
pub fn u16_bytes(u: &u16) -> [u8; 2] {
    [*u as u8, (u >> 8) as u8]
}

/// Takes an u32 and returns the u8 values for it.
pub fn u32_bytes(u: &u32) -> [u8; 4] {
    [*u as u8, (u >> 8) as u8, (u >> 16) as u8, (u >> 24) as u8]
}

/// Takes an u64 and returns the u8 values for it.
pub fn u64_bytes(u: &u64) -> [u8; 8] {
    [
        *u as u8,
        (u >> 8) as u8,
        (u >> 16) as u8,
        (u >> 24) as u8,
        (u >> 32) as u8,
        (u >> 40) as u8,
        (u >> 48) as u8,
        (u >> 56) as u8,
    ]
}

/// Takes an u64 and returns the u8 values for it.
pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        *u as u8,
        (u >> 8) as u8,
        (u >> 16) as u8,
        (u >> 24) as u8,
        (u >> 32) as u8,
        (u >> 40) as u8,
        (u >> 48) as u8,
        (u >> 56) as u8,
        (u >> 64) as u8,
        (u >> 72) as u8,
        (u >> 80) as u8,
        (u >> 88) as u8,
        (u >> 96) as u8,
        (u >> 104) as u8,
        (u >> 112) as u8,
        (u >> 120) as u8,
    ]
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

    #[test]
    fn test_u32_bytes() {
        let u: u32 = 0xabcdef12;
        let xs = u32_bytes(&u);
        assert_eq!(xs[0], 0x12);
        assert_eq!(xs[1], 0xef);
        assert_eq!(xs[2], 0xcd);
        assert_eq!(xs[3], 0xab);
    }

    #[test]
    fn test_u64_bytes() {
        let u: u64 = 0xabcdef1234567890;
        let xs = u64_bytes(&u);
        assert_eq!(xs[0], 0x90);
        assert_eq!(xs[1], 0x78);
        assert_eq!(xs[2], 0x56);
        assert_eq!(xs[3], 0x34);
        assert_eq!(xs[4], 0x12);
        assert_eq!(xs[5], 0xef);
        assert_eq!(xs[6], 0xcd);
        assert_eq!(xs[7], 0xab);
    }

    #[test]
    fn test_u128_bytes() {
        let u: u128 = 0x12345678123456781234567812345678;
        let xs = u128_bytes(&u);
        assert_eq!(xs[0], 0x78);
        assert_eq!(xs[1], 0x56);
        assert_eq!(xs[2], 0x34);
        assert_eq!(xs[3], 0x12);
        assert_eq!(xs[4], 0x78);
        assert_eq!(xs[5], 0x56);
        assert_eq!(xs[6], 0x34);
        assert_eq!(xs[7], 0x12);
        assert_eq!(xs[8], 0x78);
        assert_eq!(xs[9], 0x56);
        assert_eq!(xs[10], 0x34);
        assert_eq!(xs[11], 0x12);
        assert_eq!(xs[12], 0x78);
        assert_eq!(xs[13], 0x56);
        assert_eq!(xs[14], 0x34);
        assert_eq!(xs[15], 0x12);
    }
}
