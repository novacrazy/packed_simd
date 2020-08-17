//! 1024-bit wide vector types
#![rustfmt::skip]

use crate::*;

impl_i!([i8; 128]: i8x128, m8x128 | i8, u128 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31,
    x32,  x33,  x34,  x35,  x36,  x37,  x38,  x39,  x40,  x41,  x42,  x43,  x44,  x45,  x46,  x47,
    x48,  x49,  x50,  x51,  x52,  x53,  x54,  x55,  x56,  x57,  x58,  x59,  x60,  x61,  x62,  x63,
    x64,  x65,  x66,  x67,  x68,  x69,  x70,  x71,  x72,  x73,  x74,  x75,  x76,  x77,  x78,  x79,
    x80,  x81,  x82,  x83,  x84,  x85,  x86,  x87,  x88,  x89,  x90,  x91,  x92,  x93,  x94,  x95,
    x96,  x97,  x98,  x99,  x100, x101, x102, x103, x104, x105, x106, x107, x108, x109, x110, x111,
    x112, x113, x114, x115, x116, x117, x118, x119, x120, x121, x122, x123, x124, x125, x126, x127 |
    From: |
    /// A 1024-bit vector with 128 `i8` lanes.
);
impl_u!([u8; 128]: u8x128, m8x128 | u8, u128 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31,
    x32,  x33,  x34,  x35,  x36,  x37,  x38,  x39,  x40,  x41,  x42,  x43,  x44,  x45,  x46,  x47,
    x48,  x49,  x50,  x51,  x52,  x53,  x54,  x55,  x56,  x57,  x58,  x59,  x60,  x61,  x62,  x63,
    x64,  x65,  x66,  x67,  x68,  x69,  x70,  x71,  x72,  x73,  x74,  x75,  x76,  x77,  x78,  x79,
    x80,  x81,  x82,  x83,  x84,  x85,  x86,  x87,  x88,  x89,  x90,  x91,  x92,  x93,  x94,  x95,
    x96,  x97,  x98,  x99,  x100, x101, x102, x103, x104, x105, x106, x107, x108, x109, x110, x111,
    x112, x113, x114, x115, x116, x117, x118, x119, x120, x121, x122, x123, x124, x125, x126, x127 |
    From: |
    /// A 1024-bit vector with 128 `u8` lanes.
);
impl_m!([m8; 128]: m8x128 | i8, u128 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31,
    x32,  x33,  x34,  x35,  x36,  x37,  x38,  x39,  x40,  x41,  x42,  x43,  x44,  x45,  x46,  x47,
    x48,  x49,  x50,  x51,  x52,  x53,  x54,  x55,  x56,  x57,  x58,  x59,  x60,  x61,  x62,  x63,
    x64,  x65,  x66,  x67,  x68,  x69,  x70,  x71,  x72,  x73,  x74,  x75,  x76,  x77,  x78,  x79,
    x80,  x81,  x82,  x83,  x84,  x85,  x86,  x87,  x88,  x89,  x90,  x91,  x92,  x93,  x94,  x95,
    x96,  x97,  x98,  x99,  x100, x101, x102, x103, x104, x105, x106, x107, x108, x109, x110, x111,
    x112, x113, x114, x115, x116, x117, x118, x119, x120, x121, x122, x123, x124, x125, x126, x127 |
    From:  |
    /// A 1024-bit vector mask with 128 `m8` lanes.
);

