record delete @rec. # remove record entirely (unset/clear)
record exists @rec into @ok. # check if record is bound and non-empty
record empty @rec into @ok. # check if value == ''
record length @rec into @len. # number of characters
record matches @rec 'regex' into @ok.
record is-numeric @rec into @ok.
record is-integer @rec into @ok.
record is-decimal @rec into @ok.
record is-date @rec 'YYYY-MM-DD' into @ok.
record lens @rec as integer into @out. # throws if invalid
record try-lens @rec as integer into @out ok @ok. # sets @ok true/false
record lens @rec as decimal into @out.
record lens @rec as json into ::container.
record ensure @a default 'my_default_value' overwrite @a

• Destination / slot: the place after into.
• Scalar slot: @name (always text). (single value)
• Object slot: ::name (typed, with projections).
• scalar Projection: ::name:field[:subfield…] (read‑only views).

---

## Notes on `into`

### For scalars (`@`)

- `@name` stores a single text value (scalar).
- Overwrites the value if already set.
- Always string-based at base level.
- No projection: you always reference the whole scalar.

### For containers (`::`)

- `::container` can store structured or typed data.
- `::container:field` mutates only that field in the container.
- Root assignment overwrites the whole container.
- Type enforcement is applied (e.g., `::boolean` only accepts true/false).

### Mental model

- Scalars (`@`) are “single sticky notes” for text.
- Containers (`::`) are “labeled boxes” that can have types and compartments.
- `into` is the action of putting a value into the right sticky note or box at the end of a sentence.
