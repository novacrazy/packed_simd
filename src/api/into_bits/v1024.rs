//! `FromBits` and `IntoBits` implementations for portable 1024-bit wide vectors
#![rustfmt::skip]

#[allow(unused)]  // wasm_bindgen_test
use crate::*;

impl_from_bits!(i8x128[test_v1024]: u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(u8x128[test_v1024]: i8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(m8x128[test_v1024]: m16x64, m32x32, m64x16, m128x8);

impl_from_bits!(i16x64[test_v1024]: i8x128, u8x128, m8x128, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(u16x64[test_v1024]: i8x128, u8x128, m8x128, i16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(m16x64[test_v1024]: m32x32, m64x16, m128x8);

impl_from_bits!(i32x32[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(u32x32[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(f32x32[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(m32x32[test_v1024]: m64x16, m128x8);

impl_from_bits!(i64x16[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, u64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(u64x16[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, f64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(f64x16[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, m64x16, i128x8, u128x8, m128x8);
impl_from_bits!(m64x16[test_v1024]: m128x8);

impl_from_bits!(i128x8[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, u128x8, m128x8);
impl_from_bits!(u128x8[test_v1024]: i8x128, u8x128, m8x128, i16x64, u16x64, m16x64, i32x32, u32x32, f32x32, m32x32, i64x16, u64x16, f64x16, m64x16, i128x8, m128x8);
