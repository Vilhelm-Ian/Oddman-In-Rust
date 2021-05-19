fn main() {
    let mut vec = vec![1, -7, 5, -7, 9, 5, 1, 2, 2];
    vec.sort();
    let mut i = 0;
    while i < vec.len() {
        if i == vec.len() - 1 {
            println!("{:?}", vec[i]);
            break;
        }
        if vec[i] != vec[i + 1] {
            println!("{:?}", vec[i]);
            break;
        }
        i += 2;
    }
}
