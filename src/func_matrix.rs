use crate::{mat2, vec2};

macro_rules! inverse_impl {
    ($($t:ty)*) => ($(
        impl mat2<$t> {
            #[inline]
            pub const fn inverse(&self) -> Self {
                let one_over_determinant = 1 as $t / (
                    self.value[0].x * self.value[1].y
                    - self.value[1].x * self.value[0].y);

                let inverse = Self::new([
                    vec2::new(  self.value[1].y * one_over_determinant,
                              - self.value[0].y * one_over_determinant),
                    vec2::new(- self.value[1].x * one_over_determinant,
                                self.value[0].x * one_over_determinant)]);

                inverse
            }
        }
    )*)
}

inverse_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
