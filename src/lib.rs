use std::cmp::{max, min};
use std::fmt::{self, Debug};
use std::mem::swap;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum SegmentTreeError {
    EmptyData,
    InvalidRange,
    IndexOutBounds,
}

impl fmt::Display for SegmentTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self {
            SegmentTreeError::EmptyData => write!(f, "EmptyData"),
            SegmentTreeError::InvalidRange => write!(f, "InvalidRange"),
            SegmentTreeError::IndexOutBounds => write!(f, "IndexOutBounds"),
        }
    }
}

pub struct RSQ<T> where T: Copy + Ord + Add<Output = T> + Default {
    values: Vec<T>,
    sz: usize,
    bound: usize,
}

impl<T> RSQ<T> where T: Copy + Ord + Add<Output = T> + Default + Debug {
    pub fn build(values: &[T]) -> Result<Self, SegmentTreeError> {
        if values.is_empty() {
            return Err(SegmentTreeError::EmptyData);
        }
        let mut n = 1usize;
        while n < values.len() {
            n <<= 1;
        }
        let mut dst: Vec<T> = vec![T::default(); n << 1];
        build_inner_rsq(&mut dst, values, 0, 0, n - 1);
        Ok(RSQ {
            values: dst,
            sz: n,
            bound: values.len(),
        })
    }
    pub fn sum(&self, mut l: usize, mut r: usize) -> Result<T, SegmentTreeError> {
        if l < self.bound && r < self.bound {
            if l > r {
                swap(&mut l, &mut r);
            }
            return Ok(self.sum_inner(0, 0, self.sz - 1, l, r));
        }
        return Err(SegmentTreeError::InvalidRange);
    }

    fn sum_inner(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> T {
        if l > r {
            return T::default();
        }
        if tl == l && tr == r {
            return self.values[v];
        }
        let m = (tl + tr) >> 1;
        return self.sum_inner((v << 1) + 1, tl, m, l, min(r, m))
            + self.sum_inner((v << 1) + 2, m + 1, tr, max(l, m + 1), r);
    }
    pub fn upddate(&mut self, pos: usize, value: T) -> Result<(), SegmentTreeError> {
        if pos < self.bound {
            self.upddate_inner(0, 0, self.sz - 1, pos, value);
            return Ok(());
        }
        return Err(SegmentTreeError::IndexOutBounds);
    }
    fn upddate_inner(&mut self, v: usize, tl: usize, tr: usize, pos: usize, value: T) {
        if tl == tr {
            self.values[v] = value;
        } else {
            let m = (tl + tr) >> 1;
            if pos <= m {
                self.upddate_inner((v << 1) + 1, tl, m, pos, value);
            } else {
                self.upddate_inner((v << 1) + 2, m + 1, tr, pos, value);
            }
            self.values[v] = self.values[(v << 1) + 1] + self.values[(v << 1) + 2];
        }
    }
}

fn build_inner_rsq<T>(dst: &mut [T], src: &[T], v: usize, tl: usize, tr: usize) where T: Copy + Ord + Add<Output = T> + Default {
    if tl == tr {
        if tl < src.len() {
            dst[v] = src[tl];
        }
    } else {
        let m = (tl + tr) >> 1;
        build_inner_rsq(dst, src, (v << 1) + 1, tl, m);
        build_inner_rsq(dst, src, (v << 1) + 2, m + 1, tr);
        dst[v] = dst[(v << 1) + 1] + dst[(v << 1) + 2];
    }
}

pub struct RMaxQ<T> where T: Copy + Ord + MaxValue + MinValue {
    values: Vec<T>,
    sz: usize,
    bound: usize,
}

impl<T> RMaxQ<T> where T: Copy + Ord + MaxValue + MinValue + Debug {
    pub fn build(values: &[T]) -> Result<Self, SegmentTreeError> {
        if values.is_empty() {
            return Err(SegmentTreeError::EmptyData);
        }
        let mut n = 1usize;
        while n < values.len() {
            n <<= 1;
        }
        let mut dst: Vec<T> = vec![T::min_value(); n << 1];
        build_inner_rmaxq(&mut dst, values, 0, 0, n - 1);
        Ok(RMaxQ {
            values: dst,
            sz: n,
            bound: values.len(),
        })
    }
    pub fn max(&self, mut l: usize, mut r: usize) -> Result<T, SegmentTreeError> {
        if l < self.bound && r < self.bound {
            if l > r {
                swap(&mut l, &mut r);
            }
            return Ok(self.max_inner(0, 0, self.sz - 1, l, r));
        }
        return Err(SegmentTreeError::InvalidRange);
    }

