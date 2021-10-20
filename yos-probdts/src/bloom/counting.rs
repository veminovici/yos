use super::K2Hasher;
use super::UCounter;

use getrandom::getrandom;
use std::f64;
use std::hash::Hash;

/// Counting-bloom-filter
pub struct CountingFilter<T: UCounter> {
    /// The bit-array
    storage: Vec<T>,
    /// The number of bits
    m: usize,
    /// The number of hash functions.
    k: usize,
    /// The hasher which can generate k hashes
    hasher: K2Hasher,
}

impl<T: std::fmt::Debug + UCounter> CountingFilter<T> {
    /// Creates a new counting-bloom-filter.
    pub fn with_seed(counters_count: usize, items_count: usize, seed: &[u8; 32]) -> Self {
        debug_assert!(counters_count > 0 && items_count > 0);

        let k = Self::optimal_k(counters_count, items_count);
        let m = counters_count;
        let hasher = K2Hasher::with_seed(seed);
        let mut storage = Vec::with_capacity(m);
        for _i in 0..m {
            storage.push(T::zero());
        }

        Self {
            storage,
            m,
            k,
            hasher,
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
    pub fn set<I>(&mut self, item: &I)
    where
        I: Hash,
    {
        self.indexes(item)
            .iter()
            .for_each(|ndx| self.storage[*ndx] += T::one());
    }

    /// Check if an item is present in the set.
    /// There can be false positives, but no false negatives.
    pub fn check<I>(&self, item: &I, threshold: T) -> bool
    where
        I: Hash,
    {
        self.indexes(item).iter().fold(true, |acc, ndx| {
            if self.storage[*ndx] < threshold {
                false
            } else {
                acc
            }
        })
    }

    /// Record the presence of an item in the set,
    /// and return the previous state of this item.
    pub fn check_and_set<I>(&mut self, item: &I, threshold: T) -> bool
    where
        I: Hash,
    {
        let mut found = true;
        self.indexes(item).iter().for_each(|ndx| {
            if self.storage[*ndx] < threshold {
                found = false;
            }
            self.storage[*ndx] += T::one();
        });
        found
    }

    /// Returns the indexes for a given item.
    fn indexes<I: Hash>(&self, item: &I) -> Vec<usize> {
        self.hasher
            .iter(item)
            .take(self.k)
            .map(|n| n % self.m)
            .collect()
    }

    /// Returns the optimal value for 'k' computed based on the number of bytes and items.
    fn optimal_k(counters_count: usize, items_count: usize) -> usize {
        let m = counters_count as f64;
        let n = items_count as f64;
        let k = (m / n * f64::ln(2.0f64)).ceil() as usize;
        std::cmp::max(k, 1)
    }

    fn compute_counters_count(items_count: usize, fp: f64) -> usize {
        debug_assert!(items_count > 0);
        debug_assert!(fp > 0. && fp < 1.);

        let log2 = f64::consts::LN_2 * f64::consts::LN_2;
        ((items_count as f64) * f64::ln(fp) / (-8. * log2)).ceil() as usize
    }
}

impl<T: std::fmt::Debug + UCounter> std::fmt::Debug for CountingFilter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "storage={:?}", self.storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_check() {
        let mut filter = CountingFilter::<u8>::new(1000, 100000);
        println!("filter={:?}", filter);

        let mut item = vec![0u8, 16];
        getrandom(&mut item).unwrap();

        filter.set(&item);
        assert!(!filter.check(&item, 2));

        filter.set(&item);
        assert!(filter.check(&item, 2));
    }
}
