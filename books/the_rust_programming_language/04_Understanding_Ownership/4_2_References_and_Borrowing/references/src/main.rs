fn main() {
    let s1 = String::from("hello");
    let length: usize = calculate_length(&s1);
    println!("The length of '{s1}' is {length}")
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
