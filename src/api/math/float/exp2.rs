//! Implements vertical (lane-wise) floating-point `exp2`.

macro_rules! impl_math_float_exp2 {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Returns the base-2 exponential function of `self`: `2^(self)`.
            #[inline]
            pub fn exp2(self) -> Self {
                use crate::codegen::math::float::exp2::Exp2;
                Exp2::exp2(self)
            }
        }

        test_if! {
            $test_tt:
            paste::item! {
                pub mod [<$id _math_exp2>] {
                    use super::*;
                    #[cfg_attr(not(target_arch = "wasm32"), test)] #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
                    fn exp2() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        assert_eq!(o, z.exp2());

                        let e = $id::splat(crate::f64::consts::E as $elem_ty);
                        let tol = $id::splat(2.4e-4 as $elem_ty);
                        assert!((e - o.exp2()).abs().le(tol).all());
                    }
                }
            }
        }
    };
}
