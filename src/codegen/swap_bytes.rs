//! Horizontal swap bytes reductions.

// FIXME: investigate using `llvm.bswap`
// https://github.com/rust-lang-nursery/packed_simd/issues/19

use crate::*;

crate trait SwapBytes {
    fn swap_bytes(self) -> Self;
}

macro_rules! impl_swap_bytes {
    (v16: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                fn swap_bytes(self) -> Self {
                    unsafe { shuffle!(self, [1, 0]) }
                }
            }
        )+
    };
    (v32: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[allow(clippy::useless_transmute)]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x4 = crate::mem::transmute(self);
                        let result: u8x4 = shuffle!(bytes, [3, 2, 1, 0]);
                        crate::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v64: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[allow(clippy::useless_transmute)]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x8 = crate::mem::transmute(self);
                        let result: u8x8 = shuffle!(
                            bytes, [7, 6, 5, 4, 3, 2, 1, 0]
                        );
                        crate::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v128: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[allow(clippy::useless_transmute)]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x16 = crate::mem::transmute(self);
                        let result: u8x16 = shuffle!(bytes, [
                            15, 14, 13, 12, 11, 10, 9, 8,
                            7, 6, 5, 4, 3, 2, 1, 0
                        ]);
                        crate::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v256: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[allow(clippy::useless_transmute)]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x32 = crate::mem::transmute(self);
                        let result: u8x32 = shuffle!(bytes, [
                            31, 30, 29, 28, 27, 26, 25, 24,
                            23, 22, 21, 20, 19, 18, 17, 16,
                            15, 14, 13, 12, 11, 10, 9,  8,
                            7,  6,  5,  4,  3,  2,  1,  0
                        ]);
                        crate::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v512: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[allow(clippy::useless_transmute)]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x64 = crate::mem::transmute(self);
                        let result: u8x64 = shuffle!(bytes, [
                            63, 62, 61, 60, 59, 58, 57, 56,
                            55, 54, 53, 52, 51, 50, 49, 48,
                            47, 46, 45, 44, 43, 42, 41, 40,
                            39, 38, 37, 36, 35, 34, 33, 32,
                            31, 30, 29, 28, 27, 26, 25, 24,
                            23, 22, 21, 20, 19, 18, 17, 16,
                            15, 14, 13, 12, 11, 10, 9,  8,
                            7,  6,  5,  4,  3,  2,  1,  0
                        ]);
                        crate::mem::transmute(result)
                    }
                }
            }
        )+
    };
    (v1024: $($id:ident,)+) => {
        $(
            impl SwapBytes for $id {
                #[inline]
                #[allow(clippy::useless_transmute)]
                fn swap_bytes(self) -> Self {
                    unsafe {
                        let bytes: u8x128 = crate::mem::transmute(self);
                        let result: u8x128 = shuffle!(bytes, [
                            127, 126, 125, 124, 123, 122, 121, 120,
                            119, 118, 117, 116, 115, 114, 113, 112,
                            111, 110, 109, 108, 107, 106, 105, 104,
                            103, 102, 101, 100, 99,  98,  97,  96,
                            95,  94,  93,  92,  91,  90,  89,  88,
                            87,  86,  85,  84,  83,  82,  81,  80,
                            79,  78,  77,  76,  75,  74,  73,  72,
                            71,  70,  69,  68,  67,  66,  65,  64,
                            63,  62,  61,  60,  59,  58,  57,  56,
                            55,  54,  53,  52,  51,  50,  49,  48,
                            47,  46,  45,  44,  43,  42,  41,  40,
                            39,  38,  37,  36,  35,  34,  33,  32,
                            31,  30,  29,  28,  27,  26,  25,  24,
                            23,  22,  21,  20,  19,  18,  17,  16,
                            15,  14,  13,  12,  11,  10,  9,   8,
                            7,   6,   5,   4,   3,   2,   1,   0
                        ]);
                        crate::mem::transmute(result)
                    }
                }
            }
        )+
    };
}

impl_swap_bytes!(v16: u8x2, i8x2,);
impl_swap_bytes!(v32: u8x4, i8x4, u16x2, i16x2,);
// FIXME: 64-bit single element vector
impl_swap_bytes!(v64: u8x8, i8x8, u16x4, i16x4, u32x2, i32x2 /* u64x1, i64x1, */,);

impl_swap_bytes!(v128: u8x16, i8x16, u16x8, i16x8, u32x4, i32x4, u64x2, i64x2, u128x1, i128x1,);
impl_swap_bytes!(v256: u8x32, i8x32, u16x16, i16x16, u32x8, i32x8, u64x4, i64x4, u128x2, i128x2,);

impl_swap_bytes!(v512: u8x64, i8x64, u16x32, i16x32, u32x16, i32x16, u64x8, i64x8, u128x4, i128x4,);

impl_swap_bytes!(v1024: u8x128, i8x128, u16x64, i16x64, u32x32, i32x32, u64x16, i64x16, u128x8, i128x8,);

cfg_if! {
    if #[cfg(target_pointer_width = "8")] {
        impl_swap_bytes!(v16: isizex2, usizex2,);
        impl_swap_bytes!(v32: isizex4, usizex4,);
        impl_swap_bytes!(v64: isizex8, usizex8,);
        impl_swap_bytes!(v128: isizex16, usizex16,);
    } else if #[cfg(target_pointer_width = "16")] {
        impl_swap_bytes!(v32: isizex2, usizex2,);
        impl_swap_bytes!(v64: isizex4, usizex4,);
        impl_swap_bytes!(v128: isizex8, usizex8,);
        impl_swap_bytes!(v256: isizex16, usizex16,);
    } else if #[cfg(target_pointer_width = "32")] {
        impl_swap_bytes!(v64: isizex2, usizex2,);
        impl_swap_bytes!(v128: isizex4, usizex4,);
        impl_swap_bytes!(v256: isizex8, usizex8,);
        impl_swap_bytes!(v512: isizex16, usizex16,);
    } else if #[cfg(target_pointer_width = "64")] {
        impl_swap_bytes!(v128: isizex2, usizex2,);
        impl_swap_bytes!(v256: isizex4, usizex4,);
        impl_swap_bytes!(v512: isizex8, usizex8,);
        impl_swap_bytes!(v1024: isizex16, usizex16,);
    } else {
        compile_error!("unsupported target_pointer_width");
    }
}
