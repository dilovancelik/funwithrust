fn main() {
    let array: [i32; 12] = [1,2,3,4,5,6,7,8,9,10,11,12];

    for x in array.iter() {
        for y in array.iter() {
            print!("{number:>width$}", number=x * y, width=4);
        }
        print!("\n")
    }
}
