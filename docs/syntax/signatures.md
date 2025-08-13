# Signatures

Signatures are the last instructions of a command. It provides meta logic related to the overall command.

The following signatures are:

1. `on <osfamily>`
2. `trace`
3. `elapsed`
4. `silent`
5. `timeout <seconds> <unit>`

## `on`

`on` is a signature to indicate that the execution will be done only if the machine is from the specified OS family.

Supported instructions: `mac`, `windows`, `linux`.

> _Note_: on <os_family> must be always the last instruction.

### Examples

**Script**

```sky
~ This would be run on a mac PC
say 'I am on linux.' on linux.
say 'I am on mac.' on mac.
say 'I am on windows.' on windows.
success 'This has run on a mac desktop>'.
```

**Output**

```
[INFO] I am on mac.
[SUCC] This has run on mac desktop.
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

```bash
[TRCE] Say 'I am on linux' trace on linux.
[TRCE] Say 'I am on mac' trace on mac.
[INFO] I am on mac
[TRCE] Say 'I am on windows' trace on windows.
[SUCC] This has run on mac desktop.
```

## `elapsed`

The `elapsed` signature measures and reports the time taken to execute a command. This is useful for profiling, benchmarking, or simply tracking performance.

**Script**

```sky
cabinet copy file '/tmp/a.txt' to '/tmp/b.txt' trace elapsed.
success 'File copied.'
```

**Output**

```
[INFO] cabinet copy file '/tmp/a.txt' to '/tmp/b.txt' elapsed.
[TRCE][TIME] Elapsed: 0.023s
[SUCC] File copied.
```

## `timeout`

The `timeout` signature sets a maximum allowed execution time for a command. If the command does not complete within the specified time, it will be interrupted and marked as failed.

**Script**

```sky
cabinet read file '/tmp/large_file.txt' timeout '5' seconds.
success 'File read.'
```

**Output**

```
[INFO] cabinet read file '/tmp/large_file.txt' timeout '5' seconds.
[ERRO] Command timed out after 5 seconds.
```

## `silent`

The `silent` signature suppresses output for the command, except for errors or explicit result statements. This is useful for background operations or when you want to keep logs clean.

**Script**

```sky
cabinet delete file '/tmp/old.log' silent.
success 'Cleanup done.'
```

**Output**

```
[SUCC] Cleanup done.
```

---

**Best Practices:**

- Use `elapsed` for profiling and performance checks.
- Use `timeout` to prevent long-running or stuck operations.
- Use `silent` for background or non-critical commands.
- Combine signatures as needed for robust automation.
