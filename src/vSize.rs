//! Vectors with pointer-sized elements

use crate::codegen::pointer_sized_int::{isize_, usize_};
use crate::*;

impl_i!([isize; 2]: isizex2, msizex2 | isize_, u8 | test_v128 |
        x0, x1|
        From: |
        /// A vector with 2 `isize` lanes.
);

impl_u!([usize; 2]: usizex2, msizex2 | usize_, u8 | test_v128 |
        x0, x1|
        From: |
        /// A vector with 2 `usize` lanes.
);
impl_m!([msize; 2]: msizex2 | isize_, u8 | test_v128 |
        x0, x1 |
        From: |
        /// A vector mask with 2 `msize` lanes.
);

impl_i!([isize; 4]: isizex4, msizex4 | isize_, u8 | test_v256 |
        x0, x1, x2, x3 |
        From: |
        /// A vector with 4 `isize` lanes.
);
impl_u!([usize; 4]: usizex4, msizex4 | usize_, u8 | test_v256 |
        x0, x1, x2, x3|
        From: |
        /// A vector with 4 `usize` lanes.
);
impl_m!([msize; 4]: msizex4 | isize_, u8 | test_v256 |
        x0, x1, x2, x3 |
        From: |
        /// A vector mask with 4 `msize` lanes.
);

impl_i!([isize; 8]: isizex8, msizex8 | isize_, u8 | test_v512 |
        x0, x1, x2, x3, x4, x5, x6, x7 |
        From: |
        /// A vector with 8 `isize` lanes.
);
impl_u!([usize; 8]: usizex8, msizex8 | usize_, u8 | test_v512 |
        x0, x1, x2, x3, x4, x5, x6, x7 |
        From: |
        /// A vector with 8 `usize` lanes.
);
impl_m!([msize; 8]: msizex8 | isize_, u8 | test_v512 |
        x0, x1, x2, x3, x4, x5, x6, x7 |
        From: |
        /// A vector mask with 8 `msize` lanes.
);

impl_i!([isize; 16]: isizex16, msizex16 | isize_, u16 | test_v1024 |
        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        From: |
        /// A vector with 16 `isize` lanes.
);
impl_u!([usize; 16]: usizex16, msizex16 | usize_, u16 | test_v1024 |
        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        From: |
        /// A vector with 16 `usize` lanes.
);
impl_m!([msize; 16]: msizex16 | isize_, u16 | test_v1024 |
        x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        From: |
        /// A vector mask with 16 `msize` lanes.
);
