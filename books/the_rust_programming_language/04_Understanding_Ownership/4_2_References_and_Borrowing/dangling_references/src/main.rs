fn main() {
//    let reference_to_nothing = dangle();
    let reference_to_string = no_dangle();
    println!("{reference_to_string}")
}

//fn dangle() -> &String { // dangle returns a reference to a String
//
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}