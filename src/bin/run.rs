use ramhas::*;

fn main() {
    let mut parser =
        Parser::new("int a = 1; int b = 2; int c = a + b; { int b = 3; c = a + b; } return c;");
    let node = parser.program();
    println!("result: {}", node.borrow());
}