impl_i!([i16; 64]: i16x64, m16x64 | i16, u64 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31,
    x32,  x33,  x34,  x35,  x36,  x37,  x38,  x39,  x40,  x41,  x42,  x43,  x44,  x45,  x46,  x47,
    x48,  x49,  x50,  x51,  x52,  x53,  x54,  x55,  x56,  x57,  x58,  x59,  x60,  x61,  x62,  x63 |
    From: i8x64, u8x64 |
    /// A 1024-bit vector with 64 `i16` lanes.
);
impl_u!([u16; 64]: u16x64, m16x64 | u16, u64 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31,
    x32,  x33,  x34,  x35,  x36,  x37,  x38,  x39,  x40,  x41,  x42,  x43,  x44,  x45,  x46,  x47,
    x48,  x49,  x50,  x51,  x52,  x53,  x54,  x55,  x56,  x57,  x58,  x59,  x60,  x61,  x62,  x63 |
    From: u8x64 |
    /// A 1024-bit vector with 64 `u16` lanes.
);
impl_m!([m16; 64]: m16x64 | i16, u64 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31,
    x32,  x33,  x34,  x35,  x36,  x37,  x38,  x39,  x40,  x41,  x42,  x43,  x44,  x45,  x46,  x47,
    x48,  x49,  x50,  x51,  x52,  x53,  x54,  x55,  x56,  x57,  x58,  x59,  x60,  x61,  x62,  x63 |
    From: m8x64 |
    /// A 1024-bit vector mask with 64 `m16` lanes.
);

impl_i!([i32; 32]: i32x32, m32x32 | i32, u32 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31 |
    From: i8x32, u8x32, i16x32, u16x32 |
    /// A 1024-bit vector with 32 `i32` lanes.
);
impl_u!([u32; 32]: u32x32, m32x32 | u32, u32 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31 |
    From: u8x32, u16x32 |
    /// A 1024-bit vector with 32 `u32` lanes.
);
impl_f!([f32; 32]: f32x32, m32x32 | f32 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31 |
    From: i8x32, u8x32, i16x32, u16x32 |
    /// A 1024-bit vector with 32 `f32` lanes.
);
impl_m!([m32; 32]: m32x32 | i32, u32 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15,
    x16,  x17,  x18,  x19,  x20,  x21,  x22,  x23,  x24,  x25,  x26,  x27,  x28,  x29,  x30,  x31 |
    From: m8x32, m16x32 |
    /// A 1024-bit vector mask with 32 `m32` lanes.
);

impl_i!([i64; 16]: i64x16, m64x16 | i64, u16 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15 |
    From: i8x16, u8x16, i16x16, u16x16, i32x16, u32x16 |
    /// A 1024-bit vector with 16 `i64` lanes.
);
impl_u!([u64; 16]: u64x16, m64x16 | u64, u16 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15 |
    From: u8x16, u16x16, u32x16 |
    /// A 1024-bit vector with 16 `u64` lanes.
);
impl_f!([f64; 16]: f64x16, m64x16 | f64 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15 |
    From: i8x16, u8x16, i16x16, u16x16, i32x16, u32x16, f32x16 |
    /// A 1024-bit vector with 16 `f64` lanes.
);
impl_m!([m64; 16]: m64x16 | i64, u16 | test_v1024 |
    x0,   x1,   x2,   x3,   x4,   x5,   x6,   x7,   x8,   x9,   x10,  x11,  x12,  x13,  x14,  x15 |
    From: m8x16, m16x16, m32x16 |
    /// A 1024-bit vector mask with 16 `m64` lanes.
);

impl_i!([i128; 8]: i128x8, m128x8 | i128, u8 | test_v1024 | x0, x1, x2, x3, x4, x5, x6, x7 |
    From: i8x8, u8x8, i16x8, u16x8, i32x8, u32x8, i64x8, u64x8 |
    /// A 1024-bit vector with 8 `i128` lanes.
);
impl_u!([u128; 8]: u128x8, m128x8 | u128, u8 | test_v1024 | x0, x1, x2, x3, x4, x5, x6, x7 |
    From: u8x8, u16x8, u32x8, u64x8 |
    /// A 1024-bit vector with 8 `u128` lanes.
);
impl_m!([m128; 8]: m128x8 | i128, u8 | test_v1024 | x0, x1, x2, x3, x4, x5, x6, x7 |
    From: m8x8, m16x8, m32x8, m64x8 |
    /// A 1024-bit vector mask with 8 `m128` lanes.
);
