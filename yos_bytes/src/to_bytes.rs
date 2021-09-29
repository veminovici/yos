/// Takes an u16 and returns the u8 values for it.
pub fn u16_to_bytes(u: &u16) -> [u8; 2] {
    [*u as u8, (u >> 8) as u8]
}

/// Takes an u32 and returns the u8 values for it.
pub fn u32_to_bytes(u: &u32) -> [u8; 4] {
    [*u as u8, (u >> 8) as u8, (u >> 16) as u8, (u >> 24) as u8]
}

/// Takes an u64 and returns the u8 values for it.
pub fn u64_to_bytes(u: &u64) -> [u8; 8] {
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
pub fn u128_to_bytes(u: &u128) -> [u8; 16] {
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

/// Extract the difficulty bytes
pub fn difficulty_bytes(xs: &[u8], len: usize) -> u128 {
    let mut u = 0u128;

    for i in 0..len {
        let mut x = xs[31 - i] as u128;
        x <<= (len - i - 1) * 8;
        u |= x;
    }

    u
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn test_u16_to_bytes() {
        let u: u16 = 0x1234;
        let xs = u16_to_bytes(&u);

        assert_eq!(xs[0], 0x34);
        assert_eq!(xs[1], 0x12);
    }

    #[test]
    fn test_u32_to_bytes() {
        let u: u32 = 0x12341234;
        let xs = u32_to_bytes(&u);

        assert_eq!(xs[0], 0x34);
        assert_eq!(xs[1], 0x12);
        assert_eq!(xs[2], 0x34);
        assert_eq!(xs[3], 0x12);
    }

    #[test]
    fn test_u64_to_bytes() {
        let u: u64 = 0x1234123412341234;
        let xs = u64_to_bytes(&u);

        assert_eq!(xs[0], 0x34);
        assert_eq!(xs[1], 0x12);
        assert_eq!(xs[2], 0x34);
        assert_eq!(xs[3], 0x12);
        assert_eq!(xs[4], 0x34);
        assert_eq!(xs[5], 0x12);
        assert_eq!(xs[6], 0x34);
        assert_eq!(xs[7], 0x12);
    }

    #[test]
    fn test_u128_to_bytes() {
        let u: u128 = 0x12345678123456781234567812345678;
        let xs = u128_to_bytes(&u);

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

    #[test]
    fn test_difficulty_bytes() {
        let xs: Vec<u8> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];

        let v = difficulty_bytes(xs.as_slice(), 16);
        assert_eq!((3 << 16) + (2 << 8) + 1, v);
    }
}
