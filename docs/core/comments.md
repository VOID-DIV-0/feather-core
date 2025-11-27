# General

## Comments

Use the `~` symbol to write a comment. Feather encourages self-documenting code, so comments should be used sparingly, not excessively.

Examples:

```
~ set '3' into @value. ~ this set the value to 3.
```

## Multilines

Feather supports multiline commands using indentation. All commands, whether single or multiline, must end with a dot (.) to mark completion.

```sky
safe sensitive cabinet delete file '/root/very/long/folder' with force as result into ::operation_result silent chrono on linux.
say 'that is a long command!'.
success.
```

Compared to

```sky
safe sensitive cabinet delete file '/root/very/long/folder' with force
                                                            into ::result
                                                            silent chrono
                                                            on linux.
```
