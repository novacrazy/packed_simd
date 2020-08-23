//! Implements vertical (lane-wise) floating-point `log2`.

macro_rules! impl_math_float_log2 {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Returns the natural logarithm of `self`.
            #[inline]
            pub fn log2(self) -> Self {
                use crate::codegen::math::float::log2::Log2;
                Log2::log2(self)
            }
        }

        test_if! {
            $test_tt:
            paste::item! {
                pub mod [<$id _math_log2>] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn log2() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        assert_eq!(z, o.log2());

                        let e = $id::splat(crate::f64::consts::E as $elem_ty);
                        let tol = $id::splat(2.4e-4 as $elem_ty);
                        assert!((o - e.log2()).abs().le(tol).all());
                    }
                }
            }
        }
    };
}
