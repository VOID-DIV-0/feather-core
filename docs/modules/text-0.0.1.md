# Text

## Description

`text concat [{value} ...] into @concatenated`
`text join #list with ', ' into @csv`
`text split @s by ',' into ::parts`
text replace @s 'from' 'to' into @t
text contains @s 'needle' into @has
text startswith|endswith @s 'pre/suf' into @ok
text trim|lower|upper @s into @out
text length @s into @len
text slice @s from '2' to '5' into @sub
text format 'Hello {name}' with name @n into @msg
text match @s 'regex' into ::matches

Optional / separate:
• assert @ok 'message'. → fails the script if @ok is false (side-effect; not a bool op)
