use crate::arch::current::vecs::*;
use crate::vecs::*;
use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::std::ops::Add;
use crate::intrin::upcast::Upcast;
use crate::intrin::cmp::Cmp;
use crate::intrin::abs::Abs;
use crate::intrin::sum::{Sum,UpcastSum};
use crate::intrin::transmute::Transmute;

// TODO: Specialization
// impl<T> Sum for T where T : , T::Scalar : Add<T::Scalar, Output = T::Scalar>, T::Scalar : From<i8> {
//     #[inline(always)]
//     default fn sum(&self) -> Self::Scalar {
//         self.scalar_reduce(Self::Scalar::from(0i8), |acc, s| acc + s)
//     }
// }

// TODO: Specialization
// impl<T> UpcastSum for T where T : , T::Scalar : Add<i64, Output = i64>, i64 : From<T::Scalar> {
//     #[inline(always)]
//     default fn sum_upcast(&self) -> i64 {
//         self.scalar_reduce(0i64, |acc, s| acc + i64::from(s))
//     }
// }

#[cfg(target_feature = "sse2")]
impl Sum for i8x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let pos = unsafe { _mm_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x16::splat(0)).be_u16s() };
        let neg = unsafe { _mm_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x16::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0 as i8
    }
}

#[cfg(target_feature = "sse2")]
impl UpcastSum for i8x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        optimized!();
        let pos = unsafe { _mm_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x16::splat(0)).be_u16s() };
        let neg = unsafe { _mm_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x16::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0 as i8 as i64
    }
}

#[cfg(target_feature = "avx2")]
impl Sum for i8x32 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let pos = unsafe { _mm256_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x32::splat(0)).be_u16s() };
        let neg = unsafe { _mm256_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x32::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0
            .overflowing_add(pos.extract(8).overflowing_sub(neg.extract(8)).0).0
            .overflowing_add(pos.extract(12).overflowing_sub(neg.extract(12)).0).0 as i8
    }
}

#[cfg(target_feature = "avx2")]
impl UpcastSum for i8x32 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        optimized!();
        let pos = unsafe { _mm256_sad_epu8(self.max(Self::splat(0)).be_u8s(), u8x32::splat(0)).be_u16s() };
        let neg = unsafe { _mm256_sad_epu8(self.min(Self::splat(0)).abs().be_u8s(), u8x32::splat(0)).be_u16s() };
        pos.extract(0).overflowing_sub(neg.extract(0)).0
            .overflowing_add(pos.extract(4).overflowing_sub(neg.extract(4)).0).0
            .overflowing_add(pos.extract(8).overflowing_sub(neg.extract(8)).0).0
            .overflowing_add(pos.extract(12).overflowing_sub(neg.extract(12)).0).0 as i8 as i64
    }
}

#[cfg(target_feature = "sse2")]
impl Sum for u8x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let x = unsafe { _mm_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4)) as u8
    }
}

#[cfg(target_feature = "sse2")]
impl UpcastSum for u8x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        optimized!();
        let x = unsafe { _mm_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4)) as i64
    }
}

#[cfg(target_feature = "avx2")]
impl Sum for u8x32 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let x = unsafe { _mm256_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4) + x.extract(8) + x.extract(12)) as u8
    }
}

#[cfg(target_feature = "avx2")]
impl UpcastSum for u8x32 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        optimized!();
        let x = unsafe { _mm256_sad_epu8(*self, Self::splat(0)).be_u16s() };
        (x.extract(0) + x.extract(4) + x.extract(8) + x.extract(12)) as i64
    }
}

#[cfg(target_feature = "ssse3")]
impl Sum for i16x8 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let x =  unsafe {
            _mm_hadd_epi16(
                _mm_hadd_epi16(
                    _mm_hadd_epi16(*self, Self::splat(0)), Self::splat(0)), Self::splat(0))
        };
        x.extract(0)
    }
}

#[cfg(target_feature = "avx2")]
impl Sum for i16x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let x =  unsafe {
            _mm256_hadd_epi16(
                _mm256_hadd_epi16(
                    _mm256_hadd_epi16(*self, Self::splat(0)), Self::splat(0)), Self::splat(0))
        };
        x.extract(0) + x.extract(8)
    }
}

#[cfg(target_feature = "avx2")]
impl UpcastSum for i16x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        optimized!();
        unsafe {
            let (a, b) = self.upcast();
            let x =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(a.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            let y =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(b.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            (x.extract(0) + x.extract(4) + y.extract(0) + y.extract(4)) as i64
        }
    }
}

#[cfg(target_feature = "avx2")]
impl Sum for u16x16 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        unsafe {
            let (a, b) = self.upcast();
            let x =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(a.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            let y =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(b.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            (x.extract(0) + x.extract(4) + y.extract(0) + y.extract(4)) as u16
        }
    }
}

#[cfg(target_feature = "avx2")]
impl UpcastSum for u16x16 {
    #[inline(always)]
    fn sum_upcast(&self) -> i64 {
        optimized!();
        unsafe {
            let (a, b) = self.upcast();
            let x =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(a.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            let y =  _mm256_hadd_epi32(
                _mm256_hadd_epi32(b.be_i32s(), i32x8::splat(0)), i32x8::splat(0));
            (x.extract(0) + x.extract(4) + y.extract(0) + y.extract(4)) as i64
        }
    }
}

#[cfg(target_feature = "avx2")]
impl Sum for i32x8 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        let x = unsafe {
            _mm256_hadd_epi32(
                _mm256_hadd_epi32(*self, Self::splat(0)), Self::splat(0))
        };
        x.extract(0) + x.extract(4)
    }
}

