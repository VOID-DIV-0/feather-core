> WIP

## Modifiers

> **Note:** Use `into` with `set` to assign values. Parentheses should be reserved for grouping complex logic only, not for simple assignments.

### `with` / `without`

Feather eliminates cryptic flags like `-d`, `-ls`, or `--help`. Instead, you can chain readable `with` and `without` modifiers.

- Use `with <option>` to enable an option or specify its value.
- Use `without <option>` to disable an option.

These can be chained for clarity and read like English.

Examples:

```sky
cabinet file list
  with depth '3'
  with hidden
  without extension
  filter '[A-z]'.
```

This replaces obscure flags with self-explanatory modifiers. You can also chain them inline:

```sky
cabinet file list with depth '3' with hidden without extension filter '[A-z]'
```

