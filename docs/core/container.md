```spell
~ Simple array
array 'a' 'b' 'c' into ::fruits.
say ::fruits[0]. ~ 'a' (0-based?)

~ Simple map
map 'name' 'Alex' 'age' '25' into ::user.
say ::user:name. ~ 'Alex'

~ Nested container
container
  array 'red' 'green' 'blue' into :colors

  map 'width' '100' 'height' '200' into :dimensions

  string 'My Canvas' into :title

  null into :?description ~ nullable field

into ::canvas.

say ::canvas:title. ~ 'My Canvas'
say ::canvas:colors[0]. ~ 'red'
say ::canvas:dimensions:width. ~ '100'
say ::canvas:?description. ~ null (safe access)
```
