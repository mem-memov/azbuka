mod store;

fn main() {

	let container = store::Container::new("C:\\Users\\anna\\Documents\\rust\\test.txt");
	container.append();
	//println!("Hello, world!");

}
