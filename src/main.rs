extern crate playground;

use playground::english::{farewells, greetings};
use playground::japanese;

fn main() {
	println!("Hello in English: {}", greetings::hello());
	println!("Goodbye in English: {}", farewells::goodbye());
	
	println!("Hello in Japanese: {}", japanese::hello());
	println!("Goodbye in Japanese: {}", japanese::goodbye());
}


//#![feature(todo_macro)]
//
//trait Foo {
//    fn bar(&self);
//    fn baz(&self);
//}
//
//struct MyStruct;
//
//impl Foo for MyStruct {
//	fn bar(&self) {
//		// implementation goes here
//	}
//
//	fn baz(&self) {
//		// let's not worry about implementing baz() for now
//		todo!();
//	}
//}
//
//
//fn main() {
//	println!("Hello, world!");
//	let s = MyStruct;
//	s.bar();
//
//	// we aren't even using baz() yet, so this is fine.
//}
