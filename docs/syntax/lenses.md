> WIP

# Lenses

In Feather, a **lens** is a temporary interpretation of a value.  
It lets you "look at" a variable or literal in a specific way **for the current instruction only**, without changing the underlying variable.

> **Philosophy:**  
> _A lens is like putting on glasses to see a value differently._  
> The value itself remains the same; the lens only affects the current step.

---

## Key Properties of Lenses

- **Ephemeral**: The effect only applies for this instruction.
- **Non-destructive**: The variable is not permanently changed.
- **Contextual**: Lenses describe _how to interpret_ or _format_ a value right now.

Common lenses:

1. **`as`** – interpret a value temporarily as a type or structure
2. **`format`** – display or output the value in a specific style

---

## `as` Lens

The `as` lens temporarily interprets a literal or variable for the purpose of this instruction.

### Examples

**1. Type interpretation**

```sky
'42' as integer + '3' into @sum
```

- `'42'` is interpreted as an integer for the addition.
- `@sum` still holds a string after the operation.

**2. Structured data access**

```sky
@payload as json 'user.id' into @id
```

- Reads `@payload` as JSON to extract `user.id`.
- `@payload` remains a string afterward.

**3. Stdout / process result lens**

```sky
script 'deploy.sh' into @result as stdresult 'stderr'
```

- Temporarily views the command result through the lens of `stderr`.

---

## `format` Lens

The `format` lens controls how a value is rendered for output or assignment **without changing the underlying value**.

### Examples

```sky
ask @my_number as integer format '000'
```

- Reads user input as an integer, formats with leading zeros for output.

```sky
ask @my_number as decimal format '0.000'
```

- Interprets as a decimal and shows 3 decimal places.

```sky
set safe time now format 'HH:mm:ss' into @today
```

- Gets the current time, formats as `HH:mm:ss`, and stores the string.

---

## Lens Philosophy in Feather

- **Lenses are not assignments.** They do not persist or change the variable's type.
- **Lenses are single-use.** Each instruction can apply a lens, and the effect ends after execution.
- **Lenses are for interpretation, not control flow.** Use clauses (`safe`, `sensitive`) or signatures (`into`, `filter`) for execution behavior.

This distinction keeps Feather scripts **clear, predictable, and readable**.
