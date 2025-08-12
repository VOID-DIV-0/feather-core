![logo](./~.png)

# What is Feather?

Feather is a lightweight scripting and automation language with a built-in interactive mode called Sky.
It is designed to be embedded in applications or used as a standalone, elegant, and fast scripting interface. It is written in Rust.

# Why Feather?

Feather's strength lies in its core architectural principles and design choices that emphasize clarity, safety, and composability. The language enforces strong type safety and includes secure-by-default features that protect users from common scripting pitfalls. Its readability-first rules make scripts accessible and maintainable, even for those unfamiliar with Feather.

While this design approach makes Feather scripts harder to write quickly, it makes them much easier to read, understand, and maintain over time. This balance ensures that Feather is a reliable tool for automation and scripting tasks, fostering confidence and reducing errors.

Key principles behind Feather's design include:

- Clear and explicit logic that prioritizes understanding over brevity
- Strong type safety to catch errors early
- Secure-by-default operations to enhance script safety
- Composability that encourages building complex workflows from simple, understandable parts
- Readability-first rules that make scripts approachable for all users

# Goals

To create an elegant, fast and reliable library and REPL to make scripting easy, intuitive and simple. Even if feather might not be able to do anything, the goal is to make everything in the best possible way

# Demo

```
cabinet read file 'user_config.json' as json into ::config.

~ Extract username and password
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
vault lock 'user' with value ::config:content:user.
vault lock 'pwd' with value ::config:content:pass.
vault lock 'secret' with env 'SYS_SECRET'
vault lock 'admin' with env 'MACHINE_ADMIN'.

~ Validate credentials
~~~~~~~~~~~~~~~~~~~~~~~~~
sensitive text lower (vault unlock 'user') into @username_lc.
sensitive text lower (vault unlock 'admin') into @admin_lc.
sensitive text compare @username_lc is @admin_lc without case into @is_valid_user.
sensitive text compare @password is (vault unlock 'secret') into @is_valid_password.

time now format 'YYYY-MM-DD HH:mm:ss' into @timestamp.

if bool eval @is_valid_user and @is_valid_password
  sensitive say 'Login successful for @{username} at @{timestamp}.'
  ~ Backup config
  ~~~~~~~~~~~~~~~~~~~~
  cabinet copy file 'user_config.json' to 'backups/user_config_@{timestamp}.json'.

  ~ Log success
  ~~~~~~~~~~~~~~~~
  sensitive text concat 'SUCCESS: ' @timestamp ' User: ' @username into @log_entry.
  cabinet append file 'login.log' with @log_entry.
  success 'User authenticated and config backed up.'
else
  sensitive say 'Login failed for @{username} at @{timestamp}.'

  ~ Log failure
  ~~~~~~~~~~~~~~~~
  sensitive text concat 'FAILURE: ' @timestamp ' User: ' @username into @log_entry.
  cabinet append file 'login.log' with @log_entry.

  ~ Send notification
  ~~~~~~~~~~~~~~~~~~~~~~~~
  http post 'https://notify.example.com' with { 'user': @username, 'status': 'failed', 'time': @timestamp } into @notify_result.
  failure 'Authentication failed.'
end
```

# Performance Philosophy

Feather is not designed to compete with low-level scripting engines or compiled languages in raw speed. Instead, it prioritizes clarity, safety, and composability — values that matter most in real-world automation and tooling workflows.

We deliberately accept a slight performance tradeoff to provide:

- Explicit and readable logic
- Secure-by-default operations (`vault`, `safe`, `elevated`)
- Predictable behavior with minimal surprises
- Easier onboarding for non-programmers and junior developers

Feather's goal is not to win benchmarks. And at the end of the day, when you need raw control or edge-case performance, you can always fall back to native script execution using the script module.

# Golden Rules

Feather is designed around a few core principles that shape every part of its syntax and behavior:

1. **Minimal symbols** — use as few special characters as possible. The goal is readability, not cryptic shorthand.
2. **English-like syntax** — commands should be as close to plain English as possible.
3. **Elegant, simple flow** — scripts should be easy to follow, but we do not cut corners for the sake of brevity.
4. **Cross-platform by default** — a single script should work on any machine without modification.
5. **Clarity and speed of reading** — code should be clear and effortless to understand at a glance.
6. **Consistency over conciseness** — prefer consistent patterns even if they are slightly longer to write.
7. **Composability over complexity** — build small, understandable instructions that can be combined rather than overly complex one-liners.

Feather should feel intuitive: you can pick it up without memorizing a manual. It is designed to solve 80% of everyday tasks cleanly. When it cannot, you can gracefully fall back to native `script` commands without breaking your flow.

# Documentation

The documentation is available in the docs folder. see Feather [docs](./docs/index.md)

# Contributions

Contributions are welcome!
All contributions will be reviewed and must be approved by the project maintainer before being merged.
