use std::marker::PhantomData;
use std::alloc::*;
use std::mem;

#[derive(Debug, PartialEq, Eq)]
pub struct PlaneData {
  ptr: std::ptr::NonNull<u8>,
  _marker: PhantomData<u8>,
  len: usize,
  align: usize,
}

unsafe impl Send for PlaneData {}
unsafe impl Sync for PlaneData {}

impl Clone for PlaneData {
  fn clone(&self) -> Self {
    let mut pd = unsafe { Self::new_uninitialized(self.len, self.align) };

    pd.copy_from_slice(self);

    pd
  }
}

impl std::ops::Deref for PlaneData {
  type Target = [u8];

  fn deref(&self) -> &[u8] {
    unsafe {
      let p = self.ptr.as_ptr();

      std::slice::from_raw_parts(p, self.len)
    }
  }
}

impl std::ops::DerefMut for PlaneData {
  fn deref_mut(&mut self) -> &mut [u8] {
    unsafe {
      let p = self.ptr.as_ptr();

      std::slice::from_raw_parts_mut(p, self.len)
    }
  }
}

impl std::ops::Drop for PlaneData {
  fn drop(&mut self) {
    unsafe {
      dealloc(self.ptr.as_ptr() as *mut u8, Self::layout(self.len, self.align));
    }
  }
}

impl PlaneData {
  unsafe fn layout(len: usize, align: usize) -> Layout {
    Layout::from_size_align_unchecked(
      len * mem::size_of::<u8>(),
      1 << align,
    )
  }

  unsafe fn new_uninitialized(len: usize, align: usize) -> Self {
    let ptr = {
      let ptr = alloc(Self::layout(len, align)) as *mut u8;
      std::ptr::NonNull::new_unchecked(ptr)
    };

    PlaneData { ptr, len, align, _marker: PhantomData }
  }

  pub fn new(len: usize, align: usize) -> Self {
    let mut pd = unsafe { Self::new_uninitialized(len, align) };

    for (i, v) in pd.iter_mut().enumerate() {
      eprintln!("index at {} len {}", i, len);
      *v = 128;
    }

    pd
  }
}

fn main() {
    use std::str::FromStr;
    let align = usize::from_str(&std::env::args().nth(1).unwrap_or("5".into())).unwrap();
    let _pd = PlaneData::new(640 * 480, align);
}
