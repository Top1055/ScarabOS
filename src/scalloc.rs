use core::alloc::{GlobalAlloc, Layout};

pub struct ScarabAllocator;

//#[global_allocator]
#[cfg_attr(not(test), global_allocator)]
static ALLOCATOR: ScarabAllocator = ScarabAllocator;
unsafe impl GlobalAlloc for ScarabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}

const BLOCK_SIZE: usize = 4096; // 4KB blocks
const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap
const HEADER_LEN: usize = 8;

#[repr(align(4096))]
struct Heap {
    pub data: [u8; HEAP_SIZE],
}

static mut HEAP: Heap = Heap {
    data: [0; HEAP_SIZE],
};

static mut BITMAP: [u8; HEAP_SIZE / BLOCK_SIZE / 8] = [0; HEAP_SIZE / BLOCK_SIZE / 8];

pub fn alloc(layout: Layout) -> *mut u8 {
    // Round up size to nearest block size

    let num_blocks = (layout.size() + HEADER_LEN + BLOCK_SIZE - 1) / BLOCK_SIZE;

    unsafe {
        // Set these at the beginning position of the block we're allocating to
        let mut byte_idx = 0;
        let mut bit_idx = 0;
        let mut free_counter = 0;
        let mut found = false;

        // Find first free set of blocks in bitmap
        for (i, byte) in BITMAP.iter().enumerate() {
            if *byte != 0xff {
                // Check if byte is full
                for j in 0..8 {
                    /* Loop through bits to find free
                    // Increment counter until all blocks accounted for
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
                        // when finding a used bit, trip the reset
                        free_counter = 0;
                        byte_idx = i;
                        bit_idx = j + 1;
                    }
                }
            } else {
                // Gloss over a full byte
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
        let header_ptr = HEAP.data.as_mut_ptr().add(header) as *mut usize;
        header_ptr.write(num_blocks);
        &mut HEAP.data[header + HEADER_LEN] as *mut u8
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
        let start_block = ((header as usize) - (HEAP.data.as_ptr() as usize)) / BLOCK_SIZE;

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
