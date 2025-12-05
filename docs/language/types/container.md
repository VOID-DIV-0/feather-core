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

Container support following sub node types:

- **Value**: A single scalar value.
- **Array**: An ordered list of values.
- **Container**: A nested container structure.

Container can be created using the `container` block and accessed via projections. For simpler cases, you can produce arrays directly using the `array` intrinsic.

## Anatomy

### Syntax

#### Container Identifier

A container identifier starts with `::` followed by the container name, it can contain alphanumeric characters and underscores (`_`). It represents the root of the container structure. e.g.: `::my_container`

Identifiers can be nullable by prefixing with `?`: `::?maybe_container`.

Identifiers can be sealed (constant) by prefixing with `!`: `::!constant_container`. This means the container cannot be reassigned after its initial definition including all its child nodes.

Container supports To create a container, use the `container` block:

```spell
container
  <value> into :<field_name>                   ~ ✅ Named field
  array <items> into :<field_name>             ~ ✅ Array field
  container ... into :<field_name>             ~ ✅ Nested container
into <::container_name>.
```

Here

#### Container's Projection

Container projections allow navigation within the container structure. There are two types of projections:

- **Field projection** — Named access (`:field`)
- **Index projection** — Positional access (`#n`)

You can chain multiple projections to access nested data, as if navigating a tree structure.

A container path is formed by combining the container identifier with its projections:

```

::fruits#0
^^ ^^^^^ ^
| | └─ projection (index)
| └────── identifier
└─────────── sigil

```

This would represent the first item in the `fruits` container.

```

::user:name:email
^^ ^^^^ ^^^^ ^^^^^
| | | └─── projection (field)
| | └───────── projection (field)
| └────────────── identifier
└────────────────── sigil

```

This would represent the `email` field within the `name` field of the `user` container.

```

::canvas:colors#0#1
^^ ^^^^^^ ^^^^^^ ^ ^
| | | | └─ projection (index)
| | | └─── index sigil (yellow)
| | └──────── field (green)
| └─────────────── identifier (white)
└──────────────────── sigil (purple)

```

This would represent the second item in the first color array within the `colors` field of the `canvas` container.

The Rule: Modifiers Chain to Nodes
! (Sealed) Chains Transitively

```

::!container:field:nested
^^
└─ Everything from here down is sealed

```

Once sealed, always sealed — all child nodes inherit the seal.

? (Nullable) Does NOT Chain Transitively

```

::?container:field
^^ ^^^^^
| └─ NOT nullable (must exist if container exists)
└─ This container can be null

```

Nullable is per-node — each level declares its own nullability.

```

::?users:?profile:!email
^ ^ ^
| | └─ sealed (if profile exists, email can't change)
| └─ nullable (profile might not exist)
└─ nullable (users container might not exist)

```

```

container
 'localhost' into :host
 '5432' into :port
into ::!database.

~ Everything is sealed
'remote' into ::!database:host. ❌ Sealed
'3306' into ::!database:port. ❌ Sealed

container
 'admin' into :username
 'password' into :password
into ::?credentials.

~ ✅ Correct
container
 'im_user' into :name
 'im_pass' into :password
 'phone' into :?contact_method
into ::!an_account.

::!an_account:?contact_method. ~ May be null, sealed by root

~ ❌ Invalid (over-sealing)
container
 'im_user' into :name
 'im_pass' into :!password ❌ Redundant, container is already sealed E-CTNR-PROJ-ALRDY-SEALED
 'phone' into :?contact_method
into ::!an_account.

container
  array 'red' 'green' 'blue' into :colors

into ::artwork.

```

## Error Codes

| Code                  | Description                             | Solution                                        |
| --------------------- | --------------------------------------- | ----------------------------------------------- |
| E-CTNR-SEAL-INHERITED | Field sealed when parent already sealed | Remove `!` from field — it inherits from parent |
| E-CTNR-INVALID-PROJ   | Invalid projection syntax               | Check field name or index format                |
| E-CTNR-NULL-ACCESS    | Accessing null container                | Use nullable check first                        |

```

```
