use siphasher::sip::SipHasher13;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub(crate) struct Sips {
    hashers: [SipHasher13; 2],
}

impl Sips {
    pub(crate) fn iter<'a, I: Hash>(&self, item: &'a I) -> IterSips<'a, I> {
        IterSips::new(*self, item)
    }
}

//
// Iterators
//

pub(crate) struct IterSips<'a, I: Hash> {
    sips: Sips,
    i: usize,
    hashes: [u64; 2],
    item: &'a I,
}

impl<'a, I: Hash> IterSips<'a, I> {
    /// Builds a new instance of the Bits8 iterator
    pub(crate) fn new(sips: Sips, item: &'a I) -> Self {
        Self {
            sips,
            i: 0,
            hashes: [0u64, 0u64],
            item,
        }
    }
}

impl<'a, I: Hash> Iterator for IterSips<'a, I> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 2 {
            let sip = &mut self.sips.hashers[self.i].clone();
            self.item.hash(sip);
            let hash = sip.finish();
            self.hashes[self.i] = hash;
            self.i += 1;
            Some(hash as usize)
        } else {
            let v = (self.hashes[0] as u128)
                .wrapping_add((self.i as u128).wrapping_mul(self.hashes[1] as u128))
                as usize
                % 0xffffffffffffffc5;
            self.i += 1;
            Some(v)
        }
    }
}
