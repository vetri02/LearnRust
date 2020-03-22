fn main() {
    let first = "Vetri".to_string();
    let last = "Chelvan".to_string();

    say_name(&first, &last);
    say_name(&first, &last);
}

fn say_name(first: &String, last: &String){
    println!("{} {}", first, last);
}
