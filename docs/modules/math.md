# Math

## Description

The module Math provides all the necessary logic to operate on record in a mathematical way. Since records are all string, it lenses as integer internally, providing the right logic and validation logic internally.

- `eval`
  - `with`:
    - `rounding`:
- `add`:
- `sub`:
- `mul`:
- `mod`:
- `abs`:
- `round`:
- `floor`:
- `ceil`:
- `clamp`:
- `pow`:
- `random`:

math eval <expr> — arithmetic/compare, precedence, parentheses
• math add|sub|mul|div @a @b into @c
• math mod @a @b into @r
• math abs @n into @out
• math round|floor|ceil @n into @out
• math min|max @a @b into @out
• math clamp @n min '0' max '100' into @out
• math pow @n '2' into @sq
• (optional) math random min '0' max '1' into @r
