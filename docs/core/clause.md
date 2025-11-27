---
title: Clause
slug: clause
category: core
status: stable
version: 0.1.0
since: 0.1.0
summary: Control error handling, parallelism, security, and permissions for commands using readable, explicit clauses.
tags: [clause, flow, error, async, safe, sensitive, elevated]
---

# Clause

## Description

Clauses are optional instructions placed at the start of a command. They alter the behavior of the targeted command until the terminator. Clauses let you control error handling, parallelism, security, and permissions for each command in a simple, readable way. Clauses can be applied to both built-in commands and user-defined functions.

Clauses let you control error handling, parallelism, security, and permissions for each command in a simple, readable way.

Clauses can be applied to both built-in commands and user-defined functions.

## Summary Table

| Clause        | Minimal Syntax           | Effect                                              | Notes                                           |
| ------------- | ------------------------ | --------------------------------------------------- | ----------------------------------------------- |
| safe          | safe <command>.          | Continue on error, do not halt flow                 | -                                               |
| async         | async <command>.         | Run command in parallel                             | can use literal for tagging (i.e.: async 'tag') |
| sensitive     | sensitive <command>.     | Allow/taint variables as sensitive/!!!              | -                                               |
| !!! sensitive | !!! sensitive <command>. | Allow use of sensitive variables for egress context | will fail at runtime                            |
| elevated      | elevated <command>.      | Run with admin rights                               | -                                               |

## Anatomy

Clause <command> [modifiers] [signatures]. Clauses must always be at the beginning of a command. Multiple clauses can be grouped in a cluster (see below). There is no order requirement when using multiple clauses, but it is recommended to keep a consistent order for readability.

The suggested order is:

1. sensitive / !!! sensitive
2. safe
3. async
4. elevated

## Syntax

```sky
safe <command>.
async <command>.
async 'tag' <command>.
sensitive <command>.
!!! sensitive <command>.
elevated <command>.
```

---

## Clause `safe`

### 1. Safe Clause (Cross-platform battery check)

Here's an example of using the `safe` clause to check battery status across different operating systems without halting the script if a command fails:

```sky
2 into !@count. ~ sealed

~ Would print battery status 2 times depending on OS. All failures are ignored.
repeat @count
  safe script 'pmset -g batt' on mac.
  safe script 'WMIC Path Win32_Battery Get EstimatedChargeRemaining' on windows.
  safe script 'acpi -b' on linux.
  wait '5' seconds.
end

success.
```

---

## Clause `async`

### 2. Async Clause

Here is an example of using the `async` clause to run two long-running tasks in parallel, the wait command is used to wait all currently running async tasks to complete before proceeding:

```sky
async script 'long_running_task_1.sh'.
async script 'long_running_task_2.sh'.
say 'Both tasks started asynchronously.'.
wait. ~ Waits for all async tasks to finish
success.
```

Here's an example of using the `async` clause to run commands in parallel. This example demonstrates fetching data from two different URLs simultaneously:

```sky
function fn_world
  wait 5 seconds.
  say 'world'.
end
async 'world' fn_world.
say 'hello'.
wait 'world'.
success.
```

### 4. Async with wait all/any

```sky
async 'read file 1' cabinet read file 'my_file1.txt'.
async 'read file 2' cabinet read file 'my_file2.txt'.
wait all 'read file 1' 'read file 2'.
success.
```

```sky
async 'read file 1' cabinet read file 'my_file1.txt'.
async 'read file 2' cabinet read file 'my_file2.txt'.
wait any 'read file 1' 'read file 2'.
stop
success.
```

---

## Clause `sensitive` and `!!! sensitive`

### What is egress context?

Egress context refers to any situation where sensitive data could potentially leave the secure environment of the Feather script and become observable or accessible outside of it. This includes, but is not limited to:

- Outputting to the console (e.g., printing, logging)
- Writing to files (including temp files, logs, or exports)
- Sending over the network (HTTP requests, sockets, etc.)
- Injecting into or calling other modules (if the module is not marked as sensitive-aware)
- Passing to external scripts or programs (e.g., script, exec)
- Storing in environment variables or other process-wide state
- Any other action that makes data observable outside the current secure context

Egress-friendly commands are explicitly designed to handle sensitive data without risking exposure. These commands have built-in safeguards to ensure that sensitive information is never logged, displayed, or transmitted insecurely. In Feather, a command is only considered egress-friendly if it is clearly marked as sensitive-aware in its documentation.
By default, all commands are treated as egress-hostile, which is unsafe for handling sensitive data in an egress context—unless they are explicitly documented as egress-friendly.

#### Why use the sensitive clause?

The `sensitive` clause is used to explicitly mark commands that handle sensitive data, allowing the use of vaulted variables and ensuring that such data is treated with the necessary security precautions. It helps prevent accidental exposure of sensitive information by enforcing restrictions on how and where this data can be used.

```sky
sensitive ask 'What is my password?' with mask into @ephemeral_pw.
~ This is marked
!!! sensitive say 'You entered a password of length @{length of @ephemeral_pw} characters.'.
success.
```

```sky
'Password123$' into vault 'my_password'.
sensitive say @{vault 'my_password'}.
success.
```

```sky
'Password123$' into vault 'my_password'.
sensitive say @{vault 'my_password'} with risk.
success.
```

### 6. Elevated Clause

```sky
elevated script 'apt update'.
```

## Edge Cases

- Using `safe` on a command that always fails: script continues, logs warning.
- Using `sensitive` without vault: error if variable is not vaulted.
- Using `elevated` in unelevated context: script fails.
- Clause clusters must not wrap flow control blocks (repeat, if, etc.).

## Best Practices

- Use clause clusters for blocks of instructions that share the same context (e.g., all should be `safe` and `sensitive`).
- Do not mix clause clusters with flow control blocks—keep clusters to plain instructions only.
- Use the `sensitive` clause whenever working with vault variables to ensure security.
- Prefer explicit error handling with `safe` for commands that may fail, especially in cross-platform scripts.
- Use `elevated` only when necessary, and always document why administrative rights are required for a command.
- Keep your scripts simple and readable—clarity is more important than cleverness in Feather.

## Common Errors

- Using a vault variable without `sensitive`: `[ERRO] Cannot use sensitive vaulted variable ...`
- Using `elevated` without permissions: script fails.
- Wrapping flow control blocks in clause clusters: syntax error.

## Related Pages

- [Safe Clause](./clause.md)
- [Vault Module](../modules/vault-0.0.1.md)
- [Clusters](./clusters.md)
- [Record core](../core/)
- [Sinks](./sinks.md)
