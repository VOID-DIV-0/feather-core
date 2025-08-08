# Signatures

Signatures are the last instructions of a command. It provides meta logic related to the overall command.

The following signatures are:

1. `on`
2. `trace`
3. `chronos`
4. `silent`
5. `timeout`

## `on`

`on` is a signature to indicate that the execution will be done only if the machine is from the specified OS family.

Supported instructions: `mac`, `windows`, `linux`.

### Examples

**Script**

```sky
say 'I am on linux.' on linux.
say 'I am on mac.' on mac.
say 'I am on windows.' on windows.
success 'This has run on a mac desktop>'.
```

**Output**

```
~ [INFO] I am on mac.
~ [SUCC] This has run on mac desktop.
```

## `trace`

The instruction `trace` forces the printing of the actual command. it's use mostly to investigate a script to make sure everything is going well or just track the command process.

**Script**

```sky
say 'I am on linux.' trace on linux.
say 'I am on mac.' trace on mac.
say 'I am on windows.' trace on windows.
success 'This has run on a mac desktop>'.
```

**Output**

```
~ [TRCE] Say 'I am on linux' trace on linux.
~ [TRCE] Say 'I am on mac' trace on mac.
~ [INFO] I am on mac
~ [TRCE] Say 'I am on windows' trace on windows.
~ [SUCC] This has run on mac desktop.
```

## `chrono`

## `timeout`

Track the time it took to execute the command.

```

```
