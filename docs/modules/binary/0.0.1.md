# Module: `binary`

## Description

Instructions for manipulating binary data: encoding, decoding, bitwise operations, and format conversion. Useful for files, network packets, and low-level data.

## Summary Table

| Instruction | Syntax Example                                      | Effect                             | Notes                              |
| ----------- | --------------------------------------------------- | ---------------------------------- | ---------------------------------- |
| encode      | `binary encode @input into ::output`                | Encode input to binary container   | Supports formats: base64, hex, raw |
| decode      | `binary decode @input format 'base64' into @output` | Decode input from format to string |                                    |
| bitwise     | `binary bitwise and @a @b into ::output`            | Bitwise operation on inputs        | Operations: and, or, xor, not      |
| not         | `binary not @input into @output`                    | Bitwise NOT                        |                                    |
| shift       | `binary shift @input left 2 into @output`           | Shift bits left/right              |                                    |

> [!Information]
>
> The `into` clause is optional. When used, it captures output into a record (`@var`) or a container (`::var`). Omitting `into` allows embedding the instruction within other expressions.

---

## Usage

### `encode input [into output]`

```sky
binary encode 'Hello World!' format 'base64' into @encoded.
say @encoded. ~ 'SGVsbG8gV29ybGQh'.
```

```sky
binary encode 'Hello World!' into ::encoded.
say ::encoded:base64. ~ 'SGVsbG8gV29ybGQh'.
say ::encoded:hex. ~ '48656c6c6f20576f726c6421'.
say ::encoded:raw. ~ 'Hello World!'.
```

### Decode base64 to text

```sky
binary decode 'SGVsbG8gV29ybGQh' format 'base64' into @decoded.
say @decoded. ~ 'Hello World!'.
```

### Bitwise AND

```sky
binary bitwise and '10101010' '11001100' into @result.
say @result. ~ '10001000'.
```

### Bitwise NOT

```sky
binary not '10101010' into @result.
say @result. ~ '01010101'.
```

### Shift left

```sky
binary shift '10101010' left 2 into @result.
say @result. ~ '1010101000'.
```

---

## Binary Container Content

A binary container holds raw binary data and may include metadata such as encoding or length. You can add custom fields as needed.

```sky
Container ::Binary
    'hex' @hex_value
    'raw' @raw_value
    'base64' @base64_value
End
# Module `binary`

## Description

The `binary` module provides instructions for manipulating binary data, such as encoding, decoding, and performing bitwise operations. It is useful for handling files, network packets, and low-level data processing.

## Features

- Encode and decode binary data
- Perform bitwise operations (AND, OR, XOR, NOT, SHIFT)
- Convert between binary and other formats (hex, base64, text)
- Inspect and modify binary containers

## Instructions and Modifiers List



- `encode @input into ::output` — Binary container (default, multiple projections)
- `encode @input as <format> into @output` — Record (string) with one representation

- `decode @input into ::output` — Binary container (default, multiple projections)
- `decode @input as <format into @output` — Record (string) with one representation

> _Note:_ Supported format are `base64`, `hex` and `raw`.

- `bitwise <operation> @param1 @param2 into ::output` — Binary container (default, multiple projections)
- `bitwise <operation> @param1 @param2 into @output` — Record (string) with one representation

> operations are `and`, `or`,`xor`,`not`.

- `not @input into ::output` — Binary container (default, multiple projections)
- `not @input into @output` — Record (string) with one representation

- `shift @input <direction> <amount> into ::output` — Binary container (default, multiple projections)
- `shift @input <direction> <amount> into @output` — Record (string) with one representation

## Examples

### Encode text to base64

```

binary encode 'Hello World!' format 'base64' into @encoded.
say @encoded. ~ 'SGVsbG8gV29ybGQh'.

```

```

binary encode 'Hello World!' into ::encoded.
say ::encoded:base64. ~ 'SGVsbG8gV29ybGQh'.
say ::encoded:hex. ~ '48656c6c6f20576f726c6421'.
say ::encoded:raw. ~ 'Hello World!'.

```

### Decode base64 to text

```

binary decode 'SGVsbG8gV29ybGQh' format 'base64' into @decoded.
say @decoded. ~ 'Hello World!'.

```

### Bitwise AND

```

binary bitwise and '10101010' '11001100' into @result.
say @result. ~ '10001000'.

```

### Bitwise NOT

```

binary not '10101010' into @result.
say @result. ~ '01010101'.

```

### Shift left

```

binary shift '10101010' left 2 into @result.
say @result. ~ '1010101000'.

```

## Binary Container Content

A binary container holds raw binary data and may include metadata such as encoding or length. You can add custom fields as needed.

```

Container ::Binary
'hex' @hex_value
'raw' @raw_value
'base64' @base64_value
End

```

```
