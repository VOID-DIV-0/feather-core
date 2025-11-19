# Expression Keywords

Feather provides three domain-specific keywords for handling expressions: `solve` for mathematics, `eval` for logic, and `text` for string operations.

---

## `solve` - Mathematical Expressions

**Purpose:** Handle arithmetic and numerical comparisons.

### Operators

- `+`, `-`, `*`, `/`, `%` - Basic arithmetic
- `>`, `<`, `>=`, `<=` - Numerical comparisons
- `==`, `!=` - Equality/inequality

### Examples

```sky
solve 5 + 3 into @sum.                    ~ 8
solve @price * @quantity into @total.     ~ Multiplication
solve @balance - @cost into @remaining.   ~ Subtraction
solve @age >= 18 into @is_adult.          ~ true/false
solve @a == @b into @same.                ~ Equality check
```

### Type Rules

- All operands must be numbers (or convertible to numbers)
- Unquoted literals are treated as numbers: `5`, `3.14`, `-10`
- Variables containing string numbers are auto-converted: `'5'` becomes `5`

---

## `eval` - Logical Expressions

**Purpose:** Handle boolean logic and conditions.

### Operators

- `and`, `or`, `xor` - Logical operations
- `not` - Logical negation
- `==`, `!=` - Boolean equality

### Examples

```sky
eval true and false into @result.         ~ false
eval @is_valid or @is_backup into @ok.    ~ Boolean OR
eval not @failed into @success.           ~ Negation
eval @logged_in and @has_permission into @can_access.
```

### Type Rules

- All operands must be booleans: `true`, `false`
- Variables are evaluated as booleans: `0`/`''` = false, everything else = true

---

## `text` - String Operations

**Purpose:** Handle string concatenation and manipulation.

### Operators

- `+` - String concatenation
- `==`, `!=` - String equality

### Examples

```sky
text 'Hello' + ' world' into @greeting.   ~ 'Hello world'
text @name + ': ' + @message into @log.   ~ Name: message format
text 'User ' + @id + ' logged in' into @event.
text @first == @second into @same_text.   ~ String comparison
```

### Type Rules

- All operands must be strings
- Quoted literals are strings: `'hello'`, `"world"`
- Variables are treated as strings (no auto-conversion)

---

## Type Casting with `as`

When you need to convert between types, use the `as &schema` syntax:

```sky
~ Convert for mathematical operations
solve @age as &number + 1 into @next_age.

~ Convert for text operations
text 'Count: ' + @number as &string into @message.

~ Convert for logical operations
eval @value as &boolean and @flag into @result.
```

## Complex Examples

```sky
~ Mixed operations using different keywords
solve @price * @quantity into @subtotal.
solve @subtotal * 0.08 into @tax.
solve @subtotal + @tax into @total.

text 'Order total: $' + @total as &string into @receipt.

eval @total > 100 into @free_shipping.
eval @free_shipping and @is_member into @apply_discount.
```

## Best Practices

1. **Use the right keyword for the data type:**

   - `solve` for numbers and math
   - `eval` for booleans and logic
   - `text` for strings and concatenation

2. **Cast explicitly when needed** using `as &schema`

3. **Keep expressions simple** - prefer multiple lines over complex nested expressions

4. **Use modules for advanced operations:**
   - `math round`, `math pow` for complex math
   - `text regex`, `text interpolate` for advanced string work
   - `bool all`, `bool any` for advanced logic

---

## Wait, Stop, and Async Control

```sky
async 'upload' http post 'https://api.example.com/file' body @file.
say 'Upload started...'.

~ Cancel the async operation
stop 'upload'.

~ Wait for completion
wait 'upload'.
```

**Keywords:**

- `wait` - Wait for async operation to complete
- `stop` - Cancel async operation
- `until all` - Wait for all operations in group
- `until any` - Wait for any operation in group to complete

```
async 'upload' http post 'https://api.example.com/file' body @f.

say 'launching…'.

stop 'upload'. ~ cancel async group named 'upload'
```
