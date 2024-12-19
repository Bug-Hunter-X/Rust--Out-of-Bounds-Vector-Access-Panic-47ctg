fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    // This will panic if index is out of bounds
    println!("Value at index {}: {}", index, vec[index]);
}