    fn max_inner(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> T {
        if l > r {
            return T::min_value();
        }
        if tl == l && tr == r {
            return self.values[v];
        }
        let m = (tl + tr) >> 1;
        return max(
            self.max_inner((v << 1) + 1, tl, m, l, min(r, m)),
            self.max_inner((v << 1) + 2, m + 1, tr, max(l, m + 1), r),
        );
    }
    pub fn upddate(&mut self, pos: usize, value: T) -> Result<(), SegmentTreeError> {
        if pos < self.bound {
            self.upddate_inner(0, 0, self.sz - 1, pos, value);
            return Ok(());
        }
        return Err(SegmentTreeError::IndexOutBounds);
    }
    fn upddate_inner(&mut self, v: usize, tl: usize, tr: usize, pos: usize, value: T) {
        if tl == tr {
            self.values[v] = value;
        } else {
            let m = (tl + tr) >> 1;
            if pos <= m {
                self.upddate_inner((v << 1) + 1, tl, m, pos, value);
            } else {
                self.upddate_inner((v << 1) + 2, m + 1, tr, pos, value);
            }
            self.values[v] = max(self.values[(v << 1) + 1], self.values[(v << 1) + 2]);
        }
    }
}

fn build_inner_rmaxq<T>(dst: &mut [T], src: &[T], v: usize, tl: usize, tr: usize) where T: Copy + Ord + MaxValue + MinValue {
    if tl == tr {
        if tl < src.len() {
            dst[v] = src[tl];
        }
    } else {
        let m = (tl + tr) >> 1;
        build_inner_rmaxq(dst, src, (v << 1) + 1, tl, m);
        build_inner_rmaxq(dst, src, (v << 1) + 2, m + 1, tr);
        dst[v] = max(dst[(v << 1) + 1], dst[(v << 1) + 2]);
    }
}

pub struct RMinQ<T> where T: Copy + Ord + MaxValue + MinValue {
    values: Vec<T>,
    sz: usize,
    bound: usize,
}

impl<T> RMinQ<T> where T: Copy + Ord + MaxValue + MinValue + Debug {
    pub fn build(values: &[T]) -> Result<Self, SegmentTreeError> {
        if values.is_empty() {
            return Err(SegmentTreeError::EmptyData);
        }
        let mut n = 1usize;
        while n < values.len() {
            n <<= 1;
        }
        let mut dst: Vec<T> = vec![T::max_value(); n << 1];
        build_inner_rminq(&mut dst, values, 0, 0, n - 1);
        Ok(RMinQ {
            values: dst,
            sz: n,
            bound: values.len(),
        })
    }
    pub fn min(&self, mut l: usize, mut r: usize) -> Result<T, SegmentTreeError> {
        if l < self.bound && r < self.bound {
            if l > r {
                swap(&mut l, &mut r);
            }
            return Ok(self.min_inner(0, 0, self.sz - 1, l, r));
        }
        return Err(SegmentTreeError::InvalidRange);
    }

    fn min_inner(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> T {
        if l > r {
            return T::max_value();
        }
        if tl == l && tr == r {
            return self.values[v];
        }
        let m = (tl + tr) >> 1;
        return min(
            self.min_inner((v << 1) + 1, tl, m, l, min(r, m)),
            self.min_inner((v << 1) + 2, m + 1, tr, max(l, m + 1), r),
        );
    }
    pub fn upddate(&mut self, pos: usize, value: T) -> Result<(), SegmentTreeError> {
        if pos < self.bound {
            self.upddate_inner(0, 0, self.sz - 1, pos, value);
            return Ok(());
        }
        return Err(SegmentTreeError::IndexOutBounds);
    }
    fn upddate_inner(&mut self, v: usize, tl: usize, tr: usize, pos: usize, value: T) {
        if tl == tr {
            self.values[v] = value;
        } else {
            let m = (tl + tr) >> 1;
            if pos <= m {
                self.upddate_inner((v << 1) + 1, tl, m, pos, value);
            } else {
                self.upddate_inner((v << 1) + 2, m + 1, tr, pos, value);
            }
            self.values[v] = min(self.values[(v << 1) + 1], self.values[(v << 1) + 2]);
        }
    }
}

fn build_inner_rminq<T>(dst: &mut [T], src: &[T], v: usize, tl: usize, tr: usize) where T: Copy + Ord + MaxValue + MinValue {
    if tl == tr {
        if tl < src.len() {
            dst[v] = src[tl];
        }
    } else {
        let m = (tl + tr) >> 1;
        build_inner_rminq(dst, src, (v << 1) + 1, tl, m);
        build_inner_rminq(dst, src, (v << 1) + 2, m + 1, tr);
        dst[v] = min(dst[(v << 1) + 1], dst[(v << 1) + 2]);
    }
}

pub trait MaxValue {
    fn max_value() -> Self;
}
pub trait MinValue {
    fn min_value() -> Self;
}

macro_rules! impl_max_value {
    ( $( $t:ident )* ) => {
        $(
            impl MaxValue for $t {
                fn max_value() -> $t {
                    $t::MAX
                }
            }
        )*
    };
}

impl_max_value!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

macro_rules! impl_min_value {
    ( $( $t:ident )* ) => {
        $(
            impl MinValue for $t {
                fn min_value() -> $t {
                    $t::MIN
                }
            }
        )*
    };
}

impl_min_value!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
