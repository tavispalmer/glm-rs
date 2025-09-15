use crate::{mat2, mat3, mat4, vec2, vec4};

macro_rules! inverse_impl {
    ($($t:ty)*) => ($(
        impl mat2<$t> {
            #[inline]
            pub const fn inverse(&self) -> Self {
                let one_over_determinant = 1 as $t / (
                    self.value[0].x * self.value[1].y
                    - self.value[1].x * self.value[0].y);

                let inverse = Self::new(
                    vec2::new(  self.value[1].y * one_over_determinant,
                              - self.value[0].y * one_over_determinant),
                    vec2::new(- self.value[1].x * one_over_determinant,
                                self.value[0].x * one_over_determinant));

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

        impl mat4<$t> {
            #[inline]
            pub const fn inverse(&self) -> Self {
                let coef00 = self.value[2].z * self.value[3].w - self.value[3].z * self.value[2].w;
                let coef02 = self.value[1].z * self.value[3].w - self.value[3].z * self.value[1].w;
                let coef03 = self.value[1].z * self.value[2].w - self.value[2].z * self.value[1].w;

                let coef04 = self.value[2].y * self.value[3].w - self.value[3].y * self.value[2].w;
                let coef06 = self.value[1].y * self.value[3].w - self.value[3].y * self.value[1].w;
                let coef07 = self.value[1].y * self.value[2].w - self.value[2].y * self.value[1].w;

                let coef08 = self.value[2].y * self.value[3].z - self.value[3].y * self.value[2].z;
                let coef10 = self.value[1].y * self.value[3].z - self.value[3].y * self.value[1].z;
                let coef11 = self.value[1].y * self.value[2].z - self.value[2].y * self.value[1].z;

                let coef12 = self.value[2].x * self.value[3].w - self.value[3].x * self.value[2].w;
                let coef14 = self.value[1].x * self.value[3].w - self.value[3].x * self.value[1].w;
                let coef15 = self.value[1].x * self.value[2].w - self.value[2].x * self.value[1].w;

                let coef16 = self.value[2].x * self.value[3].z - self.value[3].x * self.value[2].z;
                let coef18 = self.value[1].x * self.value[3].z - self.value[3].x * self.value[1].z;
                let coef19 = self.value[1].x * self.value[2].z - self.value[2].x * self.value[1].z;

                let coef20 = self.value[2].x * self.value[3].y - self.value[3].x * self.value[2].y;
                let coef22 = self.value[1].x * self.value[3].y - self.value[3].x * self.value[1].y;
                let coef23 = self.value[1].x * self.value[2].y - self.value[2].x * self.value[1].y;

                let fac0 = vec4::new(coef00, coef00, coef02, coef03);
                let fac1 = vec4::new(coef04, coef04, coef06, coef07);
                let fac2 = vec4::new(coef08, coef08, coef10, coef11);
                let fac3 = vec4::new(coef12, coef12, coef14, coef15);
                let fac4 = vec4::new(coef16, coef16, coef18, coef19);
                let fac5 = vec4::new(coef20, coef20, coef22, coef23);

                let vec0 = vec4::new(self.value[1].x, self.value[0].x, self.value[0].x, self.value[0].x);
                let vec1 = vec4::new(self.value[1].y, self.value[0].y, self.value[0].y, self.value[0].y);
                let vec2 = vec4::new(self.value[1].z, self.value[0].z, self.value[0].z, self.value[0].z);
                let vec3 = vec4::new(self.value[1].w, self.value[0].w, self.value[0].w, self.value[0].w);

                let inv0 = vec4::new(
                    vec1.x * fac0.x - vec2.x * fac1.x + vec3.x * fac2.x,
                    vec1.y * fac0.y - vec2.y * fac1.y + vec3.y * fac2.y,
                    vec1.z * fac0.z - vec2.z * fac1.z + vec3.z * fac2.z,
                    vec1.w * fac0.w - vec2.w * fac1.w + vec3.w * fac2.w,
                );
                let inv1 = vec4::new(
                    vec0.x * fac0.x - vec2.x * fac3.x + vec3.x * fac4.x,
                    vec0.y * fac0.y - vec2.y * fac3.y + vec3.y * fac4.y,
                    vec0.z * fac0.z - vec2.z * fac3.z + vec3.z * fac4.z,
                    vec0.w * fac0.w - vec2.w * fac3.w + vec3.w * fac4.w,
                );
                let inv2 = vec4::new(
                    vec0.x * fac1.x - vec1.x * fac3.x + vec3.x * fac5.x,
                    vec0.y * fac1.y - vec1.y * fac3.y + vec3.y * fac5.y,
                    vec0.z * fac1.z - vec1.z * fac3.z + vec3.z * fac5.z,
                    vec0.w * fac1.w - vec1.w * fac3.w + vec3.w * fac5.w,
                );
                let inv3 = vec4::new(
                    vec0.x * fac2.x - vec1.x * fac4.x + vec2.x * fac5.x,
                    vec0.y * fac2.y - vec1.y * fac4.y + vec2.y * fac5.y,
                    vec0.z * fac2.z - vec1.z * fac4.z + vec2.z * fac5.z,
                    vec0.w * fac2.w - vec1.w * fac4.w + vec2.w * fac5.w,
                );

                let sign_a = vec4::new( 1 as $t, -1 as $t,  1 as $t, -1 as $t);
                let sign_b = vec4::new(-1 as $t,  1 as $t, -1 as $t,  1 as $t);
                let inverse = mat4::new(
                    vec4::new(
                        inv0.x * sign_a.x,
                        inv0.y * sign_a.y,
                        inv0.z * sign_a.z,
                        inv0.w * sign_a.w,
                    ),
                    vec4::new(
                        inv1.x * sign_b.x,
                        inv1.y * sign_b.y,
                        inv1.z * sign_b.z,
                        inv1.w * sign_b.w,
                    ),
                    vec4::new(
                        inv2.x * sign_a.x,
                        inv2.y * sign_a.y,
                        inv2.z * sign_a.z,
                        inv2.w * sign_a.w,
                    ),
                    vec4::new(
                        inv3.x * sign_b.x,
                        inv3.y * sign_b.y,
                        inv3.z * sign_b.z,
                        inv3.w * sign_b.w,
                    ),
                );

                let row0 = vec4::new(inverse.value[0].x, inverse.value[1].x, inverse.value[2].x, inverse.value[3].x);

                let dot0 = vec4::new(
                    self.value[0].x * row0.x,
                    self.value[0].y * row0.y,
                    self.value[0].z * row0.z,
                    self.value[0].w * row0.w,
                );
                let dot1 = (dot0.x + dot0.y) + (dot0.z + dot0.w);

                let one_over_determinant = 1 as $t / dot1;

                mat4::new(
                    vec4::new(
                        inverse.value[0].x * one_over_determinant,
                        inverse.value[0].y * one_over_determinant,
                        inverse.value[0].z * one_over_determinant,
                        inverse.value[0].w * one_over_determinant,
                    ),
                    vec4::new(
                        inverse.value[1].x * one_over_determinant,
                        inverse.value[1].y * one_over_determinant,
                        inverse.value[1].z * one_over_determinant,
                        inverse.value[1].w * one_over_determinant,
                    ),
                    vec4::new(
                        inverse.value[2].x * one_over_determinant,
                        inverse.value[2].y * one_over_determinant,
                        inverse.value[2].z * one_over_determinant,
                        inverse.value[2].w * one_over_determinant,
                    ),
                    vec4::new(
                        inverse.value[3].x * one_over_determinant,
                        inverse.value[3].y * one_over_determinant,
                        inverse.value[3].z * one_over_determinant,
                        inverse.value[3].w * one_over_determinant,
                    ),
                )
            }
        }
    )*)
}

inverse_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
