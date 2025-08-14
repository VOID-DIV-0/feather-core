# Signatures

Signatures are special instructions that modify the behavior of a command. They provide meta logic for control, debugging, and reliability.

## Signature Summary

| Signature | Syntax Example        | Effect                                | Notes                                    |
| --------- | --------------------- | ------------------------------------- | ---------------------------------------- |
| `on`      | `on linux`            | Run only on specified OS              | Must be last signature                   |
| `trace`   | `trace`               | Print command before execution        | Can combine with others                  |
| `elapsed` | `elapsed`             | Print execution time                  | Can combine with others                  |
| `timeout` | `timeout '5' seconds` | Limit execution time                  | Supported units: seconds, minutes, hours |
| `silent`  | `silent`              | Suppress output except errors/results | Can combine with others                  |

> **Signature Order:** `on <keyword_os>` must always be the last signature in a command. Other signatures can be combined in any order.

## Supported Values

- **OS keywords for `on`:** `mac`, `windows`, `linux`
- **Time units for `timeout`:** `seconds`, `minutes`, `hours`

## Edge Cases & Interactions

- If `silent` is used, only errors and explicit results are printed.
- If `timeout` and `silent` are combined, timeout errors will still be shown.
- `trace` prints the command before execution, including all signatures and arguments.

---

## `on`

`on` is a signature to indicate that the execution will be done only when the machine is with the specified OS family.

- Supported <keyword_os>: `mac`, `windows`, `linux`.

> _Note_: `on <keyword_os>` must be always the last signature instruction.

### Examples

**Script**

```sky
say 'I am on linux.' on linux.
say 'I am on mac.' on mac.
say 'I am on windows.' on windows.
success 'This script is running on a mac.'.
```

**Output**

```bash
[INFO] I am on mac.
[SUCC] This script is running on a mac.
```

## `trace`

The instruction `trace` forces the printing of the actual command. it's use mostly to investigate a script to make sure everything is going well or just track the command process.

**Script**

```sky
say 'I am on linux.' trace on linux.
say 'I am on mac.' trace on mac.
say 'I am on windows.' trace on windows.
success 'This script is running on a mac.'.
```

**Output**

```bash
[TRCE] Say 'I am on linux.' trace on linux.
[TRCE] Say 'I am on mac.' trace on mac.
[INFO] I am on mac.
[TRCE] Say 'I am on windows.' trace on windows.
[SUCC] This script is running on a mac.
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
[TRCE][INFO] cabinet copy file '/tmp/a.txt' to '/tmp/b.txt' trace elapsed.
[ELAP] Elapsed: 0.023s.
[SUCC] File copied.
```

## `timeout @amount <time_unit_keyword>`

The `timeout` signature sets a maximum allowed execution time for a command. If the command does not complete within the specified time, it will be interrupted and marked as failed.

- Supported <time_unit_keyword>: `seconds`, `minutes`, `hours`.

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

```bash
[SUCC] Cleanup done.
```

---

**Best Practices:**

- Use `elapsed` for profiling and performance checks.
- Use `timeout` to prevent long-running or stuck operations.
- Use `silent` for background or non-critical commands.
- Combine signatures as needed for robust automation.
