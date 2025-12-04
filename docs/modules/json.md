---
title: JSON Module
slug: json
category: module
status: wip
version: 0.0.1
since: 0.0.1
summary: JSON parsing and serialization operations.
tags: [json, parse, serialize, data-format]
---

```json stream open 'my_file.json' into @st.
while not @st:eof
  json stream read @st with tabulation into ::obj.
  say ::obj:greeting.
end
json stream close @st.   ~ optional; auto-closed on scope
success.
```
