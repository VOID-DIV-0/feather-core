# Feather Style Guidelines

## General Philosophy

- Prioritize readability over brevity.
- Use consistent lowercase for all keywords and identifiers.
- Indent nested structures with two spaces per level.

## Line Length

- Keep statements on a single line if they are short.
- Use multiline formatting if the line exceeds approximately 80–100 characters.

## Arguments with Separators

- Use commas to separate arguments clearly.
- If arguments fit comfortably on one line, keep them single-line.
- For longer argument lists, use multiline formatting with each argument aligned vertically.

## Modifiers (`with` / `without` / `select`)

- Place modifiers on new lines, indented.
- Align modifiers vertically for improved readability.

## Grouping Rules

- Group multiple modifiers together using commas.
- Separate grouped modifiers onto new lines with proper indentation.

## Examples

### Short single-line

```feather
module example with arg1, arg2 without flag
```

### Wrapped multiline with separators

```feather
module example with
  arg1,
  arg2,
  arg3
without
  flag1,
  flag2
```

### Complex module with grouped modifiers (`with`, `without`)

```feather
module complex
  with
    arg1,
    arg2,
    arg3
  without
    flag1,
    flag2,
    flag3
  select
    option1,
    option2
```

## Editor/Formatter Responsibility

- Automatically collapse simple cases into single lines.
- Expand long or complex statements into multiline formatting.
- Maintain alignment of modifiers for readability during auto-formatting.
