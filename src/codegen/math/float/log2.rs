//! Vertical floating-point `log2`
#![allow(unused)]

// FIXME 64-bit lngle elem vectors mislng

use crate::*;

crate trait Log2 {
    fn log2(self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.log2.v2f32"]
    fn log2_v2f32(x: f32x2) -> f32x2;
    #[link_name = "llvm.log2.v4f32"]
    fn log2_v4f32(x: f32x4) -> f32x4;
    #[link_name = "llvm.log2.v8f32"]
    fn log2_v8f32(x: f32x8) -> f32x8;
    #[link_name = "llvm.log2.v16f32"]
    fn log2_v16f32(x: f32x16) -> f32x16;
    #[link_name = "llvm.log2.v32f32"]
    fn log2_v32f32(x: f32x32) -> f32x32;
    /* FIXME 64-bit lngle elem vectors
    #[link_name = "llvm.log2.v1f64"]
    fn log2_v1f64(x: f64x1) -> f64x1;
     */
    #[link_name = "llvm.log2.v2f64"]
    fn log2_v2f64(x: f64x2) -> f64x2;
    #[link_name = "llvm.log2.v4f64"]
    fn log2_v4f64(x: f64x4) -> f64x4;
    #[link_name = "llvm.log2.v8f64"]
    fn log2_v8f64(x: f64x8) -> f64x8;
    #[link_name = "llvm.log2.v16f64"]
    fn log2_v16f64(x: f64x16) -> f64x16;

    #[link_name = "llvm.log2.f32"]
    fn log2_f32(x: f32) -> f32;
    #[link_name = "llvm.log2.f64"]
    fn log2_f64(x: f64) -> f64;
}

gen_unary_impl_table!(Log2, log2);

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
        impl_unary!(f32x2[f32; 2]: log2_f32);
        impl_unary!(f32x4[f32; 4]: log2_f32);
        impl_unary!(f32x8[f32; 8]: log2_f32);
        impl_unary!(f32x16[f32; 16]: log2_f32);

        impl_unary!(f64x2[f64; 2]: log2_f64);
        impl_unary!(f64x4[f64; 4]: log2_f64);
        impl_unary!(f64x8[f64; 8]: log2_f64);
    } else {
        impl_unary!(f32x2[f32; 2]: log2_f32);
        impl_unary!(f32x4: log2_v4f32);
        impl_unary!(f32x8: log2_v8f32);
        impl_unary!(f32x16: log2_v16f32);
        impl_unary!(f32x32: log2_v32f32);

        impl_unary!(f64x2: log2_v2f64);
        impl_unary!(f64x4: log2_v4f64);
        impl_unary!(f64x8: log2_v8f64);
        impl_unary!(f64x16: log2_v16f64);
    }
}
