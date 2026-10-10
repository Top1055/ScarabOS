use core::alloc::{GlobalAlloc, Layout};

pub struct ScarabAllocator;

#[cfg_attr(not(test), global_allocator)]
static ALLOCATOR: ScarabAllocator = ScarabAllocator;
unsafe impl GlobalAlloc for ScarabAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() > BLOCK_SIZE {
            return core::ptr::null_mut();
        }
        alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        free(ptr, layout)
    }
}

const BLOCK_SIZE: usize = 4096; // 4KB blocks
const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap
const BITMAP_SIZE: usize = HEAP_SIZE / BLOCK_SIZE / 8;

#[repr(C, align(4096))]
struct Heap {
    pub data: [u8; HEAP_SIZE],     // the actual data itself
    pub bitmap: [u8; BITMAP_SIZE], // each bit of each u8 is an entry
}

impl Heap {
    pub fn is_free(&self, adr: usize) -> bool {
        let byte = adr / 8;
        let bit = adr % 8;
        self.bitmap[byte] & (1 << bit) == 0
    }

    pub fn mark_used(&mut self, start: usize, num_blocks: usize) {
        let byte = start / 8;
        let bit = start % 8;

        for i in 0..num_blocks {
            let block = &mut self.bitmap[byte + (bit + i) / 8];
            // grab the bit we're looking at this loop
            let block_bit = (bit + i) % 8;
            let mask = 1 << block_bit; // the one is the flipped bit, and we slide it to the
                                       // left by `block_bit`

            assert!(*block & mask == 0, "Allocator handing out used memory");

            *block |= mask;
            // or the mask onto the block
            // equivilent: block = block | mask
        }
    }

    pub fn mark_free(&mut self, start: usize, num_blocks: usize) {
        let byte = start / 8;
        let bit = start % 8;

        for i in 0..num_blocks {
            let block = &mut self.bitmap[byte + (bit + i) / 8];
            let block_bit = (bit + i) % 8;
            let mask = 1 << block_bit;
            assert!(*block & mask != 0, "Double free");
            *block &= !mask;
        }
    }

    pub unsafe fn create_pointer(&mut self, start: usize) -> *mut u8 {
        let address = start * BLOCK_SIZE;
        &mut self.data[address] as *mut u8
    }
}

static mut HEAP: Heap = Heap {
    data: [0; HEAP_SIZE],
    bitmap: [0; BITMAP_SIZE],
};

#[inline]
pub fn layout_to_blocks(layout: Layout) -> usize {
    (layout.size() + BLOCK_SIZE - 1) / BLOCK_SIZE
}

pub fn alloc(layout: Layout) -> *mut u8 {
    // Round up size to nearest block size

    let num_blocks = layout_to_blocks(layout);

    unsafe {
        // Set these at the beginning position of the block we're allocating to
        let mut address_start = 0;
        let mut free_counter = 0;
        let mut found = false;

        for block in 0..BITMAP_SIZE * 8 {
            if HEAP.is_free(block) {
                free_counter += 1;
                if free_counter == num_blocks {
                    found = true;
                    break;
                }
            } else {
                free_counter = 0;
                address_start = block + 1
            }
        }

        if !found {
            return core::ptr::null_mut(); // Out of memory
        }

        // Mark blocks as allocated in bitmap
        HEAP.mark_used(address_start, num_blocks);

        // Return pointer to allocated memory
        HEAP.create_pointer(address_start)
    }
}

pub fn free(ptr: *mut u8, layout: Layout) {
    if ptr.is_null() {
        return;
    }

    let num_blocks = layout_to_blocks(layout);
    // turn the pointer addrress into a number
    unsafe {
        // subtract the start address of the heap to find ptr's address inside it
        // then divide to turn into bitmap
        let adr = ptr as usize - HEAP.data.as_ptr() as usize;
        let start_block = adr / BLOCK_SIZE;

        // Freeing a pointer alloc never returned e.g. p + 16 or a stack address
        assert!(adr % BLOCK_SIZE == 0, "Invalid free");

        // Mark blocks as free in bitmap
        HEAP.mark_free(start_block, num_blocks);
    }
}
