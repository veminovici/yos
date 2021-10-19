use getrandom::getrandom;
use siphasher::sip::SipHasher13;
use std::f64;
use std::hash::{Hash, Hasher};

/// Counting-bloom-filter
pub struct CountingFilter {
    /// The bit-array
    storage: Vec<u8>,
    /// The number of bits
    m: usize,
    /// The number of hash functions.
    k: usize,
    /// The number of sip hashers
    sips: [SipHasher13; 2],
}

impl CountingFilter {
    /// Creates a new counting-bloom-filter.
    pub fn with_seed(counters_count: usize, items_count: usize, seed: &[u8; 32]) -> Self {
        debug_assert!(counters_count > 0 && items_count > 0);

        let m = counters_count;
        let k = Self::optimal_k(counters_count, items_count);
        let mut storage = Vec::with_capacity(m);
        for _i in 0..m {
            storage.push(0u8);
        }

        let mut k1 = [0u8; 16];
        let mut k2 = [0u8; 16];

        k1.copy_from_slice(&seed[0..16]);
        k2.copy_from_slice(&seed[16..32]);

        let sips = [Self::new_sip(&k1), Self::new_sip(&k2)];

        Self {
            storage,
            m,
            k,
            sips,
        }
    }

    /// Creates a new counting-bloom-filter structure.
    pub fn new(counters_count: usize, items_count: usize) -> Self {
        let mut seed = [0u8; 32];
        getrandom(&mut seed).unwrap();
        Self::with_seed(counters_count, items_count, &seed)
    }

    /// Creates a new bloom-filter for a given number of items and
    /// an wanted rate for false positives, in (0., 1.) interval.
    pub fn with_fp(items_count: usize, fp: f64) -> Self {
        let counters_count = Self::compute_counters_count(items_count, fp);
        Self::new(counters_count, items_count)
    }

    /// Creates a new bloom-filter for a given number of items, a wanted
    /// rate for false positives, and the seed for hashing functions.
    pub fn with_fp_seed(items_count: usize, fp: f64, seed: &[u8; 32]) -> Self {
        let counters_count = Self::compute_counters_count(items_count, fp);
        Self::with_seed(counters_count, items_count, seed)
    }

    /// Records the presence of an item
    pub fn set<T>(&mut self, item: &T)
    where
        T: Hash,
    {
        let mut hashes = [0u64, 0u64];
        for i in 0..self.k {
            let ndx = (self.bloom_hash(&mut hashes, item, i) % self.m) as usize;
            self.storage[ndx] += 1;
        }
    }

    /// Check if an item is present in the set.
    /// There can be false positives, but no false negatives.
    pub fn check<T>(&self, item: &T, threshold: u8) -> bool
    where
        T: Hash,
    {
        let mut hashes = [0u64, 0u64];
        for i in 0..self.k {
            let ndx = (self.bloom_hash(&mut hashes, item, i) % self.m) as usize;
            let current = self.storage[ndx];
            if current < threshold {
                return false;
            }
        }
        true
    }

    /// Record the presence of an item in the set,
    /// and return the previous state of this item.
    pub fn check_and_set<T>(&mut self, item: &T, threshold: u8) -> bool
    where
        T: Hash,
    {
        let mut hashes = [0u64, 0u64];
        let mut found = true;
        for k_i in 0..self.k {
            let ndx = (self.bloom_hash(&mut hashes, item, k_i) % self.m) as usize;
            let current = self.storage[ndx];
            if current < threshold {
                found = false;
            }
            self.storage[ndx] += 1;
        }
        found
    }

    /// Returns the optimal value for 'k' computed based on the number of bytes and items.
    fn optimal_k(counters_count: usize, items_count: usize) -> usize {
        let m = counters_count as f64;
        let n = items_count as f64;
        let k = (m / n * f64::ln(2.0f64)).ceil() as usize;
        std::cmp::max(k, 1)
    }

    /// Creates the SipHasher13 hasher with a given key.
    #[inline]
    fn new_sip(seed: &[u8; 16]) -> SipHasher13 {
        SipHasher13::new_with_key(seed)
    }

    fn compute_counters_count(items_count: usize, fp: f64) -> usize {
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
        let mut filter = CountingFilter::new(10, 100);

        let mut item = vec![0u8, 16];
        getrandom(&mut item).unwrap();

        filter.set(&item);
        assert!(!filter.check(&item, 2));

        filter.set(&item);
        assert!(filter.check(&item, 2));
    }
}
