mod cli;

use std::io::Write;

use cli::{parse_args, Args};
use libloading as lib;
use tree_sitter::{Language, Parser};

fn main() {
    let Args {
        library_path,
        language_name,
        file_path,
    } = parse_args();

    println!("library_path={:?}", library_path);
    println!("language_name={:?}", language_name);
    println!("file_path={:?}", file_path);

    let library = lib::Library::new(&library_path).expect("Unable to load dynamic library");
    let language_name = format!("tree_sitter_{}", language_name.replace('-', "_"));
    let language: Language = unsafe {
        let language_fn: lib::Symbol<unsafe extern "C" fn() -> Language> = library
            .get(language_name.as_bytes())
            .expect("Failed to load symbol");
        language_fn()
    };

    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("Unable to create parser");

    let file_contents = std::fs::read_to_string(file_path).expect("Unable to input file");
    let tree = parser
        .parse(file_contents.as_bytes(), None)
        .expect("Unable to parse file");
    let mut cursor = tree.walk();

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut needs_newline = false;
    let mut indent_level = 0;
    let mut did_visit_children = false;
    loop {
        let node = cursor.node();
        let is_named = node.is_named();
        if did_visit_children {
            if is_named {
                stdout.write(b")").unwrap();
                needs_newline = true;
            }
            if cursor.goto_next_sibling() {
                did_visit_children = false;
            } else if cursor.goto_parent() {
                did_visit_children = true;
                indent_level -= 1;
            } else {
                break;
            }
        } else {
            if is_named {
                if needs_newline {
                    stdout.write(b"\n").unwrap();
                }
                for _ in 0..indent_level {
                    stdout.write(b"  ").unwrap();
                }
                let start = node.start_position();
                let end = node.end_position();
                if let Some(field_name) = cursor.field_name() {
                    write!(&mut stdout, "{}: ", field_name).unwrap();
                }
                write!(
                    &mut stdout,
                    "({} [{}, {}] - [{}, {}]",
                    node.kind(),
                    start.row,
                    start.column,
                    end.row,
                    end.column
                )
                .unwrap();
                needs_newline = true;
            }
            if cursor.goto_first_child() {
                did_visit_children = false;
                indent_level += 1;
            } else {
                did_visit_children = true;
            }
        }
    }
    cursor.reset(tree.root_node());
    println!("");
}
