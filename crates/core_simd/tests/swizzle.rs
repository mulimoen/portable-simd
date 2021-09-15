#![feature(portable_simd)]
use core_simd::{Simd, Swizzle};

#[test]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn permute() {
    struct Permute;
    impl Swizzle<4, 4> for Permute {
        const INDEX: [usize; 4] = [2, 1, 3, 0];
    }
    impl Swizzle<4, 2> for Permute {
        const INDEX: [usize; 2] = [1, 1];
    }

    let vector = Simd::from_array([2, 4, 1, 9]);
    assert_eq!(vector.swizzle(Permute).to_array(), [1, 4, 9, 2]);
    assert_eq!(vector.swizzle(Permute).to_array(), [4, 4]);
}
