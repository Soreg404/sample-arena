use std::alloc::{alloc, dealloc, Layout};
use std::fmt::{Debug, Formatter, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

struct ArenaAllocator<'a> {
	buffer: &'a mut [u8],
	sp: usize,
}

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

	fn allocate_t<'a, 'b, T>(&'a mut self) -> Result<&'b mut MaybeUninit<T>, ()> {
		let size = size_of::<T>();
		let bytes = self.allocate(size)?;

		unsafe {
			Ok(bytes.cast::<MaybeUninit<T>>().as_mut())
		}
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

	// let mut arena_buffer = Vec::<u8>::with_capacity(cap);
	// unsafe {
	// 	arena_buffer.set_len(cap);
	// }

	// NOTE: idk what i'm doing here
	let ptr;
	let layout = Layout::from_size_align(cap, align_of::<u8>())
		.unwrap();
	let arena_buffer = unsafe {
		ptr = alloc(layout);
		core::ptr::slice_from_raw_parts_mut(
			ptr,
			cap,
		)
			.as_mut()
			.unwrap()
	};
	////////////

	let mut arena = ArenaAllocator {
		buffer: arena_buffer,
		sp: 0,
	};

	println!("arena at start:\t\t\t{arena:?}");

	let bytes = arena
		.allocate(
			// Layout::from_size_align(4, align_of::<u32>()).unwrap()
			4
		)
		.unwrap();

	println!("arena after allocation:\t\t{arena:?}");

	let v = unsafe {
		bytes.cast::<MaybeUninit<u32>>().as_mut()
	};

	unsafe {
		println!("value before write: {:x}", v.assume_init_mut());
	}

	v.write(0x44332211);

	println!("arena after value write:\t{arena:?}");
	let v = unsafe {
		v.assume_init_mut()
	};
	println!("value hex = {v:x}");


	println!("======");
	let vf = arena.allocate_t::<f32>()
		.unwrap()
		.write(20f32);

	println!("arena after another alloc:\t{arena:?}");
	println!("value = {vf}");

	unsafe {
		dbg!({
			arena.buffer[4..10]
				.as_ptr()
				.cast::<f32>()
				.as_ref()
		});
	}

	let varr = arena.allocate_t::<[u8; 2]>()
		.unwrap();
	let varr = unsafe { varr.assume_init_mut() };

	println!("======");
	println!("arena before array write:\t{arena:?}");
	varr[0] = 0x88;
	varr[1] = 0x99;
	println!("arena after array write:\t{arena:?}");


	println!("======");
	println!("alloc error:");
	_ = dbg!(arena.allocate(2));

	unsafe {
		// NOTE to self: unnecessary at the end of program (system will clean it better/faster/stronger)
		dealloc(ptr, layout);
	}
}
