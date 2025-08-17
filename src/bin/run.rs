use ramhas::*;

fn main() {
    let mut parser =
        //Parser::new("int a = 1; int b = 2; int c = 0; { int b = 5; c = a + b; } { int e = 6; c = a + e; } return c;");
        Parser::new("int arg = 9; int a = 1; if (arg == 1) a = arg + 2; else a = arg - 3; return a;");
    parser.parse_program();
}
