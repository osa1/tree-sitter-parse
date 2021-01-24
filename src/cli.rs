use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

// See help strings below for the fields
pub(crate) struct Args {
    pub library_path: String,
    pub language_name: String,
    pub file_path: String,
    pub query: Option<String>,
}

pub(crate) fn parse_args() -> Args {
    let mut version = crate_version!().to_owned();
    let commit_hash = env!("GIT_HASH");
    if !commit_hash.is_empty() {
        version = format!("{} ({})", version, commit_hash);
    }

    let m = App::new(crate_name!())
        .version(version.as_str())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("LIBRARY_PATH")
                .takes_value(true)
                .required(true)
                .help("Path to the tree-sitter parser (.so file)"),
        )
        .arg(
            Arg::with_name("LANGUAGE_NAME")
                .takes_value(true)
                .required(true)
                .help("Name of the language defiend in the tree-sitter parser"),
        )
        .arg(
            Arg::with_name("FILE_PATH")
                .takes_value(true)
                .required(true)
                .help("Path to the file to parse"),
        )
        .arg(
            Arg::with_name("QUERY")
                .takes_value(true)
                .required(false)
                .help("tree-sitter query to run on the parsed AST"),
        )
        .get_matches();

    Args {
        library_path: m.value_of("LIBRARY_PATH").unwrap().to_owned(),
        language_name: m.value_of("LANGUAGE_NAME").unwrap().to_owned(),
        file_path: m.value_of("FILE_PATH").unwrap().to_owned(),
        query: m.value_of("QUERY").map(str::to_owned),
    }
}
