use sqparse::{parse, tokenize};

fn main() {
    let source = include_str!("print_parser_error_script.nut");
    let tokens = tokenize(source).unwrap();
    let parse_err = parse(&tokens).unwrap_err();

    println!(
        "{}",
        parse_err.display(source, &tokens, Some("print_parser_error_script.nut"))
    );
}
