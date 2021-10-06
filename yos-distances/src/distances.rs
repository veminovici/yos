use core::cmp::min;
use num::Float;

/// Euclidean distance.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Euclidean_distance).
///
/// # Example
///
/// ```
/// use yos_distances::square_euclidean;
///
/// let xs = [0., 0.];
/// let ys = [3., 4.];
/// let d = square_euclidean(&xs, &ys);
///
/// assert_eq!(25., d);
/// ```
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
///
/// # Example
///
/// ```
/// use yos_distances::manhatann;
///
/// let xs = [0., 0.];
/// let ys = [3., 4.];
/// let d = manhatann(&xs, &ys);
///
/// assert_eq!(7., d);
/// ```
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
///
/// # Example
///
/// ```
/// use yos_distances::chebyshev;
///
/// let xs = [0., 0.];
/// let ys = [3., 4.];
/// let d = chebyshev(&xs, &ys);
///
/// assert_eq!(4., d);
/// ```
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
///
/// # Example
///
/// ```
/// use yos_distances::levenshtein;
///
/// let a = "kitten".as_bytes();
/// let b = "sitting".as_bytes();
/// let d = levenshtein(a, b);
///
/// assert_eq!(3, d);
/// ```
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
///
/// # Example
///
/// ```
/// use yos_distances::hamming;
///
/// let a = "karolin".as_bytes();
/// let b = "kathrin".as_bytes();
/// let d = hamming(a, b);
///
/// assert_eq!(3, d);
/// ```
pub fn hamming<T: PartialEq>(xs: &[T], ys: &[T]) -> usize {
    debug_assert_eq!(xs.len(), ys.len());
    xs.iter()
        .zip(ys.iter())
        .fold(0, |acc, (x, y)| if *x != *y { acc + 1 } else { acc })
}

/// Lee distance.
/// For more details, see the wikipedia [page](https://en.wikipedia.org/wiki/Lee_distance).
///
/// # Example
///
/// ```
/// use yos_distances::lee;
///
/// let xs = [3., 1., 4., 0.];
/// let ys = [2., 5., 4., 3.];
///
/// let d = lee(&xs, &ys, 6);
/// assert_eq!(6, d);
/// ```
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
///
/// # Example
///
/// ```
/// use yos_distances::sift3;
///
/// let a = "hannah".as_bytes();
/// let b = "hanna".as_bytes();
/// let d = sift3(a, b, 5);
///
/// assert_eq!(0.5, d);
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_euclidean_pass() {
        let xs = [0., 0.];
        let ys = [3., 4.];
        let d = square_euclidean(&xs, &ys);

        let e: f32 = 25.;
        assert!((e - d).abs() < std::f32::EPSILON);
    }

    #[test]
    fn manhatann_pass() {
        let xs = [0., 0.];
        let ys = [3., 4.];
        let d = manhatann(&xs, &ys);
        let e: f32 = 7.;
        assert!((e - d).abs() < std::f32::EPSILON);
    }

    #[test]
    fn chebyshev_pass() {
        let xs = [0., 0.];
        let ys = [3., 4.];
        let d = chebyshev(&xs, &ys);
        let e: f32 = 4.;
        assert!((e - d).abs() < std::f32::EPSILON);
    }

    #[test]
    fn levenshtein_pass() {
        let a = "kitten".as_bytes();
        let b = "sitting".as_bytes();

        let d = levenshtein(a, b);
        assert_eq!(3, d);

        let d = levenshtein(b, a);
        assert_eq!(3, d);
    }

    #[test]
    fn hamming_pass() {
        let a = "karolin".as_bytes();
        let b = "kathrin".as_bytes();
        let d = hamming(a, b);
        assert_eq!(3, d);
    }

    #[test]
    fn lee_pass() {
        let xs = [3., 1., 4., 0.];
        let ys = [2., 5., 4., 3.];
        let d = lee(&xs, &ys, 6);
        assert_eq!(6, d);
    }

    #[test]
    fn sift3_pass() {
        let d = sift3("hannah".as_bytes(), "hanna".as_bytes(), 5);
        let e: f32 = 0.5;
        assert!((e - d).abs() < std::f32::EPSILON);
    }
}
