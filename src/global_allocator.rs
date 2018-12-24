use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;
use sys::{self, libc};

pub struct NspireAlloc;

const MIN_ALIGN: usize = 8;

/// Copied from libstd/sys/unix/alloc.rs
unsafe impl GlobalAlloc for NspireAlloc {
  #[inline]
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
      sys::malloc(layout.size()) as *mut u8
    } else {
      malloc_aligned(layout.align(), layout.size()) as *mut u8
    }
  }

  #[inline]
  unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
      sys::calloc(layout.size(), 1) as *mut u8
    } else {
      let ptr = self.alloc(layout.clone());
      if !ptr.is_null() {
        ptr::write_bytes(ptr, 0, layout.size());
      }
      ptr
    }
  }

  #[inline]
  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
      sys::free(ptr as *mut libc::c_void);
    } else {
      free_aligned(ptr as *mut libc::c_void);
    }
  }

  #[inline]
  unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    if layout.align() <= MIN_ALIGN && layout.align() <= new_size {
      sys::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
    } else {
      // Docs for GlobalAlloc::realloc require this to be valid:
      let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());

      let new_ptr = GlobalAlloc::alloc(self, new_layout);
      if !new_ptr.is_null() {
        let size = core::cmp::min(layout.size(), new_size);
        ptr::copy_nonoverlapping(ptr, new_ptr, size);
        GlobalAlloc::dealloc(self, ptr, layout);
      }
      new_ptr
    }
  }
}

// All of this converted from the C code at https://stackoverflow.com/a/6563989/9236675

const USIZE_SIZE: usize = 4;
const CHAR_SIZE: usize = 1;

unsafe fn malloc_aligned(alignment: usize, bytes: usize) -> *mut libc::c_void {
  let total_size = bytes + (2 * alignment) + USIZE_SIZE;
  let data = nspire_sys::malloc(CHAR_SIZE * total_size);
  if !data.is_null() {
    // store the original start of the malloc'd data.
    let data_start = data as *const libc::c_void;

    // dedicate enough space to the book-keeping.
    let data = data.add(USIZE_SIZE);

    // find a memory location with correct alignment.  the alignment minus
    // the remainder of this mod operation is how many bytes forward we need
    // to move to find an aligned byte.
    let offset = alignment - ((data as usize) % alignment);

    // set data to the aligned memory.
    let data = data.add(offset);

    // write the book-keeping.
    let book_keeping = (data.sub(USIZE_SIZE)) as *mut usize;
    *book_keeping = data_start as usize;
    data
  } else {
    data
  }
}

unsafe fn free_aligned(raw_data: *mut libc::c_void) {
  if !raw_data.is_null() {
    let data = raw_data as *mut i8;

    // we have to assume this memory was allocated with malloc_aligned.
    // this means the sizeof(size_t) bytes before data are the book-keeping
    // which points to the location we need to pass to free.
    let data = data.sub(USIZE_SIZE);

    // set data to the location stored in book-keeping.
    let data = (*(data as *mut usize)) as *mut i8;

    // free the memory.
    nspire_sys::free(data as *mut libc::c_void);
  }
}
