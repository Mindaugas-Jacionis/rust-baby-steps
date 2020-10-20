fn main() {
    let name = "Mindaugas".to_string();
    let surname = "Jačionis".to_string();

    say_name(name, &surname);
}


fn say_name(first: String, last: &String) {
    println!("{} {}", first, last);
}