---
title: Text Module
slug: text
category: module
status: stable
version: 0.0.1
since: 0.0.1
summary: String and pattern operations for text manipulation.
tags: [text, string, pattern, format]
---

# Text

## Description
Unified string and pattern operations: create, transform, inspect, match, and format textual values.

## Summary Table
| Category    | Instruction                                 | Output          |
|-------------|---------------------------------------------|-----------------|
| Basic       | text concat 'a' 'b' into @out.              | record          |
| Basic       | text length @s into @len.                   | record (number) |
| Basic       | text upper @s into @u.                      | record          |
| Structural  | text split @s by ',' into ::parts.          | container(list) |
| Structural  | text join ::parts with ', ' into @csv.      | record          |
| Pattern     | text match @s 'regex' into @has.            | boolean record  |
| Pattern     | text replace @s 'from' 'to' into @new.      | record          |
| Pattern     | text capture @s '([A-Z]+)-(\\d+)' into ::cap.| container       |
| Format      | text format 'Hello {name}' with name @n into @msg. | record |
| Slice       | text slice @s from '2' to '5' into @sub.    | record          |

## Categories
### 1. Basic
(…)
### 2. Structural
(…)
### 3. Pattern & Matching
(…)
### 4. Formatting
(…)
### 5. Slice & Range
(…)
### 6. Edge Cases & Performance
(…)
