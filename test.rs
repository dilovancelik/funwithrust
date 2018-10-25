fn main() {
    let s = String::from("test");

    test(&s);

    println!("{}", s);

}

fn test(string: &String){
    println!("{}", string);
}