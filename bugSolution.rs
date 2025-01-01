fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0; // Or any valid index
    if index < v.len() {
        v[index] = 10; 
    } else {
        // Handle out of bounds access
        eprintln!("Index out of bounds!");
    }
    println!("v: {:?}", v);
}
