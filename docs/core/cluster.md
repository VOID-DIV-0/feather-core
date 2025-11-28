# Cluster

Clause clusters must be flat. Nesting a clause cluster within another is not allowed.  
If you need multiple clauses, combine them into a single cluster using multiple clause keywords:

✅ Valid:

```spell
safe sensitive
  script ...
end
```

safe
sensitive
script ...
end
end

### ✅ Valid Clause with Control Flow

```spell
safe while true
  script 'do thing'
end
```

### ❌ Invalid Clause Cluster with Control Flow

```spell
safe
  while true
    script 'do thing'
  end
end
```

Clause clusters are not a general-purpose block wrapper. They are a shorthand for applying clauses to flat lists of instructions, not structural controls.

### Example 1

#### Script

```spell
'my password' into vault 'pass'
'my cred' into vault 'cred'

safe sensitive
   say 'my password @{vault 'pass'}' with risk
   say 'my credential @{vault 'cred'}' with risk
end
```

#### Output

```
[INFO][SENS][RISK] my password 'my password'
[INFO][SENS][RISK] my credential 'my cred'
```

### Example 2

#### Script

```spell
safe repeat 5
   this is invalid instruction
end
```

#### Output

```
[WARN] 'this' is not recognized as an instruction.
[WARN] 'this' is not recognized as an instruction.
[WARN] 'this' is not recognized as an instruction.
[WARN] 'this' is not recognized as an instruction.
[WARN] 'this' is not recognized as an instruction.
```
