use crate::prelude::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Vec2<S: G> {
    dims: u16,
    v: Vec<S::Scalar>,
}

impl<S: G> Vec2<S> {
    pub fn new(dims: u16, n: usize) -> Self {
        Self {
            dims,
            v: bytemuck::zeroed_vec(dims as usize * n),
        }
    }
    pub fn dims(&self) -> u16 {
        self.dims
    }
    pub fn len(&self) -> usize {
        self.v.len() / self.dims as usize
    }
    pub fn copy_within(&mut self, i: usize, j: usize) {
        assert!(i < self.len() && j < self.len());
        unsafe {
            if i != j {
                let src = self.v.as_ptr().add(self.dims as usize * i);
                let dst = self.v.as_mut_ptr().add(self.dims as usize * j);
                std::ptr::copy_nonoverlapping(src, dst, self.dims as usize);
            }
        }
    }
}

impl<S: G> Index<usize> for Vec2<S> {
    type Output = [S::Scalar];

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[self.dims as usize * index..][..self.dims as usize]
    }
}

impl<S: G> IndexMut<usize> for Vec2<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[self.dims as usize * index..][..self.dims as usize]
    }
}

impl<S: G> Deref for Vec2<S> {
    type Target = [S::Scalar];

    fn deref(&self) -> &Self::Target {
        self.v.deref()
    }
}

impl<S: G> DerefMut for Vec2<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.v.deref_mut()
    }
}
