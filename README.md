![logo](./~.png)

# What is Feather?

Feather is a lightweight scripting and automation language with a built-in interactive mode called Sky.
It is designed to be embedded in applications or used as a standalone, elegant, and fast scripting interface. It is written in Rust.

# Goals

To create an elegant, fast and reliable library and REPL to make scripting easy, intuitive and simple. Even if feather might not be able to do anything, the goal is to make everything in the best possible way

# Syntax Philosophy

Feather is built on a simple idea:

> Scripting shouldn’t feel like deciphering a puzzle. It should feels as if you are explaining steps to a machine.

We prioritize:

- Clarity over cleverness
- Consistency over conciseness
- Composability over complexity

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
