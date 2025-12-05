# GitHub Copilot Instructions for nekonomicon

This repository implements **nekonomicon**, a custom language for scripting and automation, prioritizing readability, safety, and cross-platform compatibility. AI agents should follow these project-specific conventions and workflows to be productive immediately.

## Architecture & Structure

- **nekonomicon scripts** use `.spell` files (see `packages/nekonomicon-core-vscode-extension/examples/` for real scripts).
- **Core language**: Rust crates in `packages/nekonomicon-core/` (tokenizer, parser, grammar), with a modular design for extensibility.
- **Modules**: Each feature (e.g., `cabinet`, `vault`, `math`) is documented in `docs/modules/` and implemented as a language module.
- **VS Code extension**: Syntax highlighting and language support in `packages/nekonomicon-core-vscode-extension/`.
- **Documentation**: All language rules, patterns, and module APIs are in `docs/` (see especially `core/` and `modules/`).

## nekonomicon Language Conventions

- **English-like, minimal symbols**: Avoid brackets, semicolons, or embedded conditionals.
- **Explicit logic**: Compute conditions first, then use `if`/`else`/`end` blocks.
- **Type safety**: Use `@variable` for records (scalars), `::container` for structured data, and projections like `::container:field`.
- **Security**: Use `vault`, `sensitive` for secrets and sensitive data.
- **All commands end with `.`**
- **Clauses**: `safe`, `sensitive`, `elevated`, `async` must appear at the start of a command.
- **Signatures**: `trace`, `elapsed`, `timeout`, `silent`, `on <os>` can be combined at the end of commands (see `docs/core/signatures.md`).
- **No embedded conditionals**: Always compute and store results before using them in flow control.

## Developer Workflows

- **Build**: Use Cargo for Rust crates (`cargo build` in each `packages/nekonomicon-core-*` directory).
- **Test**: Run `cargo test` in the relevant package directory. See `tests/` for Rust and `.spell` for script examples.
- **Debug**: Use Rust's standard debugging tools for core, and VS Code for `.spell` scripts (with the extension).
- **Extension packaging**: See `packages/nekonomicon-core-vscode-extension/README.md` for VSIX build/install steps.

## Project-Specific Patterns

- **Variable assignment**:
  ```spell
  'hello' into @text.
  42 into @number.
  ```
- **Conditionals** (must compute first):
  ```spell
  math compare @value > '10' into @is_greater.
  if @is_greater
    say 'Value is greater than 10'.
  end
  ```
- **Sensitive data**:
  ```spell
  vault lock 'password' with secret @pw.
  vault unlock 'password' into @pw.
  sensitive say @pw with risk.
  ```
- **Functions**:
  ```spell
  function greet
    parameter 1 into @name.
    say 'Hello, @{name}!'.
    success.
  end
  greet 'Alice'.
  ```
- **Module calls with signatures**:
  ```spell
  cabinet read file 'data.txt' into @content trace elapsed timeout 5 seconds on linux.
  ```

## Key Files & References

- `docs/` — Language rules, module APIs, and core patterns (see `core/` and `modules/` subfolders)
- `packages/nekonomicon-core-vscode-extension/examples/` — Real nekonomicon scripts
- `README.md` — Project philosophy and goals

**For advanced patterns, always consult the relevant `docs/` page before suggesting code.**
