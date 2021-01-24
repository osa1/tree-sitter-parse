mod cli;

use std::io::Write;

use cli::{parse_args, Args};
use libloading as lib;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

fn main() {
    let Args {
        library_path,
        language_name,
        file_path,
        query,
    } = parse_args();

    println!("library_path={:?}", library_path);
    println!("language_name={:?}", language_name);
    println!("file_path={:?}", file_path);
    if let Some(query) = query.as_ref() {
        println!("query={:?}", query);
    }

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

    if let Some(query) = query {
        println!();
        println!("Query result(s):");
        println!();

        let query = Query::new(language, &query).unwrap();
        let root = tree.root_node();

        println!("Matches:");
        let mut query_cursor = QueryCursor::new();
        for match_ in query_cursor.matches(&query, root, to_callback(&file_contents)) {
            println!("  {:?} (pattern_index={})", match_, match_.pattern_index);
            for capture in match_.captures {
                let node = capture.node;
                println!("    {:?} (index={})", node, capture.index);
            }
        }

        println!("Captures:");
        for (match_, _) in query_cursor.captures(&query, root, to_callback(&file_contents)) {
            println!("  {:?} (pattern_index={})", match_, match_.pattern_index);
            for capture in match_.captures {
                let node = capture.node;
                println!("    {:?} (index={})", node, capture.index);
            }
        }
    }
}

fn to_callback<'a>(source: &'a str) -> impl Fn(Node) -> &'a [u8] {
    move |n| &source.as_bytes()[n.byte_range()]
}
