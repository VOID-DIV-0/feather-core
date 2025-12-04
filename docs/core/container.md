---
title: Container
slug: container
category: core
status: wip
version: 0.0.1
since: 0.0.1
summary: Structured data containers for complex data types.
tags: [container, data-structure, array, map, nested]
---


# Container

## Description

Containers are structured data types in nekonomicon that allow for complex data organization including arrays, maps, and nested structures.

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
