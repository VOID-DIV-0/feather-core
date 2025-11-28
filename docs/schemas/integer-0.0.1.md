# Schema `integer`

Decimal is a primitive scalar schema for floating point number.

## Declaration

`integer`: ±9,223,372,036,854,775,807 (rust `i64`)

**_Examples_**

```spell
40 into @my_var.
-3999 into @my_var2.
'64.5' into @my_var3.
'test' into @my_var4.

~ basic validation
@my_var is integer.
@my_var2 is integer.
safe @my_var3 is integer. ~skipped, my_Var3 is NOT an integer.
safe @my_var4 is integer. ~skipped, my_var4 is NOT an integer.
```

## Related Pages

- [bool **v0.0.1**](../modules/bool-0.0.1.md)
- [clauses **v0.0.1**] (../core/clauses-0.0.1.md)
