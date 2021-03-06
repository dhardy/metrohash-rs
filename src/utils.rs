use std::num::Wrapping;
use std::ptr;

macro_rules! impl_read {
	($fn_name: ident, $ty: ty) => (
		#[inline(always)]
		pub fn $fn_name(ptr_addr: usize) -> Wrapping<u64> {
			let ptr: *const $ty = ptr_addr as *const $ty;
			Wrapping(unsafe { *ptr as u64 })
		}
	)
}

impl_read!(read_u64, u64);
impl_read!(read_u32, u32);
impl_read!(read_u16, u16);
impl_read!(read_u8, u8);


#[inline]
pub unsafe fn copy_32(src: *const u8, dest: *mut u8, n: usize) {
    debug_assert!(n < 32);
    match n {
        31 => ptr::copy_nonoverlapping(src, dest, 31),
        30 => ptr::copy_nonoverlapping(src, dest, 30),
        29 => ptr::copy_nonoverlapping(src, dest, 29),
        28 => ptr::copy_nonoverlapping(src, dest, 28),
        27 => ptr::copy_nonoverlapping(src, dest, 27),
        26 => ptr::copy_nonoverlapping(src, dest, 26),
        25 => ptr::copy_nonoverlapping(src, dest, 25),
        24 => ptr::copy_nonoverlapping(src, dest, 24),
        23 => ptr::copy_nonoverlapping(src, dest, 23),
        22 => ptr::copy_nonoverlapping(src, dest, 22),
        21 => ptr::copy_nonoverlapping(src, dest, 21),
        20 => ptr::copy_nonoverlapping(src, dest, 20),
        19 => ptr::copy_nonoverlapping(src, dest, 19),
        18 => ptr::copy_nonoverlapping(src, dest, 18),
        17 => ptr::copy_nonoverlapping(src, dest, 17),
        16 => ptr::copy_nonoverlapping(src, dest, 16),
        15 => ptr::copy_nonoverlapping(src, dest, 15),
        14 => ptr::copy_nonoverlapping(src, dest, 14),
        13 => ptr::copy_nonoverlapping(src, dest, 13),
        12 => ptr::copy_nonoverlapping(src, dest, 12),
        11 => ptr::copy_nonoverlapping(src, dest, 11),
        10 => ptr::copy_nonoverlapping(src, dest, 10),
        9 => ptr::copy_nonoverlapping(src, dest, 9),
        8 => ptr::copy_nonoverlapping(src, dest, 8),
        7 => ptr::copy_nonoverlapping(src, dest, 7),
        6 => ptr::copy_nonoverlapping(src, dest, 6),
        5 => ptr::copy_nonoverlapping(src, dest, 5),
        4 => ptr::copy_nonoverlapping(src, dest, 4),
        3 => ptr::copy_nonoverlapping(src, dest, 3),
        2 => ptr::copy_nonoverlapping(src, dest, 2),
        1 => ptr::copy_nonoverlapping(src, dest, 1),
        _ => (),
    }
}

#[inline(always)]
pub fn rotate_right(v: Wrapping<u64>, k: u32) -> Wrapping<u64> {
    Wrapping(v.0.rotate_right(k))
}
