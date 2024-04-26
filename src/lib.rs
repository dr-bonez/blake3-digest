use std::io::Write;
use std::ops::{Deref, DerefMut};

use blake3::Hasher;
use digest::generic_array::ArrayLength;
use digest::{FixedOutput, HashMarker, OutputSizeUser, Update};

#[derive(Debug, Clone, Default)]
pub struct Blake3<T: ArrayLength<u8>> {
    hasher: Hasher,
    _length: T,
}
impl<T: ArrayLength<u8>> From<Hasher> for Blake3<T> {
    fn from(value: Hasher) -> Self {
        Self {
            hasher: value,
            _length: T::default(),
        }
    }
}
impl<T: ArrayLength<u8>> From<Blake3<T>> for Hasher {
    fn from(value: Blake3<T>) -> Self {
        value.hasher
    }
}
impl<T: ArrayLength<u8>> Deref for Blake3<T> {
    type Target = Hasher;
    fn deref(&self) -> &Self::Target {
        &self.hasher
    }
}
impl<T: ArrayLength<u8>> DerefMut for Blake3<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hasher
    }
}
impl<T: ArrayLength<u8>> OutputSizeUser for Blake3<T> {
    type OutputSize = T;
}
impl<T: ArrayLength<u8>> Update for Blake3<T> {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }
}
impl<T: ArrayLength<u8>> FixedOutput for Blake3<T> {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        self.hasher.finalize_xof().fill(out);
    }
}
impl<T: ArrayLength<u8>> Write for Blake3<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl<T: ArrayLength<u8>> HashMarker for Blake3<T> {}
