# Result

## Description

Feather scripts must explicitly declare their outcome. This keeps scripts predictable and self-documenting.

- Use `success` to mark the script as completed successfully.
- Use `failure` to mark the script as failed.

Both instructions can optionally include a literal, record or container (see [data](./data.md) for more information).

```
success.
success 'All good'.
success @message.
success ::payload.

failure.
failure 'Something went wrong'.
failure @error_info.
failure ::fail_container.
```

If a Feather script or function reaches the end **without calling `success`**, it will automatically be marked as a failure. This enforces explicit intent and ensures that scripts never exit silently.

The `safe` modifier integrates with this pattern: any instructions of a command marked `safe` will not stop the script if it fails.

The result pattern is also applicable to [functions](./functions.md).

### `success`

The `success` instruction marks the script or the function as successfully completed and stops execution while returning the content associated with the result.

- It can be called with no parameters to indicate success.
- It can return either a literal, a record or a container.
- If a Feather script does not call `success` anywhere, it will automatically be marked as a failure when it reaches the end.

Use `success` to explicitly signal that the script or the function has finished and everything went was expected.

### `failure`

The `failure` instruction indicates the script or the function as failed either due to an error.

If the result is not defined at the end of a sky script or of a function, it's automatically assumed that it was a failure.

### Examples

#### Example 1: Not using result pattern in a sky script

This example shows what happens if you do not declare a result in your script.

**Script:**

```sky
~ say 'Hello World!'.
```

**Output:**

```text
~ [INFO] Hello World!
~ [ERRO] The script has failed, no result defined.
```

#### Example 2: Result with a record or a literal

This example demonstrates returning a record or literal as the result of a script.

**Script:**

```sky
set 'true' into @is_enabled.

if @is_enabled
   success @is_enabled.
end

failure.
```

**Output:**

```text
~ [SUCC] The script has been successful, Result: 'true'.
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
~ [INFO] Cabinet has successfully read "my_file.json" into a json.
~ [SUCC] The script "#read_file" has been successful.
~ [INFO] my function result state is: success
~ [INFO] my function result type is: container
~ [INFO] my function result value is: { "user": "Mr.Doe", "name": "John" }.
~ [SUCC] The script has been successful, Result: 'Json successfully read'.
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
