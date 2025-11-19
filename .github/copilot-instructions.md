# GitHub Copilot Instructions

This project uses **Feather**, a custom Domain Specific Language (DSL) for scripting and automation with a focus on readability, safety, and cross-platform compatibility.

## Core Principles

- English-like syntax with minimal symbols
- Explicit over implicit (no embedded conditionals)
- Strong type safety: records (`@variable`) and containers (`::container`)
- Security-first with `vault`, `sensitive`, and `with risk`
- All commands end with `.`

## When Suggesting Code

**Assume:**

- User is writing Feather DSL in `.sky` files
- Refer to `docs/` for patterns and examples
- Check `packages/feather-core-vscode-extension/examples/` for real usage

**Follow Feather Syntax:**

- Records: `@variable` (scalar values)
- Containers: `::container` (structured data)
- Projections: `::container:field`
- Schema: `&schema_name`
- Sealed: `!@constant` or `!::container:field`
- Comments: `~` for single line
- Keywords: `into`, `with`, `without`, `as`, `is`, `from`, `to`
- Flow: `if`/`else`/`end`, `repeat`, `while`, `function`/`end`
- Signatures: `trace`, `elapsed`, `timeout`, `silent`, `on <os>`
- Clauses: `safe`, `sensitive`, `elevated`, `async`

**Do NOT Suggest:**

- Embedded conditionals: ❌ `if @a > 5` → ✅ `math compare @a > '5' into @result.`
- Python/JS/shell syntax unless explicitly requested
- Semicolons or other terminators (except `.`)
- Brackets except in specific contexts

## Common Patterns

### Variable Assignment

```sky
'hello' into @text.
42 into @number.
```

### Conditionals (Must compute first)

```sky
math compare @value > '10' into @is_greater.
if @is_greater
  say 'Value is greater than 10'.
end
```

### Module Calls with Signatures

```sky
cabinet read file 'data.txt' into @content trace elapsed timeout 5 seconds on linux.
```

### Sensitive Data

```sky
vault lock 'password' with secret @pw.
vault unlock 'password' into @pw.
sensitive say @pw with risk.
```

### Functions

```sky
function greet
  parameter 1 into @name.
  say 'Hello, @{name}!'.
  success.
end

greet 'Alice'.
```

## Example Snippets

- `cabinet append file 'login.log' with @log_entry with risk elapsed timeout 5 seconds.`
- `table ::metrics into @table_view trace.`
- `say 'I am on linux.' elapsed on linux.`
- `text concat 'Hello' ' ' 'World' into @greeting.`
- `bool all @condition1 @condition2 into @result.`

## Key Documentation

- [Basics](./docs/basics/) - Language fundamentals
- [Modules](./docs/modules/) - Built-in modules
- [Schemas](./docs/schemas/) - Type system
- [Examples](./packages/feather-core-vscode-extension/examples/) - Real scripts

For best results, always consult documentation before suggesting complex patterns.
