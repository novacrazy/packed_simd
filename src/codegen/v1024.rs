//! Internal 1024-bit wide vector types

use crate::masks::*;

#[rustfmt::skip]
impl_simd_array!(
    [i8; 128]: i8x128 |
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,

    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,

    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,

    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8
);
#[rustfmt::skip]
impl_simd_array!(
    [u8; 128]: u8x128 |
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,

    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,

    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,

    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8,
    u8, u8, u8, u8
);
#[rustfmt::skip]
impl_simd_array!(
    [m8; 128]: m8x128 |
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,

    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,

    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,

    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8,
    i8, i8, i8, i8
);
#[rustfmt::skip]
impl_simd_array!(
    [i16; 64]: i16x64 |
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,

    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16
);
#[rustfmt::skip]
impl_simd_array!(
    [u16; 64]: u16x64 |
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,

    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16,
    u16, u16, u16, u16
);
#[rustfmt::skip]
impl_simd_array!(
    [m16; 64]: m16x64 |
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,

    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16,
    i16, i16, i16, i16
);

#[rustfmt::skip]
impl_simd_array!(
    [i32; 32]: i32x32 |
    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32,

    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32
);
#[rustfmt::skip]
impl_simd_array!(
    [u32; 32]: u32x32 |
    u32, u32, u32, u32,
    u32, u32, u32, u32,
    u32, u32, u32, u32,
    u32, u32, u32, u32,

    u32, u32, u32, u32,
    u32, u32, u32, u32,
    u32, u32, u32, u32,
    u32, u32, u32, u32
);
#[rustfmt::skip]
impl_simd_array!(
    [f32; 32]: f32x32 |
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,

    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32,
    f32, f32, f32, f32
);
#[rustfmt::skip]
impl_simd_array!(
    [m32; 32]: m32x32 |
    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32,

    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32,
    i32, i32, i32, i32
);

#[rustfmt::skip]
impl_simd_array!([i64; 16]: i64x16 | i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64);
#[rustfmt::skip]
impl_simd_array!([u64; 16]: u64x16 | u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64);
#[rustfmt::skip]
impl_simd_array!([f64; 16]: f64x16 | f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);
#[rustfmt::skip]
impl_simd_array!([m64; 16]: m64x16 | i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64);

#[rustfmt::skip]
impl_simd_array!([i128; 8]: i128x8 | i128, i128, i128, i128, i128, i128, i128, i128);
#[rustfmt::skip]
impl_simd_array!([u128; 8]: u128x8 | u128, u128, u128, u128, u128, u128, u128, u128);
#[rustfmt::skip]
impl_simd_array!([m128; 8]: m128x8 | i128, i128, i128, i128, i128, i128, i128, i128);
