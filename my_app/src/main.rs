use tree_sitter::{Parser, Language};
extern "C" { fn tree_sitter_json() -> Language; }

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_json() };
    parser.set_language(language).unwrap();

    let tree = parser.parse("[1, 2, {\"a\":3,\"b\":4}]", None).unwrap();

    println!("{}", tree.root_node().to_sexp());
}
