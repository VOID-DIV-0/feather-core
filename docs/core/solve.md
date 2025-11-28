---
title: Solve
slug: solve
category: core
status: stable
version: 0.1.0
summary: Arithmetic and comparison intrinsic for numeric expressions.
tags: [arithmetic, comparison, intrinsic]
---

# Solve

## Description

`solve` is nekonomicon's intrinsic for evaluating arithmetic expressions and performing comparisons. It is used to compute numeric results or to compare values, producing either a numeric record or a boolean record. All logic and string operations are handled elsewhere—`solve` is strictly for numbers and comparisons.

---

## Philosophy

nekonomicon separates arithmetic and comparison from logic and string manipulation. Use `solve` for all numeric calculations and direct comparisons. Use `decide` for boolean logic, and the `text` module for any string operations.

- **Explicit:** Only numeric operations and comparisons are allowed.
- **Minimal:** No string concatenation, no boolean logic, no side effects.
- **Readable:** Each `solve` line does one thing—compute a number or compare values.

---

## Syntax

```
solve <arithmetic_expr> into <@result>.
solve <comparison_expr> into <@result>.
```

Where:

- `<arithmetic_expr>` uses numbers, records, and arithmetic operators: `+ - * / %`
- `<comparison_expr>` uses numbers, records, and comparison operators: `> < >= <= == !=`

No parentheses are supported. For complex calculations, break into multiple lines.

---

## Supported Operators

| Operator | Meaning          | Example    |
| -------- | ---------------- | ---------- |
| +        | Addition         | `@a + @b`  |
| -        | Subtraction      | `@a - @b`  |
| \*       | Multiplication   | `@a * @b`  |
| /        | Division         | `@a / @b`  |
| %        | Modulus          | `@a % @b`  |
| >        | Greater than     | `@a > @b`  |
| <        | Less than        | `@a < @b`  |
| >=       | Greater or equal | `@a >= @b` |
| <=       | Less or equal    | `@a <= @b` |
| ==       | Equality         | `@a == @b` |
| !=       | Inequality       | `@a != @b` |

---

## Type Rules

- All operands must be numeric (unquoted numbers or records containing numbers).
- Comparisons may be performed between numbers or numeric records.
- Mixing non-numeric types results in an error (`E-SOLVE-NONNUM`).
- Division by zero results in an error (`E-SOLVE-DIVZERO`).
- Only one comparison operator is allowed per `solve` line.
- No boolean logic (`and`, `or`, `not`) is allowed—use `decide` for logic.

---

## Examples

### Arithmetic

```spell
solve 5 + 3 into @sum.              ~ 8
solve @price * @qty into @total.
solve @balance - @cost into @remain.
solve 10 % 3 into @mod.             ~ 1
solve 100 / 4 into @quarter.        ~ 25
```

### Comparison

```spell
solve @a > @b into @is_greater.
solve @score == 100 into @perfect.
solve @x + @y <= 10 into @within_limit.
```

### Chaining Calculations

```spell
solve @a + @b into @sum.
solve @sum * 2 into @double.
solve @double == 20 into @is_twenty.
```

---

## Best Practices

1. **Keep each calculation simple:** Break complex expressions into multiple lines.
2. **Use only numeric operands:** Cast or convert values before using them in `solve`.
3. **Perform comparisons in `solve`, logic in `decide`:** Do not use boolean operators in `solve`.
4. **Handle errors explicitly:** Check for division by zero and type mismatches.

---

## Error Handling

| Error Code           | Description                                     |
| -------------------- | ----------------------------------------------- |
| E-SOLVE-NONNUM       | Non-numeric operand in arithmetic               |
| E-SOLVE-DIVZERO      | Division by zero                                |
| E-SOLVE-CHAIN        | Multiple comparison operators in one expression |
| E-SOLVE-BADOP        | Unsupported operator                            |
| E-SOLVE-TYPEMISMATCH | Incompatible types in comparison                |

---

## Related Pages

- [Decide (boolean logic)](./decide.md)
- [Math Module (advanced math)](../modules/math.md)
- [Flow Control](./flow.md)
- [Data Types](./data.md)

---

## Version

- 0.1.0 — Arithmetic and comparison only; all string and logic operations removed.

---
