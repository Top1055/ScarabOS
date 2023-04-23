const BLOCK_SIZE: usize = 4096; // 4KB blocks
const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut BITMAP: [u8; HEAP_SIZE / BLOCK_SIZE / 8] = [0; HEAP_SIZE / BLOCK_SIZE / 8];

pub fn alloc(size: usize) -> *mut u8 {
    // Round up size to nearest block size
    let num_blocks = (size + BLOCK_SIZE - 1) / BLOCK_SIZE;

    unsafe {
        // Find first free block in bitmap
        let mut byte_idx = 0;
        let mut bit_idx = 0;
        let mut found = false;
        for (i, byte) in BITMAP.iter().enumerate() {
            if *byte != 0xff {
                for j in 0..8 {
                    if (*byte & (1 << j)) == 0 {
                        byte_idx = i;
                        bit_idx = j;
                        found = true;
                        break;
                    }
                }
            }
            if found {
                break;
            }
        }

        if !found {
            return core::ptr::null_mut(); // Out of memory
        }

        // Mark blocks as allocated in bitmap
        for i in 0..num_blocks {
            let byte = &mut BITMAP[byte_idx + (bit_idx + i) / 8];
            *byte |= 1 << ((bit_idx + i) % 8);
        }

        // Return pointer to allocated memory
        &mut HEAP[byte_idx * BLOCK_SIZE + bit_idx * 8 * BLOCK_SIZE] as *mut u8
    }
}

pub fn free(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }

    // Round up size to nearest block size
    let num_blocks = (size + BLOCK_SIZE - 1) / BLOCK_SIZE;

    unsafe {
        // Find starting block in heap
        let start_block = ((ptr as usize) - (HEAP.as_ptr() as usize)) / BLOCK_SIZE;

        // Mark blocks as free in bitmap
        for i in 0..num_blocks {
            let byte = &mut BITMAP[start_block / 8 + i];
            *byte &= !(1 << (start_block % 8 + i % 8));
        }
    }
}
