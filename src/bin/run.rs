use ramhas::*;
use std::path::Path;

fn main() {
    let mut parser =
        Parser::new("int a = 1; int b = 2; int c = 0; { int b = 5; c = a + b; } { int e= 6; c = a + e; } return c;");
    let result = parser.program();
}
