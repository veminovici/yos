//! A rust library for distances.

#![deny(missing_docs)]
#![deny(unreachable_pub)]
#![doc = include_str!("../README.md")]

mod distances;

pub use crate::distances::{
    chebyshev, hamming, lee, levenshtein, manhatann, sift3, square_euclidean,
};
