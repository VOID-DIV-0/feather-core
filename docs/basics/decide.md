---
title: Decide
slug: decide
category: basics
status: stable
version: 0.1.0
summary: Boolean logic composition and decision-making intrinsic.
tags: [boolean, logic, decision, intrinsic]
---

# Decide

## Description

`decide` is Feather's intrinsic for composing boolean logic and making decisions based on previously computed facts. It enables clear, English-like expression of logical conditions, combining boolean records and relational comparisons into a single, readable statement.

Unlike arithmetic (`solve`), which computes numeric results, `decide` is used exclusively for evaluating truth—answering "should this action proceed?" or "does this condition hold?" in your scripts.

---

## Philosophy

Feather treats boolean logic as a distinct phase of reasoning. Arithmetic and comparisons are handled with `solve`, while all logical composition—AND, OR, NOT, XOR, and multi-branch conditions—are handled with `decide`. This separation keeps scripts explicit, readable, and easy to audit.

- **Explicit:** No hidden type coercion or implicit truthiness. Only boolean records (`'true'` or `'false'`) and relational comparisons are allowed.
- **English-like:** Reads naturally—`decide @a and not @b into @ok.`
- **Minimal:** No operator precedence confusion; logic is composed left-to-right.

---

## Syntax

```
decide <boolean_expr> into <@result>.
```

Where `<boolean_expr>` is:
- A boolean record (`@flag`)
- A relational comparison (`@a > @b`, `@x == 'foo'`)
- Logical operators: `and`, `or`, `xor`
- Unary negation: `not`
- Any combination of the above, composed left-to-right

**No parentheses or nested expressions.**  
For complex logic, break into multiple lines for clarity.

---

## Supported Operators

| Operator | Meaning                | Example                        |
|----------|------------------------|--------------------------------|
| and      | Logical AND            | `@a and @b`                    |
| or       | Logical OR             | `@a or @b`                     |
| xor      | Logical XOR            | `@a xor @b`                    |
| not      | Logical NOT (unary)    | `not @a`                       |
| ==, !=   | Equality, inequality   | `@x == @y`, `@flag != 'true'`  |
| >, <, >=, <= | Numeric comparison | `@n > 5`, `@score <= 100`      |

---

## Examples

### 1. Basic Boolean Composition

```sky
solve @age > 18 into @is_adult.
solve @country == 'US' into @is_domestic.

decide @is_adult and @is_domestic into @eligible.
```

### 2. Multi-Branch Logic

```sky
solve @balance > 0 into @has_funds.
solve @status == 'active' into @active.

decide @has_funds and @active or @override into @can_proceed.
```

### 3. Using NOT and XOR

```sky
solve @score >= 90 into @high_score.
solve @bonus == 'yes' into @has_bonus.

bool xor @high_score @has_bonus into @has_either.
decide not @blocked and @has_either into @award.
```

### 4. Chaining for Readability

```sky
solve @x > 0 into @pos.
solve @y > 0 into @pos_y.

decide @pos and @pos_y into @both_positive.
```

### 5. String Equality

```sky
solve @user == 'admin' into @is_admin.
decide @is_admin and not @suspended into @can_access.
```

---

## Best Practices

1. **Compute facts first:** Use `solve` for arithmetic and comparisons, then combine with `decide`.
2. **Keep logic flat:** Avoid deeply nested or overly long `decide` lines. Break into steps for clarity.
3. **Be explicit:** Only use boolean records or relational comparisons as operands.
4. **No implicit truthiness:** Strings, numbers, or containers are not automatically treated as booleans.
5. **Left-to-right evaluation:** Logic is evaluated in order; no operator precedence. For complex conditions, use intermediate variables.

---

## Error Handling

| Error Code           | Description                                      |
|----------------------|--------------------------------------------------|
| E-DECIDE-NONBOOL     | Operand is not a boolean or comparison           |
| E-DECIDE-TYPEMISMATCH| Incompatible types in comparison                 |
| E-DECIDE-CHAIN       | Improper chaining of relational operators        |
| E-DECIDE-EMPTY       | No operands supplied                             |
| E-DECIDE-OPUNKNOWN   | Unknown logical operator                         |

---

## Related Pages

- [Solve (arithmetic & comparison)](./solve.md)
- [Flow Control](./flow.md)
- [Bool Module (advanced logic)](../modules/bool.md)
- [Data Types](./data.md)

---

## Version

- 0.1.0 — Initial version

---
