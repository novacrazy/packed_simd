//! `FromCast` and `IntoCast` implementations for portable 1024-bit wide vectors
#![rustfmt::skip]

use crate::*;

impl_from_cast!(i8x128[test_v1024]: u8x128, m8x128);
impl_from_cast!(u8x128[test_v1024]: i8x128, m8x128);
impl_from_cast_mask!(m8x128[test_v1024]: i8x128, u8x128);

impl_from_cast!(i16x64[test_v1024]: i8x64, u8x64, m8x64, u16x64, m16x64);
impl_from_cast!(u16x64[test_v1024]: i8x64, u8x64, m8x64, i16x64, m16x64);
impl_from_cast_mask!(m16x64[test_v1024]: i8x64, u8x64, m8x64, i16x64, u16x64);

impl_from_cast!(
    i32x32[test_v1024]: i8x32, u8x32, m8x32, i16x32, u16x32, m16x32, u32x32, f32x32, m32x32
);
impl_from_cast!(
    u32x32[test_v1024]: i8x32, u8x32, m8x32, i16x32, u16x32, m16x32, i32x32, f32x32, m32x32
);
impl_from_cast!(
    f32x32[test_v1024]: i8x32, u8x32, m8x32, i16x32, u16x32, m16x32, i32x32, u32x32, m32x32
);
impl_from_cast_mask!(
    m32x32[test_v1024]: i8x32, u8x32, m8x32, i16x32, u16x32, m16x32, i32x32, u32x32, f32x32
);

impl_from_cast!(
    i64x16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    u64x16, f64x16, m64x16, isizex16, usizex16, msizex16
);
impl_from_cast!(
    u64x16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    i64x16, f64x16, m64x16, isizex16, usizex16, msizex16
);
impl_from_cast!(
    f64x16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    i64x16, u64x16, m64x16, isizex16, usizex16, msizex16
);
impl_from_cast_mask!(
    m64x16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    i64x16, u64x16, f64x16, isizex16, usizex16, msizex16
);

impl_from_cast!(
    i128x8[test_v1024]: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
    i64x8, u64x8, f64x8, m64x8, u128x8, m128x8, isizex8, usizex8, msizex8
);
impl_from_cast!(
    u128x8[test_v1024]: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
    i64x8, u64x8, f64x8, m64x8, i128x8, m128x8, isizex8, usizex8, msizex8
);
impl_from_cast_mask!(
    m128x8[test_v1024]: i8x8, u8x8, m8x8, i16x8, u16x8, m16x8, i32x8, u32x8, f32x8, m32x8,
    i64x8, u64x8, m64x8, f64x8, i128x8, u128x8, isizex8, usizex8, msizex8
);

impl_from_cast!(
    isizex16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    i64x16, u64x16, f64x16, m64x16, usizex16, msizex16
);
impl_from_cast!(
    usizex16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    i64x16, u64x16, f64x16, m64x16, isizex16, msizex16
);
impl_from_cast_mask!(
    msizex16[test_v1024]: i8x16, u8x16, m8x16, i16x16, u16x16, m16x16, i32x16, u32x16, f32x16, m32x16,
    i64x16, u64x16, f64x16, m64x16, isizex16, usizex16
);
