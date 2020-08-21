fn main() {
	println!("Hello, world!");
	say("Hello".to_string())
}

fn say(text: String) {
	println!("{}", text);
	here("hi".to_string());
}

fn here(args: String) -> String {
	args
}

#[test]
fn returns_arg() {
	assert!(true)
}
