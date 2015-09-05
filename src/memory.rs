use std;
use cgmath::Vector3;

#[allow(dead_code)]
pub fn typed_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u8,
                                   slice.len() * std::mem::size_of::<T>())
    }
}

#[allow(dead_code)]
pub fn bytes_to_typed<T>(slice: &mut [u8]) -> &mut [T] {
    unsafe {
        std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T,
                                       slice.len() / std::mem::size_of::<T>())
    }
}

pub fn cast_to<T, U>(slice: &[U]) -> &[T] {
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const T,
                                   slice.len() * std::mem::size_of::<U>() / std::mem::size_of::<T>()
                                   )
    }
}

#[inline]
pub fn vector3_to_u32(vec: &Vector3<f32>) -> u32 {
    ((std::cmp::min(std::cmp::max((vec.x as i32), 0), 0xFF) << 16) +
     (std::cmp::min(std::cmp::max((vec.y as i32), 0), 0xFF) << 8) +
     (std::cmp::min(std::cmp::max((vec.z as i32), 0), 0xFF))
     ) as u32
}
