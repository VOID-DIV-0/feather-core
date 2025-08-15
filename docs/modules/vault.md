# Vault Module

The **Vault** module is the core of Feather’s secret management and sensitivity system.  
It provides secure handling of secrets, automatic masking, and explicit egress control with `sensitive` and `with risk`.

---

## 1. Lifecycle of a Sensitive Value

1. **Storage**

   - Secrets can be created or imported into Vault:
     ```sky
     'my-secret' into vault 'API_KEY'
     system filter 'API*' into vault
     ```
   - Vault entries are always **sensitive** by default.

2. **Retrieval**

   - Fetching from Vault always produces a **sensitive value**:
     ```sky
     vault 'API_KEY' into @key
     ```
   - Any variable containing a vault value is automatically sensitive.

3. **Taint Propagation**
   - If any part of an operation involves a sensitive value, the result is **sensitive**:
     ```sky
     @key append '-suffix' into @composite  ~ @composite is now sensitive
     'Bearer @{key}' into @auth_header ~ interpolation taints the string
     ```

---

## 2. Egress Rules

A **sensitive value cannot leave Feather** without explicit acknowledgment.  
Leaving the “safe world” includes:

- Printing to console (`say`, `ask`)
- Writing to a non-vault file (`cabinet file write`, `append`)
- Sending as a process argument or environment variable (`script`, `program`, `… into environment`)
- Network or IPC (future `http`, `queue` modules)
- Clipboard, notification, or other OS outputs

---

### Sensitive and Risk Behavior

- **`sensitive`**:  
  Allows using a vault value for computation and flow, **but it remains masked** in console and logs.
- **`sensitive … with risk`**:  
  Explicitly allows the value to egress to an untrusted sink.
  - Console shows the real value.
  - Logs (memo) **remain masked**.
  - CI may require `--allow-risk`.

---

### Example — Console Output

```sky
vault 'my_secret' into @secret

~ Masked
sensitive say @secret
~ Console: ********
~ Log: [INFO][SENS] say ********

~ Explicit egress
sensitive say @secret with risk
~ Console: my-secret
~ Log: [INFO][SENS][RISK] say Password123!
```

---

## 3. Egress Enforcement (Internal Logic)

1. Every runtime value carries a **sensitivity flag**.
2. Any interpolation or concatenation with a sensitive value **propagates taint**.
3. Any instruction that writes to an external sink:
   - Checks sensitivity
   - Fails if sensitive and `with risk` is missing
   - Masks value in logs regardless
4. Only instructions holding a **RiskGuard** (produced by `with risk`) may reveal values.

---

## 4. Masking and Logs

- Console shows raw values only with `with risk`.
- Memo/logging **always masks** sensitive values:
  ```
  ********
  ```
- Log lines include `[SENS]` and `[RISK]` tags for auditing.

---

## 5. Developer Responsibilities

1. **Never** call OS egress APIs directly (`std::fs`, `Command`, HTTP clients).  
   Always go through the **core egress layer**.
2. **Propagate sensitivity** correctly when building new instructions.
3. **Test** sensitive/no-risk/risk paths for every sink.
4. **Fail closed**: if in doubt, block egress rather than leaking secrets.

---

## 6. Example Use Cases

**Exporting an API key to a process**

```sky
vault 'API_KEY' into @key
sensitive script 'curl -H "Authorization: Bearer @{key}" https://api.example.com' with risk
```

**Writing a secret to a file**

```sky
vault 'DB_PASS' into @pass
sensitive cabinet file write 'secrets.txt' with @pass with risk
```

**Using a secret internally without egress**

```sky
vault 'SALT' into @salt
sensitive 'prefix-@{salt}' into @hash_base
```

---

## 7. Trust Model

- **Safe by default**: secrets cannot escape by accident.
- **Auditable**: reviewers can `grep with risk` to find all egress points.
- **Masked logs**: CI logs are safe even during risk operations.
- **Extensible**: new modules can integrate with Vault by respecting taint and using the core egress API.

for 30 seconds.
