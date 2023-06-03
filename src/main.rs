mod memory;
mod language;
use language::Lexer;
fn main() {
    let input = String::from("1 2: buckle my shoe, 3 4");
    let lexer = Lexer::new(input);
    lexer.print_tokens();
    println!("i think memory is working");
    println!("Hello, world!");
}
