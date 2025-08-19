# Schema `bool`

Bool is a primitive scalar (numerical/textual) schema. It's expected to have either 'true', 'false'.

## Declaration

`&bool`: [`true`| `false`]

**_Examples_**

```sky
'true' into @my_var.
'1' into @my_var2.
'True' into @my_var3.

~ basic validation
@my_var is &bool.
safe @my_var2 is &bool . ~invalid! -- skipped due to safe.
safe @my_var3 is &bool. ~invalid! -- skipped due to safe.

bool coerce @my_var2 into override @my_var2. ~ '1' -> true
bool coerce @my_var3 into override @my_var3. ~ 'True' -> true

bool all @my_var
say @my_var. ~true
say @my_var2. ~true
say @my_var3. ~true
```

## Related Pages

- [bool **v0.0.1**](../modules/bool-0.0.1.md)
- [clauses **v0.0.1**](../basics/clauses-0.0.1.md)
