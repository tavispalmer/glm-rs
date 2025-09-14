use std::{ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}, slice::SliceIndex};

use crate::vec2;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct mat2<T = f32> {
    pub(crate) value: [vec2<T>; 2],
}

#[allow(non_camel_case_types)]
pub type dmat2x2 = mat2<f64>;
#[allow(non_camel_case_types)]
pub type dmat2 = mat2<f64>;
#[allow(non_camel_case_types)]
pub type mat2x2 = mat2;

impl<T> mat2<T> {
    #[inline]
    pub const fn new(value: [vec2<T>; 2]) -> Self {
        Self { value }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const vec2<T> {
        self.value.as_ptr()
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut vec2<T> {
        self.value.as_mut_ptr()
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.value.len()
    }
    #[inline]
    pub const fn as_slice(&self) -> &[vec2<T>] {
        &self.value
    }
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [vec2<T>] {
        &mut self.value
    }
}

impl<T> Deref for mat2<T> {
    type Target = [vec2<T>];

    #[inline]
    fn deref(&self) -> &[vec2<T>] {
        self.as_slice()
    }
}

impl<T> DerefMut for mat2<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [vec2<T>] {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[vec2<T>]>> Index<I> for mat2<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[vec2<T>]>> IndexMut<I> for mat2<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

macro_rules! identity_impl {
    ($($t:ty)*) => ($(
        impl mat2<$t> {
            #[inline]
            pub const fn identity() -> Self {
                Self { value: [vec2::new(1 as $t, 0 as $t), vec2::new(0 as $t, 1 as $t)] }
            }
        }

        impl Default for mat2<$t> {
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
        impl $imp for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn $method(self) -> mat2<$t> {
                mat2 {
                    value: [
                        $imp::$method(self[0]),
                        $imp::$method(self[1]),
                    ],
                }
            }
        }

        forward_ref_unop! { impl $imp, $method for mat2<$t> }
    }
}

macro_rules! binop_impl {
    (impl $imp:ident, $method: ident for $t:ty, $u:ty) => {
        impl $imp<$u> for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn $method(self, other: $u) -> mat2<$t> {
                mat2 {
                    value: [
                        $imp::$method(self[0], other),
                        $imp::$method(self[1], other),
                    ],
                }
            }
        }
        
        forward_ref_binop! { impl $imp, $method for mat2<$t>, $u }

        impl $imp<mat2<$u>> for $t {
            type Output = mat2<$t>;

            #[inline]
            fn $method(self, other: mat2<$u>) -> mat2<$t> {
                mat2 {
                    value: [
                        $imp::$method(self, other[0]),
                        $imp::$method(self, other[1]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for $t, mat2<$u> }
    
        impl $imp<mat2<$u>> for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn $method(self, other: mat2<$u>) -> mat2<$t> {
                mat2 {
                    value: [
                        $imp::$method(self[0], other[0]),
                        $imp::$method(self[1], other[1]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for mat2<$t>, mat2<$u> }
    }
}

macro_rules! op_assign_impl {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<$u> for mat2<$t> {
            #[inline]
            fn $method(&mut self, other: $u) {
                $imp::$method(&mut self[0], other);
                $imp::$method(&mut self[1], other);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for mat2<$t>, $u }

        impl $imp<mat2<$u>> for mat2<$t> {
            #[inline]
            fn $method(&mut self, other: mat2<$u>) {
                $imp::$method(&mut self[0], other[0]);
                $imp::$method(&mut self[1], other[1]);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for mat2<$t>, mat2<$u> }
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
        impl MulAssign<$t> for mat2<$t> {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                MulAssign::mul_assign(&mut self[0], other);
                MulAssign::mul_assign(&mut self[1], other);
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for mat2<$t>, $t }

        impl MulAssign<mat2<$t>> for mat2<$t> {
            #[inline]
            fn mul_assign(&mut self, other: mat2<$t>) {
                *self = Mul::mul(*self, other)
            }
        }

        forward_ref_op_assign! { impl MulAssign, mul_assign for mat2<$t>, mat2<$t> }
    )+)
}

mul_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_assign_scalar_impl {
    ($($t:ty)+) => ($(
        impl DivAssign<$t> for mat2<$t> {
            #[inline]
            fn div_assign(&mut self, other: $t) {
                DivAssign::div_assign(&mut self[0], other);
                DivAssign::div_assign(&mut self[1], other);
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for mat2<$t>, $t }
    )+)
}

div_assign_scalar_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_assign_impl {
    ($($t:ty)+) => ($(
        impl DivAssign<mat2<$t>> for mat2<$t> {
            #[inline]
            fn div_assign(&mut self, other: mat2<$t>) {
                MulAssign::mul_assign(self, other.inverse())
            }
        }

        forward_ref_op_assign! { impl DivAssign, div_assign for mat2<$t>, mat2<$t> }
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
        impl Mul<$t> for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn mul(self, other: $t) -> mat2<$t> {
                mat2 {
                    value: [
                        Mul::mul(self[0], other),
                        Mul::mul(self[1], other),
                    ],
                }
            }
        }
        
        forward_ref_binop! { impl Mul, mul for mat2<$t>, $t }

        impl Mul<mat2<$t>> for $t {
            type Output = mat2<$t>;

            #[inline]
            fn mul(self, other: mat2<$t>) -> mat2<$t> {
                mat2 {
                    value: [
                        Mul::mul(self, other[0]),
                        Mul::mul(self, other[1]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl Mul, mul for $t, mat2<$t> }

        impl Mul<vec2<$t>> for mat2<$t> {
            type Output = vec2<$t>;

            #[inline]
            fn mul(self, other: vec2<$t>) -> vec2<$t> {
                vec2 {
                    x: self[0][0] * other.x + self[1][0] * other.y,
                    y: self[0][1] * other.x + self[1][1] * other.y,
                }
            }
        }

        forward_ref_binop! { impl Mul, mul for vec2<$t>, mat2<$t> }

        impl Mul<mat2<$t>> for vec2<$t> {
            type Output = vec2<$t>;

            #[inline]
            fn mul(self, other: mat2<$t>) -> vec2<$t> {
                vec2 {
                    x: self.x * other[0][0] + self.y * other[0][1],
                    y: self.x * other[1][0] + self.y * other[1][1],
                }
            }
        }

        forward_ref_binop! { impl Mul, mul for mat2<$t>, vec2<$t> }
    
        impl Mul<mat2<$t>> for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn mul(self, other: mat2<$t>) -> mat2<$t> {
                mat2 {
                    value: [
                        vec2 {
                            x: self[0][0] * other[0][0] + self[1][0] * other[0][1],
                            y: self[0][1] * other[0][0] + self[1][1] * other[0][1],
                        },
                        vec2 {
                            x: self[0][0] * other[1][0] + self[1][0] * other[1][1],
                            y: self[0][1] * other[1][0] + self[1][1] * other[1][1],
                        },
                    ],
                }
            }
        }

        forward_ref_binop! { impl Mul, mul for mat2<$t>, mat2<$t> }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_scalar_impl {
    ($($t:ty)*) => ($(
        impl Div<$t> for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn div(self, other: $t) -> mat2<$t> {
                mat2 {
                    value: [
                        Div::div(self[0], other),
                        Div::div(self[1], other),
                    ],
                }
            }
        }
        
        forward_ref_binop! { impl Div, div for mat2<$t>, $t }

        impl Div<mat2<$t>> for $t {
            type Output = mat2<$t>;

            #[inline]
            fn div(self, other: mat2<$t>) -> mat2<$t> {
                mat2 {
                    value: [
                        Div::div(self, other[0]),
                        Div::div(self, other[1]),
                    ],
                }
            }
        }

        forward_ref_binop! { impl Div, div for $t, mat2<$t> }
    )*)
}

div_scalar_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl Div<vec2<$t>> for mat2<$t> {
            type Output = vec2<$t>;

            #[inline]
            fn div(self, other: vec2<$t>) -> vec2<$t> {
                Mul::mul(self.inverse(), other)
            }
        }

        forward_ref_binop! { impl Div, div for vec2<$t>, mat2<$t> }

        impl Div<mat2<$t>> for vec2<$t> {
            type Output = vec2<$t>;

            #[inline]
            fn div(self, other: mat2<$t>) -> vec2<$t> {
                Mul::mul(self, other.inverse())
            }
        }

        forward_ref_binop! { impl Div, div for mat2<$t>, vec2<$t> }
    
        impl Div<mat2<$t>> for mat2<$t> {
            type Output = mat2<$t>;

            #[inline]
            fn div(self, other: mat2<$t>) -> mat2<$t> {
                let mut self_copy = self;
                DivAssign::div_assign(&mut self_copy, other);
                self_copy
            }
        }

        forward_ref_binop! { impl Div, div for mat2<$t>, mat2<$t> }
    )*)
}

div_impl! { isize i8 i16 i32 i64 i128 f32 f64 }
