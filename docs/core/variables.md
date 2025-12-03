# Variables

Variables in nekonomicon are used to store and manipulate data throughout the script. They can hold different types of values, including strings, numbers, lists, and more. Variables are defined using the `@` symbol followed by the variable name.

For a constant variable that cannot be changed after its initial assignment, use the `!` symbol with the `@` symbol.

## Defining Variables

To define a variable, use the sink `into` keyword followed by the variable name. Variable names can consist of alphanumeric characters and underscores (`_`).

```spell
'Hello, World!' into @greeting.

42 into @answer.

true into @is_active.
```

## Using Variables

Once a variable is defined, you can use it in various commands by referencing its name with the `@` symbol.

```spell
say @greeting.  ~ Outputs: Hello, World!
say @answer.    ~ Outputs: 42
say @is_active.  ~ Outputs: true
```

## Modifying Variables

Variables can be modified by assigning new values to them using the `into` keyword again.

```spell
'Goodbye, World!' into @greeting.
```

## Constant Variables

To define a constant variable that cannot be changed after its initial assignment, use the `!` symbol along with the `@` symbol.

```spell
'This is constant' into @!constant_var.
```

## Nullable Variables

To undefine a variable and free up its memory, you can use the `undefine` command.

```spell
null into @?greeting.  ~ Now @greeting is a nullable variable.

assert @?greeting is null.  ~ This will pass since @greeting is now null.
```
