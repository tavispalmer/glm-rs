use crate::{vec2, vec3, vec4};

macro_rules! dot_impl {
    ($($t:ty)*) => ($(
        impl vec2<$t> {
            #[inline]
            pub const fn dot(self, other: Self) -> $t {
                let tmp = Self::new(self.x * other.x, self.y * other.y);
                tmp.x + tmp.y
            }
        }

        impl vec3<$t> {
            #[inline]
            pub const fn dot(self, other: Self) -> $t {
                let tmp = Self::new(self.x * other.x, self.y * other.y, self.z * other.z);
                tmp.x + tmp.y + tmp.z
            }
        }

        impl vec4<$t> {
            #[inline]
            pub const fn dot(self, other: Self) -> $t {
                let tmp = Self::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w);
                (tmp.x + tmp.y) + (tmp.z + tmp.w)
            }
        }
    )*)
}

dot_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }
