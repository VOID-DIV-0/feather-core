> WIP

- `any`
- `all`
- `repeat`
- `if/else/elseif`
- `iterate by until`

## Conditionals and Loops

Feather supports straightforward flow control with English-like keywords.

### `if` / `else if` / `else`

Use `if` for conditional logic. `else if` and `else` can be used for additional branches. Always close the block with `end`.

```sky
if @value > 10
  say 'Value is greater than 10'
else if @value > 5
  say 'Value is greater than 5'
else
  say 'Value is 5 or less'
end
```

4. `timeout`

   • Flow tags (control timing/scope): once, retry 'N', timeout '5s', parallel/sequential

### `repeat`

Repeat a block a fixed number of times. Close with `end`.

```sky
repeat @count
  say 'looping'
end
```

### `while`

Repeat a block while a condition is true.

```sky
while @count > '0'
  say 'count is @{count}'
  decrease @count to '1'
end
```

### `increase` and `decrease`

Increment or decrease a value up to or down to a target, executing the block each step.

```sky
increase @i by '2' to '10'
  say 'increase: @{i}'
end

decrease @i by '3' to '0'
  say 'decrease: @{i}'
end
```
