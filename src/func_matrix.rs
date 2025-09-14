use crate::{mat2, mat3, vec2};

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

        impl mat3<$t> {
            #[inline]
            pub const fn inverse(&self) -> Self {
                let one_over_determinant = 1 as $t / (
                      self.value[0].x * (self.value[1].y * self.value[2].z - self.value[2].y * self.value[1].z)
                    - self.value[1].x * (self.value[0].y * self.value[2].z - self.value[2].y * self.value[0].z)
                    + self.value[2].x * (self.value[0].y * self.value[1].z - self.value[1].y * self.value[0].z));

                let mut inverse = Self::identity();
                inverse.value[0].x =  (self.value[1].y * self.value[2].z - self.value[2].y * self.value[1].z);
                inverse.value[1].x = -(self.value[1].x * self.value[2].z - self.value[2].x * self.value[1].z);
                inverse.value[2].x =  (self.value[1].x * self.value[2].y - self.value[2].x * self.value[1].y);
                inverse.value[0].y = -(self.value[0].y * self.value[2].z - self.value[2].y * self.value[0].z);
                inverse.value[1].y =  (self.value[0].x * self.value[2].z - self.value[2].x * self.value[0].z);
                inverse.value[2].y = -(self.value[0].x * self.value[2].y - self.value[2].x * self.value[0].y);
                inverse.value[0].z =  (self.value[0].y * self.value[1].z - self.value[1].y * self.value[0].z);
                inverse.value[1].z = -(self.value[0].x * self.value[1].z - self.value[1].x * self.value[0].z);
                inverse.value[2].z =  (self.value[0].x * self.value[1].y - self.value[1].x * self.value[0].y);

                inverse.value[0].x *= one_over_determinant;
                inverse.value[0].y *= one_over_determinant;
                inverse.value[0].z *= one_over_determinant;
                inverse.value[1].x *= one_over_determinant;
                inverse.value[1].y *= one_over_determinant;
                inverse.value[1].z *= one_over_determinant;
                inverse.value[2].x *= one_over_determinant;
                inverse.value[2].y *= one_over_determinant;
                inverse.value[2].z *= one_over_determinant;
                inverse
            }
        }
    )*)
}

inverse_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
