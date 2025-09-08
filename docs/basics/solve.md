- solve
- wait
- stop
- all of
- none of
- not

- until all
- until any

```
~ Core
solve 5 + 5 into @n.                  ~ 10
solve @n > 8 into @ok.                ~ true
solve 'hi' + '!' into @s.             ~ 'hi!'

~ Math power
math solve round(10 / 3, 2) into @r.  ~ 3.33
math solve pow(2, 16) into @p.        ~ 65536

~ Text power
text solve interpolate '{x} + {y}' with {x: 2, y: 3} into @expr.  ~ '2 + 3'

~ Bool power
bool solve all(::nums, do is_even) into @all_even.
```

```
async 'upload' http post 'https://api.example.com/file' body @f.

say 'launching…'.

stop 'upload'. ~ cancel async group named 'upload'
```
