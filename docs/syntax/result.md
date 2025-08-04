# Result

## Description

Feather uses mandatory result pattern. This means that any scripts, including internal functions and the script sky script itself need to return a result. The goal is to enforce visibility and clarity for both the writer and the readers any script. If the result is not defined at the end of a sky script or of a function, it's automatically assumed that it was a failure.

#### Examples

##### Script

```sky
~ say 'Hello World!'
```

##### Output

```
~ [ERRO] The script has failed, no result defined.
```

### `success`

The `success` instruction marks the script as successfully completed and stops execution immediately.

- It can be called with no parameters to indicate success.
- It can return either a literal, a record or a container.
- If a Feather script does not call `success` anywhere, it will automatically be marked as a failure when it reaches the end.

Use `success` to explicitly signal that the script or the function has finished and everything went as expected.

#### Examples

```sky
~ safe script 'maybe-fails.sh'
```

```sky
~ This is a .sky ~
~~~~~~~~~~~~~~~~~~

set 'true' into @is_enabled

success @is_enabled ~will return true
```

### `failure`

## Literals

In feather scripting, all variables content are string and interpreted as string unless specified otherwise.

> You can convert soft cast a variable by using the `as` modifier.

Use single quote only ', nothing else.

Examples:

`say 'hello'`
`set 'alex' into @my_name`

2. `into`
