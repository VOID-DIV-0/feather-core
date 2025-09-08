# Sinks

## `<instructions> into [@record | ::container | ::container:projection]`

→ create or modify bindings; error if exists

The `into` sink is used to create new or modify existing bindings within a record or container. It is useful when you want to add or update data.

**Example:**

```
'foo' into @foo.
'foo' into @bar.

'bar' into @bar. ~override @bar foo -> bar

say @foo, @bar.
```

## `<instructions> as [@record | ::container]`

→ create an immutable snapshot (seal)

The `as` sink creates a sealed, immutable snapshot of the data at the time of sinking. This is useful for capturing a fixed state that should not change.

**Example:**

``

## `<instructions> is [&schema]`

→ validate or assert data

The `is` sink is used for validation or assertion purposes, ensuring that data conforms to expected values or conditions.

**Example:**

```
is @settings {
  debug: false,
  maxRetries: 5
}
```

---

**Note:** Higher-level behaviors such as `record ensure` and `container merge` are handled within their respective modules. Sinking data is always performed using `into`, `as`, or `is` sinks.
