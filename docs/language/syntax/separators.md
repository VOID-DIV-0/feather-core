---
title: Separators
slug: separators
category: core
status: wip
version: 0.0.1
since: 0.0.1
summary: Command and statement separators.
tags: [separators, syntax, punctuation]
---

# Separators in nekonomicon

## Argument Separators

In nekonomicon, the comma `,` is used as an argument separator to distinguish multiple arguments within a single command. This helps the interpreter understand where one argument ends and another begins.

For example:

```
say 'a', 'b', 'c'.
text combine 'a', 'b', 'c'.
json read 'my_file' with tabulation, formatted without eof.
```

In these examples, the comma clearly separates each argument passed to the command.

Separators are optional only when a command naturally takes a single argument. However, when multiple arguments are passed, using commas is required for clarity and correct parsing.

Note that the term "separator" here refers specifically to the role of the comma in splitting arguments, and does not refer to line breaks or other syntax features.
