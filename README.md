# escape

Just shell escapes its arguments.

The idea is if you need to combine expanded shell arguments with escaped arguments passed through, you can
use this to do something like:

```bash
#! /bin/zsh
export PGHOST="op://My Vault/My DB/server"
export PGDATABASE="my_db_schema"
export PGUSER="op://May Vault/My DB/username"
export PGPORT="op://My Vault/My DB/port"
COMMAND='/Applications/Postgres.app/Contents/Versions/13/bin/psql -h "$PGHOST" -U "$PGUSER" -p "$PGPORT" -d "$PGDATABASE" '"$( escape "$@" )"
PGPASSWORD="op://My Vault/My DB/password" exec op run -- zsh -c "$COMMAND"
```

In this case, I need the `PG*` variables to _not_ be expanded before they're passed to the special 1Password subshell,
but I need to pass the shell's arguments, such as `-c 'SELECT (*) FROM my_table;'` through verbatim, which means
quoted.
