use yos_distances::*;

#[test]
fn test_square_euclidean() {
    let xs = vec![0., 0.];
    let ys = vec![3., 4.];
    let d = square_euclidean(xs.as_slice(), ys.as_slice());

    let e: f32 = 25.;
    assert!((e - d).abs() < std::f32::EPSILON);
}

#[test]
fn test_manhatann() {
    let xs = vec![0., 0.];
    let ys = vec![3., 4.];
    let d = manhatann(xs.as_slice(), ys.as_slice());
    let e: f32 = 7.;
    assert!((e - d).abs() < std::f32::EPSILON);
}

#[test]
fn test_chebyshev() {
    let xs = vec![0., 0.];
    let ys = vec![3., 4.];
    let d = chebyshev(xs.as_slice(), ys.as_slice());
    let e: f32 = 4.;
    assert!((e - d).abs() < std::f32::EPSILON);
}

#[test]
fn test_levenshtein() {
    let a = "kitten".as_bytes();
    let b = "sitting".as_bytes();

    let d = levenshtein(a, b);
    assert_eq!(3, d);

    let d = levenshtein(b, a);
    assert_eq!(3, d);
}

#[test]
fn test_hamming() {
    let a = "karolin".as_bytes();
    let b = "kathrin".as_bytes();
    let d = hamming(a, b);
    assert_eq!(3, d);
}

#[test]
fn test_lee() {
    let xs = [3., 1., 4., 0.];
    let ys = [2., 5., 4., 3.];
    let d = lee(&xs, &ys, 6);
    assert_eq!(6, d);
}

#[test]
fn test_sift3() {
    let d = sift3("hannah".as_bytes(), "hanna".as_bytes(), 5);
    let e: f32 = 0.5;
    assert!((e - d).abs() < std::f32::EPSILON);
}
