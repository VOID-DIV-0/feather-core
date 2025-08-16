# Signatures

## Description

Signatures are instructions that allow you to control command execution, output, and error handling in Feather scripts. They are appended to commands and can be combined for advanced behavior.

---

## Summary Table

| Signature | Syntax Example      | Effect                                | Notes                                    |
| --------- | ------------------- | ------------------------------------- | ---------------------------------------- |
| on        | `on linux`          | Run only on specified OS              | Must be last signature                   |
| trace     | `trace`             | Print command before execution        | Can combine with others                  |
| elapsed   | `elapsed`           | Print execution time                  | Can combine with others                  |
| timeout   | `timeout 5 seconds` | Limit execution time                  | Supported units: seconds, minutes, hours |
| silent    | `silent`            | Suppress output except errors/results | Can combine with others                  |

> [!NOTE]
>
> **Signature Order:** `on <keyword_os>` must always be the last signature in a command. Other signatures can be combined in any order.

---

## Usage

### Syntax

`<command> [signature ...]`

### Instructions

#### `on <keyword_os>`

Restricts command execution to a specific operating system. Only runs the command if the current OS matches the given keyword.

**_Properties_**

- `<keyword_os>`: Represents the os options for the timeout value, can be either `mac` `linux` or `windows`.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

say 'I am on linux.' on linux.
say 'I am on mac.' on mac.
say 'I am on windows.' on windows.
success 'This script is running on a mac.'.
```

```bash
> [INFO] I am on mac.
> [SUCC] This script is running on a mac.
```

---

#### `trace`

Prints the command and its arguments before execution. Useful for debugging and tracking script flow.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

say 'I am on linux.' trace on linux.
say 'I am on mac.' trace on mac.
say 'I am on windows.' trace on windows.
success 'This script is running on a mac.'.
```

```bash
> [TRCE] Say 'I am on linux.' trace on linux.
> [TRCE] Say 'I am on mac.' trace on mac.
> [INFO] I am on mac.
> [TRCE] Say 'I am on windows.' trace on windows.
> [SUCC] This script is running on a mac.
```

---

#### `elapsed`

Measures and reports the time taken to execute a command. Useful for profiling and performance monitoring.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

cabinet copy file '/tmp/a.txt' to '/tmp/b.txt' trace elapsed.
success 'File copied.'
```

```bash
> [TRCE][INFO] cabinet copy file '/tmp/a.txt' to '/tmp/b.txt' trace elapsed.
> [ELAP] Elapsed: 0.023s.
> [SUCC] File copied.
```

---

#### `timeout {value} <keyword_time>`

Sets a maximum allowed execution time for a command. If the command does not complete within the specified time, it will be interrupted and marked as failed.

**_Properties_**

- `value [@|:|numerical]`: Must be a **non-negative integer** or a reference/object projection resolving to one.
- `<keyword_time>`: Represents the unit for the timeout value, can be either `seconds` `minutes` or `hours`.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

cabinet read file '/tmp/large_file.txt' timeout 5 seconds. ~ can also use '5'
success 'File read.'
```

```bash
> [INFO] cabinet read file '/tmp/large_file.txt' timeout 5 seconds. ~ can also use '5'
> [ERRO] Command timed out after 5 seconds.
```

---

#### `silent`

Suppresses output for the command, except for errors or explicit result statements. Useful for background operations or keeping logs clean.

Silent provide a way to reduce the noise of the output by hiding the output when the command is successful.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

cabinet delete file '/tmp/old.log' silent.
success 'Cleanup done.'
```

```bash
> [SUCC] Cleanup done.
```

---

## Edge Cases & Interactions

- If `silent` is used, only errors and explicit results are printed.
- If `timeout` and `silent` are combined, timeout errors will still be shown.
- `trace` prints the command before execution, including all signatures and arguments.

---

## Best Practices

- Use `elapsed` for profiling and performance checks.
- Use `timeout` to prevent long-running or stuck operations.
- Use `silent` for background or non-critical commands.
- Combine signatures as needed for robust automation.

---

## Related Pages

- [Commands **v0.0.1**](../commands-0.0.1.md)
- [Modifiers **v0.0.1**](../modifiers-0.0.1.md)
