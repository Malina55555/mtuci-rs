use std::ptr;
fn main() {}
struct MyVector<T> {
	inside: *mut T,
	length: usize,
	capas: usize,
}

impl<T> MyVector<T> {
	fn new() -> Self {
		MyVector {
			inside: std::ptr::null_mut(),
			length: 0,
			capas: 0,
		}
	}


	fn with_capacity(need_len: usize) -> Self {
		let mut vector = MyVector::new();
		vector.reserve(need_len);
		vector
	}

	fn push(&mut self, new_element: T) {
    
		unsafe {
			std::ptr::write(self.inside.add(self.length), new_element);
		}
    
		self.length += 1;
	}

	fn pop(&mut self) -> Option<T> {
		if self.length == 0 {
			return None;
		}
    
		unsafe {
			self.length -= 1;
			Some(std::ptr::read(self.inside.add(self.length)))
		}
	}

	fn remove(&mut self, index: usize) -> Option<T> {
		if index >= self.length {
			return None;
		}
    
		let result;
    
		unsafe {
			result = std::ptr::read(self.inside.add(index));
			std::ptr::copy(self.inside.add(index + 1), self.inside.add(index), self.length - index - 1);
		}
    
		self.length -= 1;
    
		Some(result)
	}

	fn get(&self, index: usize) -> Option<&T> {
		if index >= self.length {
			return None;
		}
    
		unsafe {
			Some(&*self.inside.add(index))
		}
	}

	fn resize(&mut self, new_size: usize) {
		if new_size > self.length {
			self.reserve(new_size - self.length);
		} else {
			unsafe {
				for i in new_size..self.length {
					std::ptr::drop_in_place(self.inside.add(i));
				}
			}
		}
    
		self.length = new_size;
	}

	fn reserve(&mut self, additional: usize) {
		if self.length + additional <= self.capas {
			return;
		}
    
		let new_capacity = (self.length + additional).next_power_of_two();
		let mut new_buffer = unsafe {
			std::alloc::alloc(std::alloc::Layout::array::<T>(new_capacity).unwrap()) as *mut T
		};
    
		unsafe {
			std::ptr::copy_nonoverlapping(self.inside, new_buffer, self.length);
        
			if !self.inside.is_null() {
				std::alloc::dealloc(self.inside as *mut u8, std::alloc::Layout::array::<T>(self.capas).unwrap());
			}
		}
    
		self.inside = new_buffer;
		self.capas = new_capacity;
	}
}
