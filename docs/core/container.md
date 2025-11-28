## Multilines

nekonomicon supports multiline commands using indentation. All commands, whether single or multiline, must end with a dot (.) to mark completion.

```spell
safe sensitive cabinet delete file '/root/very/long/folder' with force as result into ::operation_result silent chrono on linux.
say 'that is a long command!'.
success.
```

Compared to

```spell
safe sensitive cabinet delete file '/root/very/long/folder' with force
                                                            into ::result
                                                            silent chrono
                                                            on linux.
```
