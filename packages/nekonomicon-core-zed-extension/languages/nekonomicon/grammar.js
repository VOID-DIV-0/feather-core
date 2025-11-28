module.exports = grammar({
  name: "nekonomicon",

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) => choice($.comment, $.command_line),

    comment: ($) => /~.*/,

    command_line: ($) => seq(optional($.clause), repeat1($._token), "."),

    clause: ($) =>
      choice("safe", "async", "sensitive", "(!)sensitive", "elevated"),

    _token: ($) =>
      choice(
        $.control_keyword,
        $.clause_keyword,
        $.signature_keyword,
        $.variable_scalar,
        $.variable_container,
        $.variable_projection,
        $.string,
        $.number,
        $.boolean,
        $.identifier,
      ),

    control_keyword: ($) =>
      choice(
        "if",
        "else",
        "end",
        "repeat",
        "while",
        "success",
        "failure",
        "increase",
        "decrease",
      ),

    clause_keyword: ($) => choice("is", "as", "into", "with", "without"),

    signature_keyword: ($) =>
      choice("trace", "elapsed", "timeout", "silent", "on"),

    variable_scalar: ($) => /!?@[a-zA-Z_][a-zA-Z0-9_]*/,

    variable_container: ($) =>
      /!?::[a-zA-Z_][a-zA-Z0-9_]*(?::[a-zA-Z_][a-zA-Z0-9_]*)?/,

    variable_projection: ($) =>
      /::[a-zA-Z_][a-zA-Z0-9_]*:[a-zA-Z_][a-zA-Z0-9_]+/,

    string: ($) => /'[^']*'/,

    number: ($) => /\d+(\.\d+)?/,

    boolean: ($) => choice("true", "false"),

    identifier: ($) => /[a-zA-Z][a-zA-Z0-9_-]*/,
  },
});
