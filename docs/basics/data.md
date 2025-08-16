# Data

## Introduction

For feather, the goal was to simplify the experience of manipulating scripting data without losing security, type safety and simplicity.

There is 3 type of data:

- Literals
- Records
- Containers

Records and Literals are immutable strings that will always be interpreted into a structure `container` when fed into an instruction.

## Literals

### Singleline Literals

Data can be in a form of literals. All literals must be wrapped around single quotes `' '` and interpreted as a string.

##### Example

```sky
~ say 'this is a single line literals'.
success.
```

##### Output

```text
[INFO] This is a single line string.
```

#### Multiline Literals

Feather also supports **multiline literals** using the same single-quote syntax:

```sky
~ 'This is
~ a multiline
~ string.' into @text_block.
```

## Records

### Definition

Records are immutable and constant "variable" that act as string content. records are defined with the `@` symbol and cannot be modified. Record can use alphanumerical symbole and underscores (`_`).

##### Example

```sky
~ '3333.45' into @var_1.
~ '-1' into @var_2.
~ 'This is a string' into @var_3.
~
~ say @var_1.
~ say @var_2.
~ success @var_3.
```

##### Output

```text
[INFO] 3333.45
[INFO] -1
[SUCC] The script has been successful, Result: 'This is a string'.
```

### Vaulted Records

When using the vault, records are immediately turned into sensitive and the sensitive concept "contaminate" everything it touch. Requiring to use [safe](clauses.md) clause.

```sky
~ parameter '1' into vault @secret
~ cabinet write file '.mysecret' with content @secret ~ this is unsafe! Error.
```

```sky
~ parameter '1' into vault @secret
~ safe cabinet write file '.mysecret' with content @secret ~ this is safe!
```

## Containers

Even if most of the time string are use on the superficial way, most instruction will use

```sky
container into ::my_list
  add item 'a'.
  add item 'b'.
  add item '-5'.
end

say ::my_list:2 ~ 'b'
say ::my_list:3 ~ 'c'
say ::my_list:'-5' ~ '-5'
```

```sky
container into ::my_dictionary
  add map 'key1' 'value 1'.
  add map 'key2' '0'.
  add map 'key3' '3.3333'.
end

say ::my_dictionary:key1 ~ 'value 1'.
say ::my_dictionary:key2 ~ '0'.
say ::my_dictionary:key3 ~ '3.3333'.
```

```sky
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

```sky
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

```sky
  '5' into @my_number
  @my_number > 5 as boolean into ::my_boolean

  say ::my_boolean:binary ~ '0'
  say ::my_boolean:verbose ~ 'false
  say ::my_boolean ~ {binary '0' , verbose 'false'}
```

```sky
  'hello world' into @lowercase.

  @lowercase as uppercase into @record ~ convert uppercase container into its string record.
  @lowercase as uppercase into :container ~ get the uppercase container

  say @record ~ HELLO WORLD
  say :container:value ~ HELLO WORLD
  say :container ~ { Value 'HELLO WORLD' }
  say @lowercase as uppercase ~ HELLO WORLD
```

Behavior of operators instructions can change depending if an item is lensed into a container or stayed as record

```sky
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

```sky
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

A result container in Feather is a predefined structure that holds the outcome of a script or function. You can access its fields using lenses or direct keys.

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