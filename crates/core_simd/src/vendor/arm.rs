use crate::simd::*;

#[cfg(target_arch = "arm")]
use core::arch::arm::*;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

from_transmute! { unsafe f32x2 => float32x2_t }
from_transmute! { unsafe f32x4 => float32x4_t }

from_transmute! { unsafe u8x8 => uint8x8_t }
from_transmute! { unsafe u8x16 => uint8x16_t }
from_transmute! { unsafe i8x8 => int8x8_t }
from_transmute! { unsafe i8x16 => int8x16_t }
from_transmute! { unsafe u8x8 => poly8x8_t }
from_transmute! { unsafe u8x16 => poly8x16_t }

from_transmute! { unsafe u16x4 => uint16x4_t }
from_transmute! { unsafe u16x8 => uint16x8_t }
from_transmute! { unsafe i16x4 => int16x4_t }
from_transmute! { unsafe i16x8 => int16x8_t }
from_transmute! { unsafe u16x4 => poly16x4_t }
from_transmute! { unsafe u16x8 => poly16x8_t }

from_transmute! { unsafe u32x2 => uint32x2_t }
from_transmute! { unsafe u32x4 => uint32x4_t }
from_transmute! { unsafe i32x2 => int32x2_t }
from_transmute! { unsafe i32x4 => int32x4_t }

from_transmute! { unsafe Simd<u64, 1> => uint64x1_t }
from_transmute! { unsafe u64x2 => uint64x2_t }
from_transmute! { unsafe Simd<i64, 1> => int64x1_t }
from_transmute! { unsafe i64x2 => int64x2_t }
from_transmute! { unsafe Simd<u64, 1> => poly64x1_t }
from_transmute! { unsafe u64x2 => poly64x2_t }

#[cfg(target_arch = "arm")]
mod arm {
    use super::*;
    from_transmute! { unsafe Simd<u8, 4> => uint8x4_t }
    from_transmute! { unsafe Simd<i8, 4> => int8x4_t }

    from_transmute! { unsafe Simd<u16, 2> => uint16x2_t }
    from_transmute! { unsafe Simd<i16, 2> => int16x2_t }
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    use super::*;
    from_transmute! { unsafe Simd<f64, 1> => float64x1_t }
    from_transmute! { unsafe f64x2 => float64x2_t }
}
