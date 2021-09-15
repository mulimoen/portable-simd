use crate::{LaneCount, Simd, SimdElement, SupportedLaneCount};

pub trait Swizzle<const INPUT_LANES: usize, const OUTPUT_LANES: usize> {
    const INDEX: [usize; OUTPUT_LANES];

    #[doc(hidden)]
    const INDEX_IMPL: [u32; OUTPUT_LANES] = check::<INPUT_LANES, OUTPUT_LANES>(Self::INDEX);

    #[doc(hidden)]
    fn swizzle<T>(&self, vector: Simd<T, INPUT_LANES>) -> Simd<T, OUTPUT_LANES>
    where
        T: SimdElement,
        LaneCount<INPUT_LANES>: SupportedLaneCount,
        LaneCount<OUTPUT_LANES>: SupportedLaneCount,
    {
        unsafe { crate::intrinsics::simd_shuffle(vector, vector, Self::INDEX_IMPL) }
    }
}

const fn check<const INPUT_LANES: usize, const OUTPUT_LANES: usize>(
    index: [usize; OUTPUT_LANES],
) -> [u32; OUTPUT_LANES] {
    let mut output = [0; OUTPUT_LANES];
    let mut i = 0;
    while i < OUTPUT_LANES {
        let index = index[i];
        assert!(index as u32 as usize == index);
        assert!(index < INPUT_LANES, "source lane exceeds input lane count",);
        output[i] = index as u32;
        i += 1;
    }
    output
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    pub fn swizzle<const OUTPUT_LANES: usize>(
        self,
        index: impl Swizzle<LANES, OUTPUT_LANES>,
    ) -> Simd<T, OUTPUT_LANES>
    where
        LaneCount<OUTPUT_LANES>: SupportedLaneCount,
    {
        index.swizzle(self)
    }
}
