// #![feature(allocator_api)]

// use std::alloc::GlobalAlloc;
// use std::alloc::{alloc, AllocError, Allocator, Global, Layout};
// use std::ptr::NonNull;

use std::fmt::{Debug, Formatter, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

struct ArenaAllocator<'a> {
	buffer: &'a mut [u8],
	sp: usize,
}

// unsafe impl<'a> Allocator for ArenaAllocator<'a> {
// 	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
// 		let sp_tmp = self.sp;
// 		let size = layout.size();
// 		let range = sp_tmp..sp_tmp + size;
// 		unsafe {
// 		}
// 		// self.sp += layout.size();
// 		let s = &self.buffer[range];
//
// 		;
// 	}
//
// 	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
// 		todo!()
// 	}
// }

impl ArenaAllocator<'_> {
	fn allocate(&mut self, size: usize) -> Result<NonNull<[u8]>, ()> {
		if self.sp + size > self.buffer.len() {
			return Err(());
		}

		let sp_tmp = self.sp;
		self.sp += size;

		let ptr = self.buffer[sp_tmp..sp_tmp + size].as_mut_ptr();

		let slice = std::ptr::slice_from_raw_parts_mut(
			ptr,
			size,
		);

		Ok(NonNull::new(slice).unwrap())
	}
}

impl Debug for ArenaAllocator<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut it = self.buffer.iter().peekable();
		let mut buffer_str = String::new();
		loop {
			match it.next() {
				None => {
					break;
				}
				Some(b) => {
					match it.peek() {
						None => {
							buffer_str.write_fmt(format_args!("{:02x}", b))?;
						}
						Some(_) => {
							buffer_str.write_fmt(format_args!("{:02x} ", b))?;
						}
					}
				}
			}
		}
		f.debug_struct("ArenaAllocator")
			.field("buffer", &buffer_str)
			.field("sp", &self.sp)
			.finish()?;

		Ok(())
	}
}

fn main() {
	let cap = 10;
	let mut arena_buffer = Vec::<u8>::with_capacity(cap);
	unsafe {
		arena_buffer.set_len(cap);
	}
	let mut arena = ArenaAllocator {
		buffer: arena_buffer.as_mut_slice(),
		sp: 0,
	};

	println!("arena at start:\t\t{arena:?}");

	let bytes = arena
		.allocate(
			// Layout::from_size_align(4, align_of::<u32>()).unwrap()
			4
		)
		.unwrap();

	println!("arena after allocation:\t{arena:?}");

	let v = unsafe {
		bytes.cast::<u32>().as_mut()
	};

	println!("arena after value cast:\t{arena:?}");
	println!("value hex before = {v:x}");

	*v = 0x11223344;

	println!("value hex after = {v:x}");
	println!("arena after value init:\t{arena:?}");

	_ = dbg!(arena.allocate(7));


	// let a = MaybeUninit::new(a);
	// let a = a.write(0x01010101u32);
}
