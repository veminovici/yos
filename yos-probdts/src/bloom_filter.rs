use bit_vec::BitVec;
use getrandom::getrandom;
use siphasher::sip::SipHasher13;
use std::f64;
use std::hash::{Hash, Hasher};

/// Bloom-filter. It consists of a bit-array of 'm' bits. Initialy all bits are set to 0.
/// There are k different hash functions defined, each of which maps or hashes some set element to
/// one of the 'm' array positions, generating a uniform random distribution. 'k' is a small constant
/// which depends on the desired false error rate, while m is proportional to 'k' and to the number of
/// elements to be added.
pub struct BloomFilter {
    /// The bit-array
    bits: BitVec,
    /// The number of bits
    m: usize,
    /// The number of hash functions.
    k: usize,
    /// The number of sip hashers
    sips: [SipHasher13; 2],
}

impl BloomFilter {
    /// Creates a new bloom-filter.
    pub fn with_seed(bytes_count: usize, items_count: usize, seed: &[u8; 32]) -> Self {
        debug_assert!(bytes_count > 0 && items_count > 0);

        let m = bytes_count * 8;
        let k = Self::optimal_k(bytes_count, items_count);
        let bits = BitVec::from_elem(m, false);

        let mut k1 = [0u8; 16];
        let mut k2 = [0u8; 16];

        k1.copy_from_slice(&seed[0..16]);
        k2.copy_from_slice(&seed[16..32]);

        let sips = [Self::new_sip(&k1), Self::new_sip(&k2)];

        Self { bits, m, k, sips }
    }

    /// Creates a new bloom-filter structure.
    pub fn new(bytes_count: usize, items_count: usize) -> Self {
        let mut seed = [0u8; 32];
        getrandom(&mut seed).unwrap();
        Self::with_seed(bytes_count, items_count, &seed)
    }

    /// Creates a new bloom-filter for a given number of items and
    /// an wanted rate for false positives, in (0., 1.) interval.
    pub fn with_fp(items_count: usize, fp: f64) -> Self {
        let bytes_count = Self::compute_bytes_count(items_count, fp);
        Self::new(bytes_count, items_count)
    }

    /// Creates a new bloom-filter for a given number of items, a wanted
    /// rate for false positives, and the seed for hashing functions.
    pub fn with_fp_seed(items_count: usize, fp: f64, seed: &[u8; 32]) -> Self {
        let bytes_count = Self::compute_bytes_count(items_count, fp);
        Self::with_seed(bytes_count, items_count, seed)
    }

    /// Records the presence of an item
    pub fn set<T>(&mut self, item: &T)
    where
        T: Hash,
    {
        let mut hashes = [0u64, 0u64];
        for i in 0..self.k {
            let bit_offset = (self.bloom_hash(&mut hashes, item, i) % self.m) as usize;
            self.bits.set(bit_offset, true);
        }
    }

    /// Check if an item is present in the set.
    /// There can be false positives, but no false negatives.
    pub fn check<T>(&self, item: &T) -> bool
    where
        T: Hash,
    {
        let mut hashes = [0u64, 0u64];
        for i in 0..self.k {
            let bit_offset = (self.bloom_hash(&mut hashes, item, i) % self.m) as usize;
            if !self.bits.get(bit_offset).unwrap() {
                return false;
            }
        }
        true
    }

    /// Record the presence of an item in the set,
    /// and return the previous state of this item.
    pub fn check_and_set<T>(&mut self, item: &T) -> bool
    where
        T: Hash,
    {
        let mut hashes = [0u64, 0u64];
        let mut found = true;
        for k_i in 0..self.k {
            let bit_offset = (self.bloom_hash(&mut hashes, item, k_i) % self.m) as usize;
            if !self.bits.get(bit_offset).unwrap() {
                found = false;
                self.bits.set(bit_offset, true);
            }
        }
        found
    }

    /// Returns the optimal value for 'k' computed based on the number of bytes and items.
    fn optimal_k(bytes_count: usize, items_count: usize) -> usize {
        let m = bytes_count as f64;
        let n = items_count as f64;
        let k = (m / n * f64::ln(2.0f64)).ceil() as usize;
        std::cmp::max(k, 1)
    }

    /// Creates the SipHasher13 hasher with a given key.
    #[inline]
    fn new_sip(seed: &[u8; 16]) -> SipHasher13 {
        SipHasher13::new_with_key(seed)
    }

    fn compute_bytes_count(items_count: usize, fp: f64) -> usize {
        debug_assert!(items_count > 0);
        debug_assert!(fp > 0. && fp < 1.);
        let log2 = f64::consts::LN_2 * f64::consts::LN_2;
        ((items_count as f64) * f64::ln(fp) / (-8. * log2)).ceil() as usize
    }

    fn bloom_hash<T>(&self, hashes: &mut [u64; 2], item: &T, i: usize) -> usize
    where
        T: Hash,
    {
        if i < 2 {
            let sip = &mut self.sips[i].clone();
            item.hash(sip);
            let hash = sip.finish();
            hashes[i] = hash;
            hash as usize
        } else {
            (hashes[0] as u128).wrapping_add((i as u128).wrapping_mul(hashes[1] as u128)) as usize
                % 0xffffffffffffffc5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_check() {
        let mut b = BloomFilter::new(10, 100);

        let mut item = vec![0u8, 16];
        getrandom(&mut item).unwrap();

        b.set(&item);
        assert!(b.check(&item));
    }
}
