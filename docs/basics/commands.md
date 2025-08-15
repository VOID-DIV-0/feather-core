# Commands

## What is a command?

A command in Feather is a sentence-like sequence of instructions composed of clauses, modules, literals, and modifiers, etc. Commands may span single or multiple lines but always end with a dot (.) terminator.

```
[Clause] [Module] [Actions] [Modifiers / Lenses] [Signatures]. -> [Result<[@record | ::Container]>]
```

| Instructions | Example       | Description                    |
| ------------ | ------------- | ------------------------------ |
| Clause       | `safe`        | Modifies behavior or handling  |
| Module       | `cabinet`     | Defines the logical context    |
| Actions      | `delete file` | Action to perform              |
| Modifier     | `with force`  | Enhances or alters behavior    |
| Lens         | `as result`   | Converts or inspects value     |
| Signature    | `on linux`    | Platform, target, or condition |
| Terminator   | `.`           | Required to end any command    |

# Result Pattern

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
