use std::{ffi::{c_int, c_uint}, ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign}, slice::{self, SliceIndex}};

use crate::vec1;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct vec4<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[allow(non_camel_case_types)]
pub type bvec4 = vec4<bool>;
#[allow(non_camel_case_types)]
pub type dvec4 = vec4<f64>;
#[allow(non_camel_case_types)]
pub type ivec4 = vec4<c_int>;
#[allow(non_camel_case_types)]
pub type uvec4 = vec4<c_uint>;

impl<T> vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        &raw const *self as *const T
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T{
        &raw mut *self as *mut T
    }
    #[inline]
    pub const fn len(&self) -> usize {
        4
    }
    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<T> Deref for vec4<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for vec4<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for vec4<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for vec4<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

macro_rules! zero_impl {
    ($($t:ty)*) => ($(
        impl vec4<$t> {
            pub const fn zero() -> Self {
                Self { x: 0 as $t, y: 0 as $t, z: 0 as $t, w: 0 as $t }
            }
        }
    )*)
}

zero_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! unop_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for vec4<$t> {
            type Output = vec4<$t>;

            #[inline]
            fn $method(self) -> vec4<$t> {
                vec4 {
                    x: $imp::$method(self.x),
                    y: $imp::$method(self.y),
                    z: $imp::$method(self.z),
                    w: $imp::$method(self.w),
                }
            }
        }

        forward_ref_unop! { impl $imp, $method for vec4<$t> }
    }
}

macro_rules! binop_impl {
    (impl $imp:ident, $method: ident for $t:ty, $u:ty) => {
        impl $imp<$u> for vec4<$t> {
            type Output = vec4<$t>;

            #[inline]
            fn $method(self, other: $u) -> vec4<$t> {
                vec4 {
                    x: $imp::$method(self.x, other),
                    y: $imp::$method(self.y, other),
                    z: $imp::$method(self.z, other),
                    w: $imp::$method(self.w, other),
                }
            }
        }
        
        forward_ref_binop! { impl $imp, $method for vec4<$t>, $u }

        impl $imp<vec1<$u>> for vec4<$t> {
            type Output = vec4<$t>;

            #[inline]
            fn $method(self, other: vec1<$u>) -> vec4<$t> {
                vec4 {
                    x: $imp::$method(self.x, other.x),
                    y: $imp::$method(self.y, other.x),
                    z: $imp::$method(self.z, other.x),
                    w: $imp::$method(self.w, other.x),
                }
            }
        }
        
        forward_ref_binop! { impl $imp, $method for vec4<$t>, vec1<$u> }

        impl $imp<vec4<$u>> for $t {
            type Output = vec4<$t>;

            #[inline]
            fn $method(self, other: vec4<$u>) -> vec4<$t> {
                vec4 {
                    x: $imp::$method(self, other.x),
                    y: $imp::$method(self, other.y),
                    z: $imp::$method(self, other.z),
                    w: $imp::$method(self, other.w),
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for $t, vec4<$u> }

        impl $imp<vec4<$u>> for vec1<$t> {
            type Output = vec4<$t>;

            #[inline]
            fn $method(self, other: vec4<$u>) -> vec4<$t> {
                vec4 {
                    x: $imp::$method(self.x, other.x),
                    y: $imp::$method(self.x, other.y),
                    z: $imp::$method(self.x, other.z),
                    w: $imp::$method(self.x, other.w),
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for vec1<$t>, vec4<$u> }
    
        impl $imp<vec4<$u>> for vec4<$t> {
            type Output = vec4<$t>;

            #[inline]
            fn $method(self, other: vec4<$u>) -> vec4<$t> {
                vec4 {
                    x: $imp::$method(self.x, other.x),
                    y: $imp::$method(self.y, other.y),
                    z: $imp::$method(self.z, other.z),
                    w: $imp::$method(self.w, other.w),
                }
            }
        }

        forward_ref_binop! { impl $imp, $method for vec4<$t>, vec4<$u> }
    }
}

macro_rules! op_assign_impl {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<$u> for vec4<$t> {
            #[inline]
            fn $method(&mut self, other: $u) {
                $imp::$method(&mut self.x, other);
                $imp::$method(&mut self.y, other);
                $imp::$method(&mut self.z, other);
                $imp::$method(&mut self.w, other);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for vec4<$t>, $u }

        impl $imp<vec1<$u>> for vec4<$t> {
            #[inline]
            fn $method(&mut self, other: vec1<$u>) {
                $imp::$method(&mut self.x, other.x);
                $imp::$method(&mut self.y, other.x);
                $imp::$method(&mut self.z, other.x);
                $imp::$method(&mut self.w, other.x);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for vec4<$t>, vec1<$u> }

        impl $imp<vec4<$u>> for vec4<$t> {
            #[inline]
            fn $method(&mut self, other: vec4<$u>) {
                $imp::$method(&mut self.x, other.x);
                $imp::$method(&mut self.y, other.y);
                $imp::$method(&mut self.z, other.z);
                $imp::$method(&mut self.w, other.w);
            }
        }

        forward_ref_op_assign! { impl $imp, $method for vec4<$t>, vec4<$u> }
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
        op_assign_impl! { impl MulAssign, mul_assign for $t, $t }
    )+)
}

mul_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl DivAssign, div_assign for $t, $t }
    )+)
}

