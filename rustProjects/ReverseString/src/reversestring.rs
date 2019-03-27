fn main() {
    println!("{:?}",reverse_string("Hello"));
}

fn reverse_string(text: &str) -> String {
    return text.chars().rev().collect::<String>();
}
