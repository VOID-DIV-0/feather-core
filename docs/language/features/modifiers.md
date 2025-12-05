---
title: Modifiers
slug: modifiers
category: core
status: wip
version: 0.0.1
since: 0.0.1
summary: Command behavior modifiers using with and without keywords.
tags: [modifiers, with, without, options]
---

# Modifiers

## Description

### `with` / `without`

nekonomicon uses readable keywords alongside `with` / `without` so modifiers stay intuitive, memorable, and easy to chain.

Modifier options differ for each module or core component.

- Use `with <option>` to enable an option or specify its value.
- Use `without <option>` to disable an option.
- Repeating the same option (for example `with depth '3'` followed by `with depth '5'`) is a compile-time error.

Examples:

```spell
cabinet file list
  with depth '3'
  with hidden
  without extension
  filter '[A-Za-z]'.
```

This replaces obscure flags with self-explanatory modifiers. You can also chain them inline:

```spell
cabinet file list with depth '3' with hidden without extension filter '[A-Za-z]'
```
