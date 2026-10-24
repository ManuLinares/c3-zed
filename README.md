# C3 Extension for Zed

C3 language support for the [Zed editor](https://zed.dev).

## Features

- Syntax highlighting via [tree-sitter-c3](https://github.com/c3lang/tree-sitter-c3)
- Outline view
- Indentation rules
- Automatic LSP management via [c3_ls](https://github.com/ManuLinares/c3_ls) (downloads precompiled binaries or resolves `c3_ls` from `PATH`)

## Development

To test locally:
1. Run `zeditor: install dev extension` in Zed.
2. Select this directory.

To update queries from upstream:
```bash
./scripts/update_grammar.sh
```
