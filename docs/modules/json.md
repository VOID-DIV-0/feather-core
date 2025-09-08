```json stream open 'my_file.json' into @st.
while not @st:eof
  json stream read @st with tabulation into ::obj.
  say ::obj:greeting.
end
json stream close @st.   ~ optional; auto-closed on scope
success.
```
