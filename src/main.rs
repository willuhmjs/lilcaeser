use lilcaeser::caeser;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("String");
    println!("{}", caeser(input.trim(), -3))
}
