# Actions

## Description

### `success`

The `success` instruction marks the script as successfully completed and stops execution immediately.

- It can be called with no arguments to indicate success.
- It can also return a message or a variable value if provided.
- If a Feather script does not call `success` anywhere, it will automatically be marked as a failure when it reaches the end.

Use `success` to explicitly signal that the script has finished and everything went as expected.

Examples:

```sky
SKY> safe script 'maybe-fails.sh'
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
