extern crate cc;

use std::path::PathBuf;

fn main() {
    let tree_sitter_json: PathBuf = std::fs::canonicalize::<PathBuf>(["..", "tree-sitter-json", "src"].iter().collect()).unwrap();
    
    cc::Build::new()
        .include(&tree_sitter_json)
        .file(tree_sitter_json.join("parser.c"))
        //.file(tree_sitter_json.join("scanner.c"))
        .compile("tree-sitter-json");
}