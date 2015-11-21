pub fn make_arr_unsafe(len: usize) -> Box<[i32]> {
    use std::rt::heap::allocate;
    use std::mem::{min_align_of, transmute};
    use std::ptr::set_memory;
    use std::raw::Slice;

    let size = len * i32::BYTES;
    unsafe {
        let mem = allocate(size, min_align_of::<i32>());
        set_memory(mem, 0, size);
        let slice = Slice { data: mem as *const i32, len: len };
        transmute(slice)
    }
}
