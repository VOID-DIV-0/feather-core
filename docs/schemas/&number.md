# Schema `decimal`

Decimal is a primitive scalar schema for floating point number.

## Declaration

`&decimal`: +- ~+1.8×10^308

**_Examples_**

```sky
4.5 into @my_var.
-9999.0 into @my_var2.
'0' into @my_var3.
'test' into @my_var4.

~ basic validation
@my_var is &decimal.
@my_var2 is &decimal.
@my_var3 is &decimal.
safe @my_var4 is &decimal. ~skipped, my_var4 is NOT a decimal
```

## Related Pages

- [bool **v0.0.1**](../modules/bool-0.0.1.md)
- [clauses **v0.0.1**] (../basics/clauses-0.0.1.md)
