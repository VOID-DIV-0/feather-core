    •	wait <time> <unit>. — pause the current script thread.
    •	Units: ms | seconds | minutes | hours.
    •	wait 'tag'. — block until all tasks in group 'tag' finish (success or failure).
    •	wait any 'tag1' 'tag2' .... — return when one of the listed groups has any task finish. (See discharge rule below.)
    •	wait all 'tag1' 'tag2' .... — block until all listed groups are finished.

wait all. — block until all known async groups are finished.

```
async 'upload' http post 'https://api' body ::payload.
say 'launched…'.
wait 'upload'.
say 'done'.
```

```
repeat '5'
  async 'upload' http post 'https://api' body ::payload.
end
wait 'upload'.
```

```
async 'mirror' http get 'https://fast.example.com/file'.
async 'mirror' http get 'https://backup.example.com/file'.
wait any 'mirror'.
stop 'mirror'.   ~ cancel remaining mirror requests
```

```
async 'prep' script 'prepare.sh'.
async 'ship' script 'ship.sh'.
wait all.        ~ waits both 'prep' and 'ship'
```
