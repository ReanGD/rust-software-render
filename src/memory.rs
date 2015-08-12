use std;

#[allow(dead_code)]
pub fn typed_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u8,
                                   slice.len() * std::mem::size_of::<T>())
    }
}

pub fn bytes_to_typed<T>(slice: &mut [u8]) -> &mut [T] {
    unsafe {
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T,
                                       slice.len() / std::mem::size_of::<T>())
    }
}
