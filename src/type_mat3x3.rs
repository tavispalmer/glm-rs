use std::{ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}, slice::SliceIndex};

use crate::vec3;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct mat3<T = f32> {
    pub(crate) value: [vec3<T>; 3],
}

#[allow(non_camel_case_types)]
pub type dmat3x3 = mat3<f64>;
#[allow(non_camel_case_types)]
pub type dmat3 = mat3<f64>;
#[allow(non_camel_case_types)]
pub type mat3x3 = mat3;

impl<T> mat3<T> {
    #[inline]
    pub const fn new(value: [vec3<T>; 3]) -> Self {
        Self { value }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const vec3<T> {
        self.value.as_ptr()
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut vec3<T> {
        self.value.as_mut_ptr()
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.value.len()
    }
    #[inline]
    pub const fn as_slice(&self) -> &[vec3<T>] {
        &self.value
    }
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [vec3<T>] {
        &mut self.value
    }
}

impl<T> Deref for mat3<T> {
    type Target = [vec3<T>];

    #[inline]
    fn deref(&self) -> &[vec3<T>] {
        self.as_slice()
    }
}

impl<T> DerefMut for mat3<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [vec3<T>] {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[vec3<T>]>> Index<I> for mat3<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[vec3<T>]>> IndexMut<I> for mat3<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

macro_rules! identity_impl {
    ($($t:ty)*) => ($(
        impl mat3<$t> {
            #[inline]
            pub const fn identity() -> Self {
                Self { value: [vec3::new(1 as $t, 0 as $t, 0 as $t), vec3::new(0 as $t, 1 as $t, 0 as $t), vec3::new(0 as $t, 0 as $t, 1 as $t)] }
            }
        }

        impl Default for mat3<$t> {
            #[inline]
            fn default() -> Self {
                Self::identity()
            }
        }
    )*)
}

identity_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! unop_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn $method(self) -> mat3<$t> {
                mat3 {
                    value: [
                        $imp::$method(self[0]),
                        $imp::$method(self[1]),
                        $imp::$method(self[2]),
                    ],
                }
            }
        }

        forward_ref_unop! { impl $imp, $method for mat3<$t> }
    }
}

macro_rules! binop_impl {
    (impl $imp:ident, $method: ident for $t:ty, $u:ty) => {
        impl $imp<$u> for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn $method(self, other: $u) -> mat3<$t> {
                mat3 {
                    value: [
                        $imp::$method(self[0], other),
                        $imp::$method(self[1], other),
                        $imp::$method(self[2], other),
                    ],
                }
            }
        }
        
        forward_ref_binop! { impl $imp, $method for mat3<$t>, $u }

        impl $imp<mat3<$u>> for $t {
            type Output = mat3<$t>;

            #[inline]
            fn $method(self, other: mat3<$u>) -> mat3<$t> {
                mat3 {
                    value: [
                        $imp::$method(self, other[0]),
                        $imp::$method(self, other[1]),
                        $imp::$method(self, other[2]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for $t, mat3<$u> }
    
        impl $imp<mat3<$u>> for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn $method(self, other: mat3<$u>) -> mat3<$t> {
                mat3 {
                    value: [
                        $imp::$method(self[0], other[0]),
                        $imp::$method(self[1], other[1]),
                        $imp::$method(self[2], other[2]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for mat3<$t>, mat3<$u> }
    }
}

macro_rules! op_assign_impl {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<$u> for mat3<$t> {
            #[inline]
            fn $method(&mut self, other: $u) {
                $imp::$method(&mut self[0], other);
                $imp::$method(&mut self[1], other);
                $imp::$method(&mut self[2], other);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for mat3<$t>, $u }

        impl $imp<mat3<$u>> for mat3<$t> {
            #[inline]
            fn $method(&mut self, other: mat3<$u>) {
                $imp::$method(&mut self[0], other[0]);
                $imp::$method(&mut self[1], other[1]);
                $imp::$method(&mut self[2], other[2]);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for mat3<$t>, mat3<$u> }
    }
}

macro_rules! add_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl AddAssign, add_assign for $t, $t }
    )+)
}

add_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! sub_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl SubAssign, sub_assign for $t, $t }
    )+)
}

sub_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! mul_assign_impl {
    ($($t:ty)+) => ($(
        impl MulAssign<$t> for mat3<$t> {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                MulAssign::mul_assign(&mut self[0], other);
                MulAssign::mul_assign(&mut self[1], other);
                MulAssign::mul_assign(&mut self[2], other);
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for mat3<$t>, $t }

        impl MulAssign<mat3<$t>> for mat3<$t> {
            #[inline]
            fn mul_assign(&mut self, other: mat3<$t>) {
                *self = Mul::mul(*self, other)
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for mat3<$t>, mat3<$t> }
    )+)
}

mul_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_assign_scalar_impl {
    ($($t:ty)+) => ($(
        impl DivAssign<$t> for mat3<$t> {
            #[inline]
            fn div_assign(&mut self, other: $t) {
                DivAssign::div_assign(&mut self[0], other);
                DivAssign::div_assign(&mut self[1], other);
                DivAssign::div_assign(&mut self[2], other);
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for mat3<$t>, $t }
    )+)
}

div_assign_scalar_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_assign_impl {
    ($($t:ty)+) => ($(
        impl DivAssign<mat3<$t>> for mat3<$t> {
            #[inline]
            fn div_assign(&mut self, other: mat3<$t>) {
                MulAssign::mul_assign(self, other.inverse())
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for mat3<$t>, mat3<$t> }
    )+)
}

div_assign_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! neg_impl {
    ($($t:ty)*) => ($(
        unop_impl! { impl Neg, neg for $t }
    )*)
}

neg_impl! { isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl Add, add for $t, $t }
    )*)
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! sub_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl Sub, sub for $t, $t }
    )*)
}

