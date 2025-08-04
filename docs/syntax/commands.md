# Commands

## What is a command?

A command in Feather is a sentence-like instruction composed of clauses, modules, literals, and modifiers. Commands may span single or multiple lines but always end with a dot (.) terminator.

```
[Clause] [Module] [Actions] [Modifiers / Lenses] [Signatures] -> Result
```

| Part       | Example       | Description                    |
| ---------- | ------------- | ------------------------------ |
| Clause     | `safe`        | Modifies behavior or handling  |
| Module     | `cabinet`     | Defines the logical context    |
| Actions    | `delete file` | Action to perform              |
| Modifier   | `with force`  | Enhances or alters behavior    |
| Lens       | `as result`   | Converts or inspects value     |
| Signature  | `on linux`    | Platform, target, or condition |
| Terminator | `.`           | Required to end any command    |

# Result Pattern

Feather scripts must explicitly declare their outcome. This keeps scripts predictable and self-documenting.

- Use `success` to mark the script as completed successfully.
- Use `failure` to mark the script as failed.

Both instructions can optionally include a message or variable:

```
success.
success 'All good'.
success @message.

failure.
failure 'Something went wrong'.
failure @error_info.
```

If a Feather script reaches the end **without calling `success`**, it will automatically be marked as a failure. This enforces explicit intent and ensures that scripts never exit silently.

The `safe` modifier integrates with this pattern: any instructions of a command marked `safe` will not stop the script if it fails.

The result pattern is also applicable to [functions](./syntax/functions.md).

## Comments

Use the `~` symbol to write a comment. Feather encourages self-documenting code, so comments should be used sparingly, not excessively.

Examples:

```
~ set '3' into @value. ~ this set the value to 3.
```

## Multilines

Feather supports multiline commands using indentation. All commands, whether single or multiline, must end with a dot (.) to mark completion.

```sky
~ safe sensitive cabinet delete file '/root/very/long/folder' with force as result into ::operation_result silent chrono on linux.
~ say 'that is a long command!'.
~ success.
```

Compared to

```sky
~ safe sensitive cabinet delete file '/root/very/long/folder' with force
~                                                             as result
~                                                             into ::operation_result
~                                                             silent chrono
~                                                             on linux.
```
