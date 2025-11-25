# Functions

Functions in Feather allow you to define reusable logic blocks with parameters, optional parameters and clear success/failure signaling.

---

## Function Definition

Declare a function using the `function ... end` block:

```feather
function my_func
  # function body here
  success.
end
```

---

## Function Invocation

You can invoke functions in two ways:

- **Direct Invocation:**  
  Call the function by name and supply arguments.

  ```sky
  print_name 'John' 'Doe'
  ```

- **Function Injection (`do`):**  
  Pass the function to a module or block for execution under that module’s rules, using the `do` keyword.

  ```sky
  resilience retry do print_name 'John' 'Doe'
  ```

---

## Parameters

Functions accept parameters, which are injected in order. You can specify required and optional parameters.

### Example 1: Required Parameters

```sky
function print_name
  parameter 1 into @surname
  parameter 2 into @family_name

  say 'My name is @{surname} @{family_name}.'
  success 'Name has been printed out' ~ function must return success/failure
end

print_name 'John' 'Doe'
success
```

**Output:**

```
[INFO] My name is John Doe.
[SUCC] The function has been successful, Result: "Name has been printed out".
[SUCC] The script has been successful.
```

---

### Example 2: Optional Parameters

```sky
function print_name
  parameter 1 into @surname
  parameter 2 into @family_name
  safe parameter 3 into @middle_name

  if @middle_name is unknown
    say 'My name is @{surname} @{family_name}.'
  else
    say 'My name is @{surname} @{middle_name} @{family_name}.'
  end

  success 'Name has been printed out'
end

print_name 'John' 'Doe'
print_name 'John' 'Feather' 'Doe'
success
```

**Output:**

```
[WARN] [SAFE] Parameter 3 is unknown in function "print_name".
[INFO] My name is John Feather Doe.
[SUCC] The function has been successful, Result: "Name has been printed out".
[SUCC] The script has been successful.
```

---

### Example 3: Missing Required Parameter

```sky
function print_name
  parameter 1 into @surname
  parameter 2 into @family_name
  parameter 3 into @middle_name

  say 'My name is @{surname} @{middle_name} @{family_name}.'
  success 'Name has been printed out'
end

print_name 'John' 'Doe'
success
```

**Output:**

```
[ERRO] Parameter 3 could not be put into @middle_name variable for function 'print_name'
[FAIL] The function 'print_name' has failed.
[FAIL] The script has failed.
```

---

### Example 4: Function Output Manipulation

```sky
function full_name
  parameter 1 into @first
  parameter 2 into @last

  success '@{first} @{last}'
end

full_name 'John' 'Doe' as result as uppercase into @fullname
say @fullname
```

**Output:**

```
[SUCC] The function 'full_name' has been successful, Result: "John Doe".
[INFO] JOHN DOE
[SUCC] The script has been successful.
```

---

## Function Injection (`do`)

Using `do fn_name ...` means the function is passed to a module for execution, not called directly. The module (like `resilience`, `timeout`, etc.) controls when and how the function is invoked, often adding features like retries or error handling.

**Example:**

```sky
timeout 5s do print_name 'John' 'Doe'
```

This will run `print_name` but only allow it to execute for up to 5 seconds.

---

## Error Contracts

When working with functions, you may encounter these common error codes:

- `E-FUNC-NOTFOUND`: The function name does not exist.
- `E-FUNC-BADTYPE`: The value provided is not a function or is of the wrong type.
- `E-FUNC-NO-SIGNAL`: The function did not return a success or failure signal as required.

---

## Best Practices

- Always end functions with a `success` or `failure` signal.
- Use `safe parameter` for optional arguments.
- Prefer clear, descriptive parameter names.
- Use function injection for advanced control (retries, timeouts, etc.).

---

## See Also

- [Actions](./actions-0.0.1.md)
- [Resilience Module](../modules/resilience-0.0.1.md)
- [Flow Control](./flow-0.0.1.md)
