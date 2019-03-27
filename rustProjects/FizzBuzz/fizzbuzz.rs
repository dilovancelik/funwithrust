static FIZZ_INT: i32 = 3;
static BUZZ_INT: i32 = 5;

fn main() {
    for i in 1..=100 {
        println!("{fizz}{buzz}{number}",  fizz=fizz(i), buzz=buzz(i), number=number(i));
    }
}

fn fizz(x: i32) -> String {
    if x % FIZZ_INT == 0 {
        return String::from("Fizz ");
    } else {
        return String::from("");
    }
}

fn buzz(x: i32) -> String {
    if x % BUZZ_INT == 0 {
        return String::from("Buzz ");
    } else {
        return String::from("");
    }
}

fn number(x: i32) -> String {
    if x % FIZZ_INT == 0 || x % BUZZ_INT == 0 {
        return String::from("");
    } else {
        return String::from(x.to_string());
    }
}
