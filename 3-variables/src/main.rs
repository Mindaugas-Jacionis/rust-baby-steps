fn main() {
    let name = "Mindaugas"; // immutable variable
    let mut surname = "N/A"; // mutable variable

    println!("{} {}", name, surname);

    surname = "Jačionis";

    println!("{} {}", name, surname);
}
