use getrandom::getrandom;
use siphasher::sip::SipHasher13;
use std::hash::{Hash, Hasher};

/// Exposes SipHashers as iterator. A caller can take
/// 'k' hashes for the same item. It used in bloom-filter
/// implementations.
#[derive(Copy)]
pub struct K2Hasher {
    /// The collection os SipHashers
    hashers: [SipHasher13; 2],
}

impl Default for K2Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for K2Hasher {
    fn clone(&self) -> Self {
        Self { 
            hashers: [self.hashers[0], self.hashers[1]]
        }
    }
}

impl K2Hasher {
    /// Creates a new instance of the K2Hasher, using for initialization a seed.
    pub fn with_seed(seed: &[u8; 32]) -> Self {
        let mut k1 = [0u8; 16];
        let mut k2 = [0u8; 16];

        k1.copy_from_slice(&seed[0..16]);
        k2.copy_from_slice(&seed[16..32]);

        let hashers = [Self::new_sip(&k1), Self::new_sip(&k2)];
        Self { hashers }
    }

    /// Creates a new instance of the K2Hasher.
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        getrandom(&mut seed).unwrap();
        Self::with_seed(&seed)
    }

    /// Returns the hash iterator.
    pub fn iter<'a, I: Hash>(&self, item: &'a I) -> IterK2Hasher<'a, I> {
        IterK2Hasher::new(*self, item)
    }

    /// Creates the SipHasher13 hasher with a given key.
    #[inline]
    fn new_sip(seed: &[u8; 16]) -> SipHasher13 {
        SipHasher13::new_with_key(seed)
    }
}

//
// Iterators
//


/// An iterators which returns the next hash
/// for a given item.
pub struct IterK2Hasher<'a, I: Hash> {
    hasher: K2Hasher,
    i: usize,
    hashes: [u64; 2],
    item: &'a I,
}

impl<'a, I: Hash> IterK2Hasher<'a, I> {
    /// Builds a new instance of the Bits8 iterator
    pub fn new(hasher: K2Hasher, item: &'a I) -> Self {
        Self {
            hasher,
            i: 0,
            hashes: [0u64, 0u64],
            item,
        }
    }
}

impl<'a, I: Hash> Iterator for IterK2Hasher<'a, I> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ndx = if self.i < 2 {
            let sip = &mut self.hasher.hashers[self.i].clone();

            self.item.hash(sip);
            let hash = sip.finish();

            self.hashes[self.i] = hash;
            Some(hash as usize)
        } else {
            let v = (self.hashes[0] as u128)
                .wrapping_add((self.i as u128).wrapping_mul(self.hashes[1] as u128))
                as usize
                % 0xffffffffffffffc5;

            Some(v)
        };

        self.i += 1;
        ndx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let khasher = K2Hasher::new();

        let mut item = vec![0u8, 16];
        getrandom(&mut item).unwrap();

        let hash_iter = khasher.iter(&item);
        let hs1: Vec<usize> = hash_iter.take(10).collect();

        let hash_iter = khasher.iter(&item);
        let hs2: Vec<usize> = hash_iter.take(10).collect();

        assert_eq!(hs1, hs2);
    }
}
