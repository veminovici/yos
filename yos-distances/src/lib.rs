//! A rust library for distance functions.

#![deny(missing_docs)]
#![deny(unreachable_pub)]

mod distances;

pub use crate::distances::{
    chebyshev, hamming, lee, levenshtein, manhatann, sift3, square_euclidean,
};
