# Functions

Function must always be

## Example 1

Injecting parameter to a function.

```sky
function #print_name
  parameter 1 into @surname
  parameter 2 into @family_name

    say 'My name is @{surname} @{family_name}.'
  success 'Name has been printed out' ~ function must return success/failure
end


#print_name 'John' 'Doe'
success
```

```
[INFO] My name is John Doe.
[SUCC] The funciton has been successful, Result: "Name has been printed out".
[SUCC] The script has been successful.
```

## Example 2

Injecting parameter and optional parameter to a function .

```sky
function #print_name
  parameter 1 into @surname
  parameter 2 into @family_name
  safe parameter 3 into @middle_name

  if @middle_name is unknown
    say 'My name is @{surname} @{family_name}.'
  else
    say 'My name is @{surname} @{middle_name} @{family_name}.'
  end

  success 'Name has been printed out' ~ function must return success/failure
end


#print_name 'John' 'Doe'
#print_name 'John' 'Feather' 'Doe
success
```

```text
[WARN] [SAFE] Parameter 3 is unknown in function "print_name".
[INFO] My name is John Feather Doe.
[SUCC] The funciton has been successful, Result: "Name has been printed out".
[SUCC] The script has been successful.
```

## Example 3

Injecting parameter to a function with missing parameter.

```sky
function #print_name
  parameter 1 into @surname
  parameter 2 into @family_name
  parameter 3 into @middle_name

  say 'My name is @{surname} @{middle_name} @{family_name}.'

  success 'Name has been printed out' ~ function must return success/failure
end


#print_name 'John' 'Doe'
success
```

## Example 4

```
[ERRO] Parameter 3 could not be put into @middle_name variable for function 'print_name'
[FAIL] The function 'print_name' has failed.
[FAIL] The script has failed.
```

```sky
function #full_name
  parameter 1 into @first
  parameter 2 into @last

  success '@{first} @{last}'
end

#full_name 'John' 'Doe' as result as uppercase into @fullname
say @fullname
```

```
[SUCC] The function 'print_name' has been successful, Result: "John Doe".
[INFO] JOHN DOE
[SUCC] The script has been successful.
```
