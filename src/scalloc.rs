use core::alloc::{GlobalAlloc, Layout};

pub struct ScarabAllocator;

#[global_allocator]
static ALLOCATOR: ScarabAllocator = ScarabAllocator;
unsafe impl GlobalAlloc for ScarabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}

const BLOCK_SIZE: usize = 4096; // 4KB blocks
const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap
const HEADER_LEN: usize = 8;

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut BITMAP: [u8; HEAP_SIZE / BLOCK_SIZE / 8] = [0; HEAP_SIZE / BLOCK_SIZE / 8];

pub fn alloc(size: usize) -> *mut u8 {
    // Round up size to nearest block size

    let num_blocks = (size + HEADER_LEN + BLOCK_SIZE - 1) / BLOCK_SIZE;

    unsafe {
        // Find first free set of blocks in bitmap
        let mut byte_idx = 0;
        let mut bit_idx = 0;
        let mut free_counter = 0;
        let mut found = false;

        for (i, byte) in BITMAP.iter().enumerate() {
            if *byte != 0xff {
                // Check if byte is full
                for j in 0..8 {
                    /* Loop through bits to find free
                    // Increment timer until all blocks accounted for
                    // Reset upon taken space
                     */
                    if (*byte & (1 << j)) == 0 {
                        free_counter += 1;
                        if free_counter == num_blocks {
                            // Storing the last available block
                            found = true;
                            break;
                        }
                    } else {
                        free_counter = 0;
                        bit_idx = j + 1;
                    }
                }
            } else {
                free_counter = 0;
                byte_idx = i + 1;
                bit_idx = 0;
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
        let header = byte_idx * 8 * BLOCK_SIZE + bit_idx * BLOCK_SIZE;
        // Track size of alloc
        let header_ptr = HEAP.as_mut_ptr().add(header) as *mut usize;
        header_ptr.write(num_blocks);
        &mut HEAP[header + HEADER_LEN] as *mut u8
    }
}

pub fn free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Read from the header
        let header = ptr.sub(HEADER_LEN) as *mut usize;
        let num_blocks = header.read();
        // Find starting block in heap
        let start_block = ((header as usize) - (HEAP.as_ptr() as usize)) / BLOCK_SIZE;

        // Mark blocks as free in bitmap
        for i in 0..num_blocks {
            let block_number = start_block + i;
            let byte_idx = block_number / 8;
            let bit_idx = block_number % 8;
            let byte = &mut BITMAP[byte_idx];
            *byte &= !(1 << (bit_idx));
        }
    }
}
