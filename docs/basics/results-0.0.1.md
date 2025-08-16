# Result

## Description

Feather scripts must explicitly declare their outcome. This keeps scripts predictable and self-documenting.

- Use `success` to mark the script as completed successfully.
- Use `failure` to mark the script as failed.

> [!Tip]
>
> Data object: Success support any Feather values, including scalars, text, containers, and container projections. See [data](./data-0.0.1.md) for details.

## Summary Table

| Result  | Syntax Example            | Effect                         | Notes                   |
| ------- | ------------------------- | ------------------------------ | ----------------------- |
| success | `success 'Hello World!'`  | Run only on specified OS       | Must be last signature  |
| failure | `failure ::error:message` | Print command before execution | Can combine with others |

If a Feather script or function reaches the end **without calling `success`**, it will automatically be marked as a failure. This enforces explicit intent and ensures that scripts never exit silently.

The `safe` modifier integrates with this pattern: any instructions of a command marked `safe` will not stop the script if it fails.

The result pattern is also applicable to [functions](./functions.md).

### `success {value}`

The `success` instruction marks the script or the function as successfully completed and stops execution while returning the content associated with the result.

- It can be called with no parameters to indicate success.
- It can return either a literal, a record or a container.
- If a Feather script does not call `success` anywhere, it will automatically be marked as a failure when it reaches the end.

Use `success` to explicitly signal that the script or the function has finished and everything went was expected.

**_Properties_**

- `value [@|:]`: Can be pretty much anything, from scalar, text, object and object projection.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

set true into @is_enabled.

if @is_enabled
   success @is_enabled.
end

failure.
```

```bash
> [SUCC] The script has been successful, Result: 'true'.
```

### `failure {value}`

The `failure` instruction indicates the script or the function as failed either due to an error.

If the result is not defined at the end of a sky script or of a function, it's automatically assumed that it was a failure.

**_Properties_**

- `value [@|:]`: Can be pretty much anything, from scalar, text, object and object projection.

**_Examples_**

```sky
~ Script.sky
~~~~~~~~~~~~~

say 'Hello World!'. ~ script will fails due to missing success.
```

```text
[INFO] Hello World!
[ERRO] The script has failed, no result defined.
```

```sky
~ Script.sky
~~~~~~~~~~~~~

say 'Hello World!'. ~ script will fails due to missing success.
success.
```

```text
[INFO] Hello World!
[ERRO] The script has been successful.
```

## Related Pages

- [Data **v0.0.1**](../data-0.0.1.md)
- [Functions **v0.0.1**](../functions-0.0.1.md)
