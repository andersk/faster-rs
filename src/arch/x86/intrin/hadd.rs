// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vektor::x86_64::*;
use crate::vektor::x86::*;
use crate::intrin::transmute::*;
use crate::intrin::hadd::*;
use crate::std::ops::Add;
use crate::arch::current::vecs::*;
use crate::vecs::*;

#[cfg(target_feature = "sse3")]
impl HAdd for f32x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm_hadd_ps(_mm_shuffle_ps(*self, other, 0b01000100),
                             _mm_shuffle_ps(*self, other, 0b11101110)) }
    }
}

#[cfg(target_feature = "sse3")]
impl HAdd for f64x2 {
    #[inline(always)]
    #[cfg(target_feature = "sse3")]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm_hadd_pd(*self, other) }
    }
}

#[cfg(target_feature = "avx2")]
impl HAdd for f32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm256_hadd_ps(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked(),
                                _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_f32s_unchecked()) }
    }
}

#[cfg(target_feature = "avx")]
impl HAdd for f64x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm256_hadd_pd(*self, other) }
    }
}

#[cfg(target_feature = "ssse3")]
impl HAdd for i16x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm_hadd_epi16(_mm_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                _mm_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "ssse3")]
impl HAdd for i32x4 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm_hadd_epi32(_mm_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                _mm_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl HAdd for i16x16 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm256_hadd_epi16(_mm256_unpacklo_epi32(self.be_i32s(), other.be_i32s()).be_i16s(),
                                   _mm256_unpackhi_epi32(self.be_i32s(), other.be_i32s()).be_i16s()) }
    }
}

#[cfg(target_feature = "avx2")]
impl HAdd for i32x8 {
    #[inline(always)]
    fn hadd(&self, other: Self) -> Self {
        optimized!();
        unsafe { _mm256_hadd_epi32(_mm256_unpacklo_epi64(self.be_i64s(), other.be_i64s()).be_i32s(),
                                   _mm256_unpackhi_epi64(self.be_i64s(), other.be_i64s()).be_i32s()) }
    }
}

impl HAdd for u64x2 { hop!(hadd, Add::add, 0, 1); }
impl HAdd for u64x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl HAdd for u64x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for u32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl HAdd for u32x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for u32x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for u16x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for u16x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for u16x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HAdd for u8x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for u8x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HAdd for u8x64 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
impl HAdd for i64x2 { hop!(hadd, Add::add, 0, 1); }
impl HAdd for i64x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl HAdd for i64x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "ssse3"))]
impl HAdd for i32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
#[cfg(not(target_feature = "avx2"))]
impl HAdd for i32x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for i32x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
#[cfg(not(target_feature = "ssse3"))]
impl HAdd for i16x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "avx2"))]
impl HAdd for i16x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for i16x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HAdd for i8x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
impl HAdd for i8x32 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31); }
impl HAdd for i8x64 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63); }
#[cfg(not(target_feature = "sse3"))]
impl HAdd for f64x2 { hop!(hadd, Add::add, 0, 1); }
#[cfg(not(target_feature = "avx"))]
impl HAdd for f64x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
impl HAdd for f64x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
#[cfg(not(target_feature = "sse3"))]
impl HAdd for f32x4 { hop!(hadd, Add::add, 0, 1, 2, 3); }
#[cfg(not(target_feature = "avx2"))]
impl HAdd for f32x8 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7); }
impl HAdd for f32x16 { hop!(hadd, Add::add, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15); }
