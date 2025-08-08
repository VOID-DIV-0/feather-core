# Feather vs Shells — Practical Side‑by‑Side Examples

This page shows how common automation tasks look in **Bash**, **Windows CMD**, **PowerShell**, and **Feather**.  
Feather favors **clarity and safety**: one action per line, explicit modules, and an explicit result.

> Tip: In Feather examples below, endings like `success.` or `failure.` are intentional. A script that reaches the end without a result is considered failed by design.

---

## Task 1 — Greet user from an environment variable with a fallback

### Bash

```bash
name="${USER:-world}"
echo "Hello, $name"
```

### CMD

```bat
set "name=%USERNAME%"
if "%name%"=="" set "name=world"
echo Hello, %name%
```

### PowerShell

```powershell
$name = $env:USERNAME
if ([string]::IsNullOrEmpty($name)) { $name = 'world' }
Write-Host "Hello, $name"
```

### Feather

```sky
system get 'USERNAME' into @name.

if(bool eval (string require @name))
  failure 'name is empty'
end

say 'Hello @{name}'.
success.
```

---

## Task 2 — Read a JSON file and print a field (`config.json` → `user.name`)

### Bash (with jq)

```bash
name=$(jq -r '.user.name' config.json)
echo "User: $name"
```

### CMD (calls PowerShell for JSON)

```bat
for /f "usebackq delims=" %%i in (`powershell -NoProfile -Command "(Get-Content config.json | ConvertFrom-Json).user.name"`) do set "name=%%i"
echo User: %name%
```

### PowerShell

```powershell
$obj = Get-Content config.json | ConvertFrom-Json
Write-Host "User: $($obj.user.name)"
```

### Feather

```sky
cabinet read file 'config.json' as json into ::cfg.
say 'User: @{::cfg:content:user:name}'.
success.
```

---

## Task 3 — Backup a folder with a datestamped name

> Copy `./project` to `./backups/project_YYYY-MM-DD`

### Bash

```bash
date=$(date +%F)
cp -a ./project "./backups/project_$date"
```

### CMD

```bat
for /f "tokens=1-3 delims=/ " %%a in ("%date%") do set dt=%%c-%%a-%%b
xcopy /e /i /y project "backups\\project_%dt%"
```

### PowerShell

```powershell
$dt = Get-Date -Format "yyyy-MM-dd"
Copy-Item -Recurse -Force ./project "backups/project_$dt"
```

### Feather

```sky
time now format 'YYYY-MM-DD' into @dt.
cabinet copy folder './project' to 'backups/project_@{dt}'.
success 'backup complete'.
```

---

## Task 4 — Find files containing text (recursive) and print matches

### Bash (ripgrep/grep)

```bash
grep -R --line-number --ignore-case "TODO" ./src
```

### CMD

```bat
findstr /S /N /I /C:"TODO" src\*.*
```

### PowerShell

```powershell
Select-String -Path .\src\* -Pattern "TODO" -SimpleMatch -CaseInsensitive
```

### Feather

```sky
cabinet list in './src' with recursion as tree into ::files
text lower 'TODO' into @needle.

iterate ::files as ::file
  cabinet read file ::file:path into @content.
  bool eval text contains @content @needle into @has.
  if @has
    say '@{::file:path}'
  end
end

success.
```

---

## Task 5 — Dangerous delete, but **safe by default**

> Delete a temp folder. In Feather we mark it `safe` so failures won’t halt the script, and we can add `chrono` to time it.

### Bash

```bash
rm -rf ./temp || true
```

### CMD

```bat
rmdir /s /q temp
```

### PowerShell

```powershell
Remove-Item -Recurse -Force ./temp -ErrorAction SilentlyContinue
```

### Feather

```sky
safe cabinet delete folder './temp' with force chrono.
success.
```

---

## Task 6 — Simple arithmetic with a check

### Bash

```bash
a=5; b=7
sum=$((a + b))
if [ "$sum" -gt 10 ]; then echo "ok"; fi
```

### CMD

```bat
set /a sum=5+7
if %sum% gtr 10 echo ok
```

### PowerShell

```powershell
$sum = 5 + 7
if ($sum -gt 10) { Write-Host "ok" }
```

### Feather

```sky
math add '5' '7' into @sum.
bool eval @sum > '10' into @ok.
if @ok
  say 'ok'.
end
success.
```

---

## Notes on Philosophy

- Feather prefers **clarity over cleverness**: explicit modules (`cabinet`, `text`, `math`, `time`) and one action per line.
- Security is **opt‑in explicit**: use `safe`, `sensitive`, and `with risk` where appropriate.
- Scripts **must** end with a `success`/`failure`, making outcomes unambiguous.
- Use **`eval`** for expression grammar _inside a module_ (`math eval`, `bool eval`), and **`@{}` only for string interpolation**.

---

## _Have a great mini‑example you want to add? Drop it here and we’ll keep this page tight and practical._
