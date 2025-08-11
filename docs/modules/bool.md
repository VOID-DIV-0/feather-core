# Module `bool`

## Description

The `bool` module enables boolean logic operations on record or container values.

## Features

- Evaluate boolean conditions using expressions.
- Combine multiple boolean values using `all` or `any`.

## Instructions and modifiers list

- `eval <expression>`
- `any @parameter1 @parameter2 @...`
- `all @parameter1 @parameter2 @....`

### `eval`

`eval` evaluates a boolean expression, combining results from other modules or boolean variables.

Supported eval keywords: `and`, `or`, `xor`, `is`, `is not`, `(<command>)`

```
bool eval (math compare @a > @b)
          and
          (text contains @name 'admin')
          into @ok.
```

### `all`

```
math compare @a != '0' into @a_not_zero.
math compare @b != '0' into @b_not_zero.
math compare @a as decimal > @b as decimal into @a_is_bigger.

bool all @a_not_zero @b_not_zero @a_is_bigger into @is_ok.

if @is_ok
  success.
else
  failure.
end
```

Note: Flow statements like `if` and `while` only accept boolean values. Always compute conditions first with `bool` or domain-specific `compare` commands.

```
'5.5' as decimal into ::a.
'4.5' as decimal into ::b.

math floor ::a into ::a. ~'5'
math floor ::b into ::b. ~'4'

math compare ::a > ::b into @is_bigger. ~ 'true'

if @is_bigger
   success.
else
  failure.
end
```

### `any`

This example shows how to use `bool any` to check if any of several conditions are true.

```
math compare @a == '0' into @a_is_zero.
math compare @b == '0' into @b_is_zero.
math compare @c == '0' into @c_is_zero.

if bool any @a_is_zero @b_is_zero @c_is_zero ~ Embedded flow must always be a single line
  say 'At least one value is zero.'
  success.
else
  say 'No values are zero.'
  failure.
end
```

### Bool container

When using the `as` lens keyword, you can turn any literal into a boolean container. `as bool`.

Here its structure

- ::bool
  - verbose: ['true'|'false']
  - binary: ['1'|'0']
  - value: ['true' | 'false' | '1' | '0']

```sky
'true' as bool into ::boolean1.
'0' as bool into ::boolean2.

say ::boolean1:verbose. ~ 'true'
say ::boolean1:binary. ~ '1'
say ::boolean1:value. ~ 'true'

say ::boolean2:verbose. ~ 'false'
say ::boolean2:binary. ~ '0'
say ::boolean2:value. ~ '0'
```