#[cfg(target_feature = "sse3")]
impl Sum for f32x4 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        unsafe {
            let x = _mm_hadd_ps(*self, *self);
            let x = _mm_hadd_ps(x, x);
            x.extract(0)
        }
    }
}

#[cfg(target_feature = "sse3")]
impl Sum for f64x2 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        unsafe {
            let x = _mm_hadd_pd(*self, *self);
            x.extract(0)
        }
    }
}

#[cfg(target_feature = "avx")]
impl Sum for f32x8 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        unsafe {
            let x = _mm256_hadd_ps(*self, *self);
            let x = _mm256_hadd_ps(x, x);
            x.extract(0) + x.extract(4)
        }
    }
}

#[cfg(target_feature = "avx")]
impl Sum for f64x4 {
    #[inline(always)]
    fn sum(&self) -> Self::Scalar {
        optimized!();
        unsafe {
            let x = _mm256_hadd_pd(*self, *self);
            x.extract(0) + x.extract(2)
        }
    }
}

impl_packed_sum!(u8x64, i8x64, u16x32, u16x8, i16x32, u32x16, u32x8, u32x4, i32x16, i32x4, f32x16, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8);
impl_packed_upcast_sum!(u8x64, i8x64, u16x32, u16x8, i16x32, i16x8, u32x16, u32x8, u32x4, i32x16, i32x8, i32x4, f32x16, f32x8, f32x4, u64x8, u64x4, u64x2, i64x8, i64x4, i64x2, f64x8, f64x4, f64x2);

#[cfg(not(target_feature = "avx2"))]
impl_packed_sum!(i8x32, u8x32, i16x16, u16x16, i32x8);
#[cfg(not(target_feature = "avx2"))]
impl_packed_upcast_sum!(i8x32, u8x32, i16x16, u16x16);

#[cfg(not(target_feature = "avx"))]
impl_packed_sum!(f32x8, f64x4);

#[cfg(not(target_feature = "sse3"))]
impl_packed_sum!(f32x4, f64x2);

#[cfg(not(target_feature = "sse2"))]
impl_packed_sum!(i8x16, u8x16);
#[cfg(not(target_feature = "sse2"))]
impl_packed_upcast_sum!(i8x16, u8x16);

#[cfg(not(target_feature = "ssse3"))]
impl_packed_sum!(i16x8);
#[cfg(not(target_feature = "ssse3"))]
impl_packed_upcast_sum!();

mod tests {
    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    test_packed_sum_int!(u8x64, u8, test_packed_sum_u8x64);
    test_packed_sum_int!(u8x32, u8, test_packed_sum_u8x32);
    test_packed_sum_int!(u8x16, u8, test_packed_sum_u8x16);
    test_packed_sum_int!(i8x64, i8, test_packed_sum_i8x64);
    test_packed_sum_int!(i8x32, i8, test_packed_sum_i8x32);
    test_packed_sum_int!(i8x16, i8, test_packed_sum_i8x16);
    test_packed_sum_int!(u16x32, u16, test_packed_sum_u16x32);
    test_packed_sum_int!(u16x16, u16, test_packed_sum_u16x16);
    test_packed_sum_int!(u16x8, u16, test_packed_sum_u16x8);
    test_packed_sum_int!(i16x32, i16, test_packed_sum_i16x32);
    test_packed_sum_int!(i16x16, i16, test_packed_sum_i16x16);
    test_packed_sum_int!(i16x8, i16, test_packed_sum_i16x8);
    test_packed_sum_int!(u32x16, u32, test_packed_sum_u32x16);
    test_packed_sum_int!(u32x8, u32, test_packed_sum_u32x8);
    test_packed_sum_int!(u32x4, u32, test_packed_sum_u32x4);
    test_packed_sum_int!(i32x16, i32, test_packed_sum_i32x16);
    test_packed_sum_int!(i32x8, i32, test_packed_sum_i32x8);
    test_packed_sum_int!(i32x4, i32, test_packed_sum_i32x4);
    test_packed_sum_int!(u64x8, u64, test_packed_sum_u64x8);
    test_packed_sum_int!(u64x4, u64, test_packed_sum_u64x4);
    test_packed_sum_int!(u64x2, u64, test_packed_sum_u64x2);
    test_packed_sum_int!(i64x8, i64, test_packed_sum_i64x8);
    test_packed_sum_int!(i64x4, i64, test_packed_sum_i64x4);
    test_packed_sum_int!(i64x2, i64, test_packed_sum_i64x2);

    test_packed_sum!(f32x16, f32, test_packed_sum_f32x16);
    test_packed_sum!(f32x8, f32, test_packed_sum_f32x8);
    test_packed_sum!(f32x4, f32, test_packed_sum_f32x4);

    test_packed_sum!(f64x8, f64, test_packed_sum_f64x8);
    test_packed_sum!(f64x4, f64, test_packed_sum_f64x4);
    test_packed_sum!(f64x2, f64, test_packed_sum_f64x2);
}
