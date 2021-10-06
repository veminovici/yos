use core::cmp::min;
use num::Float;

/// Euclidean distance.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Euclidean_distance).
pub fn square_euclidean<T: Float>(xs: &[T], ys: &[T]) -> T {
    debug_assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| ((*x) - (*y)) * ((*x) - (*y)))
        .fold(T::zero(), ::std::ops::Add::add)
}

/// Manhatann distance. The distance between two vector is the sum of the
/// differences of their cartesian coordinates.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Taxicab_geometry).
pub fn manhatann<T: Float>(xs: &[T], ys: &[T]) -> T {
    debug_assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| (*x - *y).abs())
        .fold(T::zero(), ::std::ops::Add::add)
}

/// Chebyshev distance. The distance between vectors is the greatest
/// of their differences along any coordinate dimension.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Chebyshev_distance).
pub fn chebyshev<T: Float>(xs: &[T], ys: &[T]) -> T {
    debug_assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| (*x - *y).abs())
        .fold(T::min_value(), |acc, x| x.max(acc))
}

/// Levenshtein distance. The minimum number of single character
/// edits (ins, del, sub) required to change one word into another.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Levenshtein_distance).
pub fn levenshtein<T: PartialEq>(xs: &[T], ys: &[T]) -> usize {
    if ys.is_empty() {
        xs.len()
    } else if xs.is_empty() {
        ys.len()
    } else if xs[0] == ys[0] {
        levenshtein(&xs[1..], &ys[1..])
    } else {
        let a = levenshtein(&xs[1..], ys);
        let b = levenshtein(xs, &ys[1..]);
        let c = levenshtein(&xs[1..], &ys[1..]);
        1 + min(min(a, b), c)
    }
}

/// Hamming distance. The minimum number of subs required to change one word into another.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Hamming_distance).
pub fn hamming<T: PartialEq>(xs: &[T], ys: &[T]) -> usize {
    debug_assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .fold(0, |acc, (x, y)| if *x != *y { acc + 1 } else { acc })
}

/// Lee distance.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Lee_distance).
pub fn lee<T: Float>(xs: &[T], ys: &[T], alen: i32) -> i32 {
    debug_assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .map(|(x, y)| (*x - *y).abs().to_i32().unwrap())
        .map(|d| min(d, alen - d))
        .sum()
}

/// Sift3 distance. Fast string distance algorithm. The higher the returned value, the more different
/// the two words are. A value of 0.0 means both strings are equal.
pub fn sift3<T: PartialEq>(xs: &[T], ys: &[T], max_offset: usize) -> f32 {
    let len_xs = xs.len();
    let len_ys = ys.len();

    if len_xs == 0 {
        if len_ys == 0 {
            return 0.0;
        } else {
            return len_ys as f32;
        }
    }

    if len_ys == 0 {
        return len_xs as f32;
    }

    let mut c = 0;
    let mut offset1 = 0;
    let mut offset2 = 0;
    let mut lcs = 0;

    while (c + offset1 < len_xs) && (c + offset2 < len_ys) {
        if xs[c + offset1] == ys[c + offset2] {
            lcs += 1;
        } else {
            offset1 = 0;
            offset2 = 0;
            for i in 0..max_offset {
                if (c + i < len_xs) && xs[c + i] == ys[c] {
                    offset1 = i;
                    break;
                }

                if (c + i < len_ys) && (xs[c] == ys[c + i]) {
                    offset2 = i;
                    break;
                }
            }
        }
        c += 1;
    }

    ((len_xs + len_ys) as f32) / 2.0 - (lcs as f32)
}
