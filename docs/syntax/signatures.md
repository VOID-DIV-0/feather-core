# Signatures

- Control data flow or execution logic across instructions.
- Typically at the end or in fixed positions.
- Tend to be verbs or control structures.

silent, memo, profile

1. `on`
2. `memo`
3. `chronos`

   • Logging tags (telemetry):

## `on`

`on` is a signature to indicate that the execution will be done only if the machine is from the specified OS family.

Supported instructions: `mac`, `windows`, `linux`, `unix`

### Example

#### .sky Script

```sky
say 'I am on linux' on linux.
say 'I am on mac' on mac.
say 'I am on windows' on windows.
say 'I am on BSD' on unix.
success 'This has run on a mac desktop>'.
```

```
~ [INFO] I am on mac
~ [SUCC] This has run on mac desktop.
```

## `memento`

## `chrono`

## timeout

Track the time it took to execute the command.

```

```
