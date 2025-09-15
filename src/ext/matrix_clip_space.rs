use crate::mat4;

macro_rules! ortho_impl {
    ($($t:ty)*) => ($(
        impl mat4<$t> {
            #[inline]
            pub const fn ortho(left: $t, right: $t, bottom: $t, top: $t) -> Self {
                let mut result = Self::identity();
                result.value[0].x = 2 as $t / (right - left);
                result.value[1].y = 2 as $t / (top - bottom);
                result.value[2].z = - 1 as $t;
                result.value[3].x = - (right + left) / (right - left);
                result.value[3].y = - (top + bottom) / (top - bottom);
                result
            }
        }
    )*)
}

ortho_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
