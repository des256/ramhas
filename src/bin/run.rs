use ramhas::*;

fn main() {
    let mut parser = Parser::new("return 12 + 19 * 2 - (27 - 4) + 2;");
    let id = parser.parse_program();

    parser.print_state();
    println!("result: {:?}", id);
}