div_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! rem_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl RemAssign, rem_assign for $t, $t }
    )+)
}

rem_assign_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! bitand_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl BitAndAssign, bitand_assign for $t, $t }
    )+)
}

bitand_assign_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! bitor_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl BitOrAssign, bitor_assign for $t, $t }
    )+)
}

bitor_assign_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! bitxor_assign_impl {
    ($($t:ty)+) => ($(
        op_assign_impl! { impl BitXorAssign, bitxor_assign for $t, $t }
    )+)
}

bitxor_assign_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! shl_assign_impl {
    ($t:ty, $f:ty) => {
        op_assign_impl! { impl ShlAssign, shl_assign for $t, $f }
    };
}

macro_rules! shl_assign_impl_all {
    ($($t:ty)*) => ($(
        shl_assign_impl! { $t, u8 }
        shl_assign_impl! { $t, u16 }
        shl_assign_impl! { $t, u32 }
        shl_assign_impl! { $t, u64 }
        shl_assign_impl! { $t, u128 }
        shl_assign_impl! { $t, usize }

        shl_assign_impl! { $t, i8 }
        shl_assign_impl! { $t, i16 }
        shl_assign_impl! { $t, i32 }
        shl_assign_impl! { $t, i64 }
        shl_assign_impl! { $t, i128 }
        shl_assign_impl! { $t, isize }
    )*)
}

shl_assign_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

macro_rules! shr_assign_impl {
    ($t:ty, $f:ty) => {
        op_assign_impl! { impl ShrAssign, shr_assign for $t, $f }
    };
}

macro_rules! shr_assign_impl_all {
    ($($t:ty)*) => ($(
        shr_assign_impl! { $t, u8 }
        shr_assign_impl! { $t, u16 }
        shr_assign_impl! { $t, u32 }
        shr_assign_impl! { $t, u64 }
        shr_assign_impl! { $t, u128 }
        shr_assign_impl! { $t, usize }

        shr_assign_impl! { $t, i8 }
        shr_assign_impl! { $t, i16 }
        shr_assign_impl! { $t, i32 }
        shr_assign_impl! { $t, i64 }
        shr_assign_impl! { $t, i128 }
        shr_assign_impl! { $t, isize }
    )*)
}

shr_assign_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

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
        binop_impl! { impl Mul, mul for $t, $t }
    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! div_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl Div, div for $t, $t }
    )*)
}

div_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! rem_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl Rem, rem for $t, $t }
    )*)
}

rem_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

macro_rules! bitand_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl BitAnd, bitand for $t, $t }
    )*)
}

bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! bitor_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl BitOr, bitor for $t, $t }
    )*)
}

bitor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! bitxor_impl {
    ($($t:ty)*) => ($(
        binop_impl! { impl BitXor, bitxor for $t, $t }
    )*)
}

bitxor_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! shl_impl {
    ($t:ty, $f:ty) => {
        binop_impl! { impl Shl, shl for $t, $f }
    };
}

macro_rules! shl_impl_all {
    ($($t:ty)*) => ($(
        shl_impl! { $t, u8 }
        shl_impl! { $t, u16 }        
        shl_impl! { $t, u32 }
        shl_impl! { $t, u64 }
        shl_impl! { $t, u128 }
        shl_impl! { $t, usize }

        shl_impl! { $t, i8 }
        shl_impl! { $t, i16 }
        shl_impl! { $t, i32 }
        shl_impl! { $t, i64 }
        shl_impl! { $t, i128 }
        shl_impl! { $t, isize }
    )*)
}

shl_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

macro_rules! shr_impl {
    ($t:ty, $f:ty) => {
        binop_impl! { impl Shr, shr for $t, $f }
    };
}

macro_rules! shr_impl_all {
    ($($t:ty)*) => ($(
        shr_impl! { $t, u8 }
        shr_impl! { $t, u16 }        
        shr_impl! { $t, u32 }
        shr_impl! { $t, u64 }
        shr_impl! { $t, u128 }
        shr_impl! { $t, usize }

        shr_impl! { $t, i8 }
        shr_impl! { $t, i16 }
        shr_impl! { $t, i32 }
        shr_impl! { $t, i64 }
        shr_impl! { $t, i128 }
        shr_impl! { $t, isize }
    )*)
}

shr_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

macro_rules! not_impl {
    ($($t:ty)*) => ($(
        unop_impl! { impl Not, not for $t }
    )*)
}

not_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
