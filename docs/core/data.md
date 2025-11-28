# Data

For nekonomicon, the goal was to simplify the experience of manipulating scripting data without losing security, type safety and simplicity.

There is 4 type of data:

- Literals
- Records
- Containers
- Schema

Records and Literals are immutable strings that will always be interpreted into a structure `container` when fed into an instruction.

## Literals

### Singleline Literals

Data can be in a form of literals. All literals must be wrapped around single quotes `' '` and interpreted as a string.

> [!Note]
>
> Use escape charact '\' when using single quote.

**_Example_**

```spell
say 'this is a single line \'literals\''.

success.
```

### Multiline Literals

nekonomicon also supports **multiline literals** using the same single-quote syntax:

**_Example_**

```spell
'This is
a multiline
string.' into @text_block.

success.
```

### Numerical/Decimal Literals

nekonomicon supports numerical literals. They do not require the string quote syntax but internally are processed as string.

**_Example_**

```spell
say 0.1.
say '4'.

success.
```

### boolean Literals

nekonomicon supports boolean literals. They do not require the string quote syntax but internally are processed as string.

**_Example_**

```spell
say true.
say 'true'.

success.
```

---

## Records

### Definition

Records are immutable and constant "variable" that act as string content. records are defined with the `@` symbol and cannot be modified. Record can be composed of alphanumerical symbol and underscores (`_`).

**_Example_**

```spell
3333.45 into @var_1.
-1 into @var_2.
'This is a string' into @var_3.

say @var_1.
say @var_2.

success @var_3.
```

```text
[INFO] 3333.45
[INFO] -1
[SUCC] The script has been successful, Result: 'This is a string'.
```

## Containers

```
::response:users[0]:name.
         ↑      ↑ ↑    ↑
         |      | |    └─ terminator
         |      | └────── map key
         |      └──────── array index
         └─────────────── map key

```

Even if most of the time string are use on the superficial way, most instruction will use

```spell
container create 'user'
  add item 'id' value 'alex_00001'

  add map 'phones'
    with key 'home' value '000.0000.000'
    with key 'cell' value '111.11111.111'

  add container 'hello_world'
    add item 'greeting' value 'hello world'
  end
end
```

```spell
container into ::my_list
  add item 'a'.
  add item 'b'.
  add item '-5'.
end

say ::my_list:2 ~ 'b'
say ::my_list:3 ~ 'c'
say ::my_list:'-5' ~ '-5'
```

```spell
container into ::my_dictionary
  add map 'key1' 'value 1'.
  add map 'key2' '0'.
  add map 'key3' '3.3333'.
end

say ::my_dictionary:key1 ~ 'value 1'.
say ::my_dictionary:key2 ~ '0'.
say ::my_dictionary:key3 ~ '3.3333'.
```

```spell
container into ::my_own_dataset
  add map 'key1' 'Name'.

  add container 'my_container'
      add item 'val1'.
      add item 'val2'.
      add item 'val3'.
  end
end

say ::my_own_dataset:key1 ~ Name.
say ::my_own_dataset:my_container:1 ~ '1'.
```

### Open

Containers are mutable and can be reopened by using `open`

```spell
container into ::my_list
  add item 'a'.
  add item 'b'.
end

open container ::my_list
  add item 'c'.
  add item 'c'.
end

open container ::my_list
  remove item 'c'.
end

say ::my_list:c ~ not found!
```

### Lenses

You can turn any string literal / record into a container by using lenses

```spell
  '5' into @my_number
  @my_number > 5 as boolean into ::my_boolean

  say ::my_boolean:binary ~ '0'
  say ::my_boolean:verbose ~ 'false
  say ::my_boolean ~ {binary '0' , verbose 'false'}
```

```spell
  'hello world' into @lowercase.

  @lowercase as uppercase into @record ~ convert uppercase container into its string record.
  @lowercase as uppercase into :container ~ get the uppercase container

  say @record ~ HELLO WORLD
  say :container:value ~ HELLO WORLD
  say :container ~ { Value 'HELLO WORLD' }
  say @lowercase as uppercase ~ HELLO WORLD
```

Behavior of operators instructions can change depending if an item is lensed into a container or stayed as record

```spell
'5' into @my_number

say @my_number + '5' ~ It would be 55
say @my_number as integer + '5' ~ It would be 10
```

### Notes on Containers

- Containers must always be initialized before use
- You can nest containers to model structured data
- Containers support dynamic keys, including non-alphanumeric characters if quoted

```

```

#### Example 3: Full example using container

This example shows how to return a container as the result of a function and access its fields.

**Script:**

```spell
~ ~ open json file and add John name
~ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~ function #read_file
~  cabinet read file 'my_file.json' as json into ::jsonfile.
~
~   open container ::jsonfile:content
~     add map 'name' 'John'.
~   end
~
~   success ::jsonfile:content.
~ end
~
~ ~ Calling our function to obtain json
~ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~ #read_file into ::my_json_result.
~
~ say 'my function result state is: ' + ::my_json_result:state.
~ say 'my function result type is: ' + ::my_json_result:type.
~ say 'my function result value is: ' + ::my_json_result:value.
~
~ success 'Json successfully read'.
```

**Output:**

```text
[INFO] Cabinet has successfully read "my_file.json" into a json.
[SUCC] The script "#read_file" has been successful.
[INFO] my function result state is: success
[INFO] my function result type is: container
[INFO] my function result value is: { "user": "Mr.Doe", "name": "John" }.
[SUCC] The script has been successful, Result: 'Json successfully read'.
```

### Anatomy of Result structure

A result container in nekonomicon is a predefined structure that holds the outcome of a script or function. You can access its fields using lenses or direct keys.

```
Container ::Result
    'state': ['success' | 'failure']
    'type': ['literal' | 'record' | 'container']
    'value': '<content>'
End
```

**Accessing Result Data:**

- Use `into` to assign the result to a variable or container.
- Use lenses (`as`) or direct access (`::Result:key`) to extract values

container create 'user'
add item 'id' value 'alex_00001'

add map 'phones'
with key 'home' value '000.0000.000'
with key 'cell' value '111.11111.111'

add container 'hello_world'
add item 'greeting' value 'hello world'
end
end

list create 'my_array' with 'a' 'b' 'c'.

say ::my_array:0. ~ 'a'
say ::my_array:(0). ~ 'a'
