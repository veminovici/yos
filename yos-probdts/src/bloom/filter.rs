use super::K2Hasher;

use bit_vec::BitVec;
use getrandom::getrandom;
use std::f64;
use std::hash::Hash;

/// Bloom-filter. It consists of a bit-array of 'm' bits. Initialy all bits are set to 0.
/// There are k different hash functions defined, each of which maps or hashes some set element to
/// one of the 'm' array positions, generating a uniform random distribution. 'k' is a small constant
/// which depends on the desired false error rate, while m is proportional to 'k' and to the number of
/// elements to be added.
pub struct Filter {
    /// The bit-array
    bits: BitVec,
    /// The number of bits
    m: usize,
    /// The number of hash functions.
    k: usize,
    /// The hasher which can generate k hashes
    hasher: K2Hasher,
}

impl Filter {
    /// Creates a new bloom-filter.
    pub fn with_seed(bytes_count: usize, items_count: usize, seed: &[u8; 32]) -> Self {
        debug_assert!(bytes_count > 0 && items_count > 0);

        let k = Self::optimal_k(bytes_count, items_count);
        let m = bytes_count * 8;
        let hasher = K2Hasher::with_seed(seed);
        let bits = BitVec::from_elem(m, false);

        Self { bits, m, k, hasher }
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
    pub fn set<T: Hash>(&mut self, item: &T) {
        self.bit_offsets(item)
            .iter()
            .for_each(|b| self.bits.set(*b, true));
    }

    /// Check if an item is present in the set.
    /// There can be false positives, but no false negatives.
    pub fn check<T: Hash>(&self, item: &T) -> bool {
        let mut found = true;
        self.bit_offsets(item).iter().for_each(|b| {
            if !self.bits.get(*b).unwrap() {
                found = false;
            }
        });
        found
    }

    /// Record the presence of an item in the set,
    /// and return the previous state of this item.
    pub fn check_and_set<T: Hash>(&mut self, item: &T) -> bool {
        let mut found = true;
        self.bit_offsets(item).iter().for_each(|b| {
            if !self.bits.get(*b).unwrap() {
                found = false;
                self.bits.set(*b, true);
            }
        });
        found
    }

    /// Returns the bit offsets for a given item.
    fn bit_offsets<T: Hash>(&self, item: &T) -> Vec<usize> {
        self.hasher
            .iter(item)
            .take(self.k)
            .map(|n| n % self.m)
            .collect()
    }

    /// Returns the optimal value for 'k' computed based on the number of bytes and items.
    fn optimal_k(bytes_count: usize, items_count: usize) -> usize {
        let m = bytes_count as f64;
        let n = items_count as f64;
        let k = (m / n * f64::ln(2.0f64)).ceil() as usize;
        std::cmp::max(k, 1)
    }

    fn compute_bytes_count(items_count: usize, fp: f64) -> usize {
        debug_assert!(items_count > 0);
        debug_assert!(fp > 0. && fp < 1.);
        let log2 = f64::consts::LN_2 * f64::consts::LN_2;
        ((items_count as f64) * f64::ln(fp) / (-8. * log2)).ceil() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_check() {
        let mut filter = Filter::new(1000, 100000);

        let mut item = vec![0u8, 16];
        getrandom(&mut item).unwrap();

        filter.set(&item);
        assert!(filter.check(&item));
    }
}
