extern crate playground;

use playground::english::{farewells, greetings};
use playground::japanese;

fn main() {
	println!("Hello in English: {}", greetings::hello());
	println!("Goodbye in English: {}", farewells::goodbye());
	
	println!("Hello in Japanese: {}", japanese::hello());
	println!("Goodbye in Japanese: {}", japanese::goodbye());
}


trait Foo {
   fn bar(&self);
   fn baz(&self) -> i32;
}

struct MyStruct;

impl Foo for MyStruct {
	fn bar(&self) {
		// implementation goes here
	}

	fn baz(&self) -> i32 {
		// let's not worry about implementing baz() for now
		todo!()
	}
}
