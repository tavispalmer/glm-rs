use crate::{vec1, vec2, vec3, vec4};

impl<T> vec1<T> {
    #[inline]
    pub(crate) fn splat_x(self) -> Self {
        Self {
            x: self.x,
        }
    }
}

impl<T: Copy> vec2<T> {
    #[inline]
    pub(crate) const fn splat_x(self) -> Self {
        Self {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    pub(crate) const fn splat_y(self) -> Self {
        Self {
            x: self.y,
            y: self.y,
        }
    }
}

impl<T: Copy> vec3<T> {
    #[inline]
    pub(crate) const fn splat_x(self) -> Self {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    pub(crate) const fn splat_y(self) -> Self {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    pub(crate) const fn splat_z(self) -> Self {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }
}

impl<T: Copy> vec4<T> {
    #[inline]
    pub(crate) const fn splat_x(self) -> Self {
        Self {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    #[inline]
    pub(crate) const fn splat_y(self) -> Self {
        Self {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    #[inline]
    pub(crate) const fn splat_z(self) -> Self {
        Self {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    #[inline]
    pub(crate) const fn splat_w(self) -> Self {
        Self {
            x: self.w,
            y: self.w,
            z: self.w,
            w: self.w,
        }
    }
}
