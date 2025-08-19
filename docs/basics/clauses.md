# Clauses

Clauses are optional instruction placed at the start of a command to modify the entire command. It's major

The order of clauses is important. This ensures commands are quickly understood, easier to read, and makes it immediately clear if a clause is unused. Clauses always appear at the beginning of a command and must follow the order: `safe`, `elevated`, `sensitive`

## `safe`

Use the `safe` clause when you want instructions to not stop the flow if a function or instruction returns failed result. This modifier is only valid at the beginning and must follow the clause priority order.

### Example 1

This example prints the battery value twice in a row and safely continues if any unsupported OS script fails.

#### Command

```sky
'2' into @count

repeat @count
  ~ Get battery on mac, safe to continue on failure.
  safe script 'pmset -g batt' on mac.

  ~ Get battery on windows, safe to continue on failure.
  safe script 'WMIC Path Win32_Battery Get EstimatedChargeRemaining' on windows.

  ~ Get battery on linux, safe to continue on failure.
  safe script 'acpi -b' on linux.
  wait '5' seconds.
end

success
```

#### Output

```
[WARN][SAFE] Cannot run "safe script 'pmset -g batt' on mac", the operating system is not mac.
[WARN][SAFE] Cannot run "safe script 'WMIC Path Win32_Battery Get EstimatedChargeRemaining'", the operating system is not windows.
[INFO] Battery 0: Discharging, 85%, 02:15:00 remaining
[WARN][SAFE] Cannot run "safe script 'pmset -g batt' on mac", the operating system is not mac.
[WARN][SAFE] Cannot run "safe script 'WMIC Path Win32_Battery Get EstimatedChargeRemaining'", the operating system is not windows.
[INFO] Battery 0: Discharging, 85%, 02:15:00 remaining
[SUCC] The script has been successful.
```

### Example 2

This example shows that safe will print out [SAFE] even on successful command.

#### Command

```sky
~ This script will continue even if the first script fails
safe script 'this_command_does_not_exist'.
safe script 'hello_world.sh'.

say 'This line still runs because of safe'.
success '3'.
```

#### Output

```
[WARN][SAFE] Script "this_command_does_not_exist" has not been found.
[INFO][SAFE] Hello World!
[INFO] This line still runs because of safe.
[SUCC] The script has been successful, Result: '3'.
```

## `sensitive`

See [vault](../vault.md) for more information. Since Feather has an embedded password management system, any time you use a vault variable it will fail unless you explicitly agree to "pass" the variable into the command using the `sensitive` clause.

### Example 1

This example shows what happens if you forget to use `sensitive` with a vault secret in Feather scripting.

#### Command

```sky
sensitive input required 'my_password' into @ephemeral_pw.
```

```sky
sensitive 'Password123$' into vault 'my_password'.

say @{vault 'my_password'} ~ will fail due to using vault without sensitive.
success
```

#### Output

```
[ERRO] Cannot use sensitive vaulted variable "my password".
[FAIL] Error at line 3 of the .sky.
```

### Example 2

This example shows what happens if you use `safe` while sending the vaulted value into an egress (out of feather environment)

#### Command

```sky
'Password123$' into vault 'my_password'

sensitive say @{vault 'my_password'}
success
```

#### Output

```
[INFO][SENS] ***********
[SUCC]
```

### Example 3

This example shows what happens if you really want to use the vault content as egress value (outside of feather environment)

#### Command

```sky
'Password123$' into vault 'my_password'

sensitive say @{vault 'my_password'} with risk
success
```

#### Output

```
[INFO][SENS][RISK] Password123$
[FAIL] Success
```

## `elevated`

Use the `elevated` clause to provide administrative rights to a command. Only use this when necessary, and document why elevation is required for clarity and security.

## Clauses Cluster

Clauses can be grouped into a "cluster" by writing the clauses with no adjacent instruction and ending with an `end`. Clause clusters should only contain plain instructions and must not overlap with other clusters (like `repeat`, `while`, etc.). See (./clusters.md) for more information.

Clause clusters must only wrap plain instructions and may not enclose control structures like `repeat`, `while`, `if`, or `match`.

However, you can apply clauses directly inline with certain flows instructions when supported by the syntax.

See [clusters.md](clusters.md) for more information on how to use cluster.

## Best Practices

- Use clause clusters for blocks of instructions that share the same context (e.g., all should be `safe` and `sensitive`).
- Do not mix clause clusters with flow control blocks (like `repeat`, `if`, etc.)—keep clusters to plain instructions only.
- Use the `sensitive` clause whenever working with vault variables to ensure security.
- Prefer explicit error handling with `safe` for commands that may fail, especially in cross-platform scripts.
- Use `elevated` only when necessary, and always document why administrative rights are required for a command.
- Keep your scripts simple and readable—clarity is more important than cleverness in Feather.