sub_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul<$t> for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn mul(self, other: $t) -> mat3<$t> {
                mat3 {
                    value: [
                        Mul::mul(self[0], other),
                        Mul::mul(self[1], other),
                        Mul::mul(self[2], other),
                    ],
                }
            }
        }
        
        forward_ref_binop! { impl Mul, mul for mat3<$t>, $t }

        impl Mul<mat3<$t>> for $t {
            type Output = mat3<$t>;

            #[inline]
            fn mul(self, other: mat3<$t>) -> mat3<$t> {
                mat3 {
                    value: [
                        Mul::mul(self, other[0]),
                        Mul::mul(self, other[1]),
                        Mul::mul(self, other[2]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl Mul, mul for $t, mat3<$t> }

        impl Mul<vec3<$t>> for mat3<$t> {
            type Output = vec3<$t>;

            #[inline]
            fn mul(self, other: vec3<$t>) -> vec3<$t> {
                self[0] * other.splat_x() + self[1] * other.splat_y() + self[2] * other.splat_z()
            }
        }

        forward_ref_binop! { impl Mul, mul for vec3<$t>, mat3<$t> }

        impl Mul<mat3<$t>> for vec3<$t> {
            type Output = vec3<$t>;

            #[inline]
            fn mul(self, other: mat3<$t>) -> vec3<$t> {
                vec3 {
                    x: self.dot(other[0]),
                    y: self.dot(other[1]),
                    z: self.dot(other[2]),
                }
            }
        }

        forward_ref_binop! { impl Mul, mul for mat3<$t>, vec3<$t> }
    
        impl Mul<mat3<$t>> for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn mul(self, other: mat3<$t>) -> mat3<$t> {
                let src_a0 = &self[0];
                let src_a1 = &self[1];
                let src_a2 = &self[2];

                let src_b0 = &other[0];
                let src_b1 = &other[1];
                let src_b2 = &other[2];

                let mut tmp0 = src_a0 * src_b0.x;
                tmp0 += src_a1 * src_b0.y;
                tmp0 += src_a2 * src_b0.z;
                let mut tmp1 = src_a0 * src_b1.x;
                tmp1 += src_a1 * src_b1.y;
                tmp1 += src_a2 * src_b1.z;
                let mut tmp2 = src_a0 * src_b2.x;
                tmp2 += src_a1 * src_b2.y;
                tmp2 += src_a2 * src_b2.z;

                mat3::new([tmp0, tmp1, tmp2])
            }
        }

        forward_ref_binop! { impl Mul, mul for mat3<$t>, mat3<$t> }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_scalar_impl {
    ($($t:ty)*) => ($(
        impl Div<$t> for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn div(self, other: $t) -> mat3<$t> {
                mat3 {
                    value: [
                        Div::div(self[0], other),
                        Div::div(self[1], other),
                        Div::div(self[2], other),
                    ],
                }
            }
        }
        
        forward_ref_binop! { impl Div, div for mat3<$t>, $t }

        impl Div<mat3<$t>> for $t {
            type Output = mat3<$t>;

            #[inline]
            fn div(self, other: mat3<$t>) -> mat3<$t> {
                mat3 {
                    value: [
                        Div::div(self, other[0]),
                        Div::div(self, other[1]),
                        Div::div(self, other[2]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl Div, div for $t, mat3<$t> }
    )*)
}

div_scalar_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<vec3<$t>> for mat3<$t> {
            type Output = vec3<$t>;

            #[inline]
            fn div(self, other: vec3<$t>) -> vec3<$t> {
                Mul::mul(self.inverse(), other)
            }
        }

        forward_ref_binop! { impl Div, div for vec3<$t>, mat3<$t> }

        impl Div<mat3<$t>> for vec3<$t> {
            type Output = vec3<$t>;

            #[inline]
            fn div(self, other: mat3<$t>) -> vec3<$t> {
                Mul::mul(self, other.inverse())
            }
        }

        forward_ref_binop! { impl Div, div for mat3<$t>, vec3<$t> }
    
        impl Div<mat3<$t>> for mat3<$t> {
            type Output = mat3<$t>;

            #[inline]
            fn div(self, other: mat3<$t>) -> mat3<$t> {
                let mut self_copy = self;
                DivAssign::div_assign(&mut self_copy, other);
                self_copy
            }
        }

        forward_ref_binop! { impl Div, div for mat3<$t>, mat3<$t> }
    )*)
}

div_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
