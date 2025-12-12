---
title: Invoke
slug: invoke
category: core
status: stable
version: 0.1.0
since: 0.0.1
summary: Load external modules into script scope for use.
tags: [invoke, modules, dependencies, summon]
---

# Invoke

## Description

The `invoke` instruction loads external modules into your script's scope, making their functionality available for use. Modules must be summoned (installed) before they can be invoked.

By default, **no modules are loaded**—you must explicitly invoke each module you need. This ensures scripts only have access to declared dependencies, improving security, auditability, and portability.

**Key concepts:**

- **`summon`** — Install a module globally (via CLI: `neko summon <module>`)
- **`invoke`** — Load a summoned module into the current script

This separation allows:

- **Per-script dependency control** — Each spell declares exactly what it needs
- **Clean global environment** — Summoned modules don't pollute all scripts
- **Easy auditing** — `invoke` statements show exactly what capabilities a script uses
- **Security** — Modules with sensitive capabilities (file I/O, network, system) are opt-in

---

## Summary Table

| Pattern                 | Syntax             | Effect                         | Notes                        |
| ----------------------- | ------------------ | ------------------------------ | ---------------------------- |
| Invoke single module    | `invoke module.`   | Load module into script scope  | Must be summoned first       |
| Invoke multiple modules | `invoke m1 m2 m3.` | Load multiple modules at once  | Space-separated module names |
| Check if summoned       | N/A                | Automatic validation on invoke | Error if module not summoned |

---

## Syntax

### Basic Invoke

```spell
invoke cabinet.
```

Loads the `cabinet` module, making its commands available.

### Multiple Modules

```spell
invoke cabinet vault network.
```

Loads multiple modules in a single statement.

---

## Examples

### 1. Simple Module Invocation

```spell
invoke cabinet.

cabinet read file 'config.txt' into @config.
say 'Config loaded: ' @config.

success.
```

### 2. Module Not Summoned (Error)

```spell
invoke cabinet.    ~ ❌ ERROR: Module 'cabinet' not summoned. Run: neko summon cabinet
```

### 3. Multiple Modules

```spell
invoke cabinet vault text.

vault lock 'api_key' with 'secret123'.
cabinet read file 'data.txt' into @data.
text length @data into @length.

say 'Read ' @length ' characters from file.'.

success.
```

### 4. Conditional Module Usage

```spell
invoke cabinet.

decide @enable_logging equals true into @should_log.

if @should_log
  cabinet write file 'app.log' with 'Application started.'.
end

success.
```

### 5. Docker Environment Setup

Here's how to summon modules globally in a Docker container and invoke them in scripts:

```dockerfile
FROM nekonomicon/nekonomicon:latest

# Summon modules globally
RUN neko summon cabinet
RUN neko summon vault
RUN neko summon network

# Run spell that invokes modules
RUN neko conjure my_script.spell
```

```spell
~~~
    Script: my_script.spell
    Requires: cabinet, vault modules
~~~

invoke cabinet vault.

vault lock 'api_key' with 'secret123'.
cabinet read file 'config.txt' into @config.

say 'Configuration loaded successfully.'.
success.
```

---

## Module Availability

nekonomicon ships with **no modules loaded by default**. Core modules include:

- `cabinet` — File and directory operations
- `vault` — Secure secret storage
- `network` — HTTP requests and network operations
- `text` — String manipulation
- `math` — Mathematical operations
- `system` — System information and process management

**To use any module:**

1. Summon it: `neko summon <module>`
2. Invoke it in your spell: `invoke <module>.`

---

## Best Practices

1. **Invoke at the top of scripts:** Place `invoke` statements at the beginning for clarity

   ```spell
   ~~~ Dependencies ~~~
   invoke cabinet vault network.

   ~~~ Main script ~~~
   cabinet read file 'data.txt' into @data.
   ```

2. **Document required modules:** Use comments to explain why modules are needed

   ```spell
   invoke cabinet.    ~ Required for reading configuration files
   invoke vault.      ~ Required for API key storage
   ```

3. **Minimize dependencies:** Only invoke modules you actually use

4. **Fail fast:** Let scripts error early if modules aren't summoned

5. **Use in CI/CD:** Document summoned modules in deployment scripts or Dockerfiles

---

## Platform Considerations

- **Module availability:** Some modules may be platform-specific (e.g., Windows-only, Linux-only)
- **Summon once, invoke many:** Summoned modules are available to all scripts on that system
- **No auto-loading:** Modules are never implicitly loaded—always explicit via `invoke`

---

## Error Handling

| Error Code            | Description                   | Solution                            |
| --------------------- | ----------------------------- | ----------------------------------- |
| E-INVK-NOT-SUMMONED   | Module not installed          | Run `neko summon <module>`          |
| E-INVK-INVALID-MODULE | Module name doesn't exist     | Check spelling or available modules |
| E-INVK-DUPLICATE      | Module already invoked        | Remove duplicate `invoke` statement |
| E-INVK-CONFLICT       | Module conflicts with another | Check module documentation          |

---

## Related Pages

- [Clause](./clause.md) — Command modifiers like `safe`, `async`, `sensitive`
- [Cabinet Module](../modules/cabinet.md) — File I/O operations
- [Vault Module](../modules/vault.md) — Secure secret management
- [CLI Usage](../reference/cli-usage.md) — `neko summon` and other CLI commands

---

## Version History

- **0.1.0** — Documentation overhaul following standard format
- **0.0.1** — Initial invoke/summon implementation
