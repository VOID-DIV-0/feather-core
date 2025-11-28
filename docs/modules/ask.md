# ask

The `ask` module provides interactive prompts for user input. It allows you to request input from users, optionally providing a default value, validating input against a schema, or hiding input (e.g., for passwords).

---

## Sink Rules

- **Sink:** `ask` always produces a single value from the user's input.
- **Type:** The output type is determined by the input or by validation schema (e.g., string, number, boolean).
- **Blocking:** Prompts block execution until user input is received.

---

## Signatures

```nekonomicon
ask(prompt: string) -> string
ask(prompt: string, default: any) -> any
ask(prompt: string, schema: Schema) -> any
ask(prompt: string, { hidden: true }) -> string
```

- **prompt:** The message displayed to the user.
- **default:** (Optional) Value to use if the user presses enter without input.
- **schema:** (Optional) Validation schema (e.g., `"number"`, `"boolean"`, regex, or custom object) to enforce input type or format.
- **options:** (Optional) Object with flags, e.g., `{ hidden: true }` to hide input.

---

## Examples

### 1. Simple Prompt

```nekonomicon
let name = ask("What is your name?")
say("Hello, " + name + "!")
```

### 2. Prompt with Default Value

```nekonomicon
let color = ask("Favorite color?", "blue")
say("You chose: " + color)
```

### 3. Prompt with Schema Validation

```nekonomicon
let age = ask("Enter your age:", "number")
say("You are " + age + " years old.")
```

#### With Boolean Schema

```nekonomicon
let subscribe = ask("Subscribe to newsletter?", "boolean")
if subscribe {
  say("Thank you for subscribing!")
}
```

#### With Regex Schema

```nekonomicon
let email = ask("Enter your email:", /.+@.+\..+/)
say("Email recorded: " + email)
```

### 4. Hidden Input (Password)

```nekonomicon
let pw = ask("Enter your password:", { hidden: true })
say("Password accepted.")
```

---

## Notes

- If a `default` is provided and the user submits empty input, the default is returned.
- If a `schema` is provided, the input is validated and (if possible) coerced to the schema type. Invalid input will re-prompt the user.
- When `{ hidden: true }` is used, input is not shown on the screen (useful for sensitive data).
- Only one of `default`, `schema`, or `options` may be provided as the second argument. For combined schema/options, use an options object:
  ```nekonomicon
  let pin = ask("Enter 4-digit PIN:", { schema: /^\d{4}$/, hidden: true })
  ```
- The prompt string should be concise and clear.

---
