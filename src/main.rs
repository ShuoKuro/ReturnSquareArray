fn main() {
    println!(
        "{},{},{},{},{}",
        arr_square()[0],
        arr_square()[1],
        arr_square()[2],
        arr_square()[3],
        arr_square()[4]
    );
}
fn arr_square() -> [i32; 5] {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        arr[i] *= arr[i];
    }
    return arr;
}
