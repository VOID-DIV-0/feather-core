# GitHub Copilot Instructions

This project uses a custom Domain Specific Language (DSL).

**When suggesting code or completions:**

- Assume the user is working on the DSL.
- Refer to the `docs/` folder for examples and documentation on how the DSL is written.
- Prefer DSL-style syntax and patterns found in the documentation.
- Use project-specific naming conventions and formatting styles as seen in the docs.
- Avoid suggesting code in other languages unless explicitly requested.
- If unsure, default to DSL syntax and consult the documentation.

For best results, check the documentation in `docs/` before making suggestions.

## Example DSL Snippets

- `cabinet append file 'login.log' with @log_entry with risk elapsed timeout 5 seconds.`
- `table ::metrics into @table_view trace.`
- `say 'I am on linux.' elapsed on linux.`
