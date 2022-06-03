commands
<li> `new feature {{branch name}}` --source {{source}}  --fetch</li>
Runs (git add .), (git commit -m checkout), (git checkout -b {{branch name}})
if source is present it runs (git reser --hard {{{source}}}) after all other commands
if --fetch is present it runs (git fetch) as the initial command