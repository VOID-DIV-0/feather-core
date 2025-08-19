# Vault

The Vault module is the core of Feather’s secret management and sensitivity system. It provides secure handling of secrets, automatic masking, and explicit egress control with `sensitive` and `with risk`.

---

## Summary Table

| Instruction      | Syntax Example                              | Effect                              | Notes                            |
| ---------------- | ------------------------------------------- | ----------------------------------- | -------------------------------- |
| bind environment | `vault bind environment with filter 'DB_*'` | Bind environment variables to Vault | Supports filter/regex            |
| bind system      | `vault bind system`                         | Bind system secrets to Vault        | Supports filter/regex            |
| bind environment | `vault bind environment with filter 'DB_*'` | Bind environment variables to Vault | Supports filter/regex            |
| bind system      | `vault bind system`                         | Bind system secrets to Vault        | Supports filter/regex            |
| remote           | `vault remote ::http_connection`            | Connect to remote vault provider    | Use connection container         |
| remote pull      | `vault remote pull 'SECRET_NAME'`           | Pull secret from remote vault       | Requires prior remote connection |
| lock             | `vault lock 'NAME' with secret {value}`     | Store secret in Vault               | Value is tainted as sensitive    |
| unlock           | `vault unlock 'NAME' into @secret`          | Retrieve secret from Vault          | Value is tainted as sensitive    |

---

### `bind <vault_target_keyword> [with|without modifier...]`

Thi

**_Properties_**

- `<vault_target_keyword>`: Target source (`environment`, `system`, or `remote`).

**_Modifiers_**

- `with filter {glob}`: Include only names matching glob.
- `with regex {regex}`: Include only names matching regex.

**_Examples_**

---

### `bind system`

---

### `remote`

---

### `remote pull`

---

### `lock`

---

### `unlock`

---

### Ephemeral sensitivity (no vault entry created)

### Store secret in Vault for reuse

```sky
vault lock 'PASSWORD_DEV' with secret @ephemeral_pw.
```

### Bind ENV variables starting with DB\_ into Vault

```sky
vault bind environment with filter 'DB_*'.
```

### Retrieve secrets from Vault

```sky
vault unlock 'PASSWORD_DEV' into @user_password.
vault unlock 'DB_PASS' into @db_secret.
```

### Egress requires with risk

```sky
sensitive say @user_password with risk.
sensitive say @db_secret with risk.
```

---

## Notes

- Every time you unlock a secret, the value is tainted as sensitive and requires `with sensitive` on use and `with risk` on egress.
- All bind instructions support filtering with `filter` or `regex`.
- Remote vaults require a connection container and secrets are pulled individually.

## Related Pages

- [Results **v0.0.1**](./results-0.0.1.md)
