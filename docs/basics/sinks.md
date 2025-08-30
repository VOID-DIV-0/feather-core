# Sinks

## `<instructions> into [@record | ::container | ::container:projection]`

→ create; error if exists

## `<instructions> update [@record | ::container | ::container:projection]`

→ replace wholesale; error if missing (or allow creating—decide)

## `<instructions> save [@record | ::container | ::container:projection]`

permanent immutable sinked receiver.

---

**Note:** Higher-level behaviors such as `record ensure` and `container merge` are handled within their respective modules. Sinking data is always performed using `into` or `into overwrite`.
