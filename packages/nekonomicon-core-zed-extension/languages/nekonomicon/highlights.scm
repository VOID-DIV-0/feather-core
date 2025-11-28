; Comments
(comment) @comment

; Strings
(string) @string

; Keywords
[
  "if"
  "else"
  "end"
  "repeat"
  "while"
  "success"
  "failure"
] @keyword

; Clauses
[
  "safe"
  "async"
  "sensitive"
  "elevated"
] @keyword.directive

; Special keywords
[
  "is"
  "as"
  "into"
  "with"
  "without"
] @keyword.operator

; Signatures
[
  "trace"
  "elapsed"
  "timeout"
  "silent"
  "on"
] @attribute

; Booleans
[
  "true"
  "false"
] @boolean

; Numbers
(number) @number

; Variables
(variable_scalar) @variable
(variable_container) @variable
(variable_projection) @variable.other.member

; Terminator
"." @punctuation.delimiter
