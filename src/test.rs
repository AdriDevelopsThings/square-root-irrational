use rand::Rng;

use crate::{prim_factors, is_square_root_rational};

#[test]
fn test_prim_factors() {
    assert_eq!(prim_factors(1), vec![]);
    assert_eq!(prim_factors(2), vec![]);
    assert_eq!(prim_factors(3), vec![]);
    assert_eq!(prim_factors(4), vec![(2, 2)]);
    assert_eq!(prim_factors(5), vec![]);
    
    assert_eq!(prim_factors(651), vec![(3, 1), (7, 1), (31, 1)]);
    assert_eq!(prim_factors(982), vec![(2, 1), (491, 1)]);
    assert_eq!(prim_factors(255), vec![(3, 1), (5, 1), (17, 1)]);
}

#[test]
fn test_is_square_root_irrational() {
    assert!(is_square_root_rational(1));
    assert!(!is_square_root_rational(2));
    assert!(!is_square_root_rational(3));
    assert!(is_square_root_rational(4));
    assert!(!is_square_root_rational(5));
    
    assert!(!is_square_root_rational(8));
    assert!(is_square_root_rational(9));
    assert!(is_square_root_rational(100));
    assert!(!is_square_root_rational(101));

    let mut rng = rand::thread_rng();

    for _ in 1..100 {
        let n: u16 = rng.gen();
        let is_rational = is_square_root_rational(n as u32);
        let is_real_rational = f64::sqrt(n as f64).fract() == 0f64;
        println!("√{n} is_rational={is_rational}, is_real_rational={is_real_rational}");
        assert_eq!(is_rational, is_real_rational);
    }

    for _ in 1..30 {
        let n: u8 = rng.gen();
        let n = (n as u32) * (n as u32);
        println!("√{n} is_rational");
        assert!(is_square_root_rational(n));
    }
}