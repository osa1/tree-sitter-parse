# tree-sitter-parse

This is a tool that works like [tree-sitter-cli][1]'s `parse` command, except it
loads the parser from a shared library (`.so` on Linux). This is helpful when
testing your tree-sitter parser which can't be compiled with tree-sitter-cli,
maybe because you are using a lexer implemented in another language or something
like that.

## Example

tree-sitter-parse accepts three arguments:

- Path to the shared library (`.so` on Linux) for the tree-sitter parser
- Name of the language defined in the tree-sitter parser, i.e. the string in
  `name: '...'` part in your `grammar.js`.
- Path to the file to parse.

For example, I'm working on a parser that uses a lexer implemented in another
language. After building it to `.so` with something like:

```
$ clang-10 -I src src/parser.c -fPIC libmylexer.a -shared -o parser.so
```

I can use the `.so` file to parse a file using:

```
$ tree-sitter-parse <path_to_parser.so> mylang path_to_input
```

Parsing code is copied from tree-sitter-cli so the output should be similar to
what `tree-sitter parse` generates.

[1]: https://github.com/tree-sitter/tree-sitter/tree/master/cli
[2]: https://github.com/tree-sitter/tree-sitter-rust.git
