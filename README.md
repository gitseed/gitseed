# gitseed
CI system

# Environment
The following environment variables are required.

GITSEED_DB_URL - the DB url in postgres format, for example:
`export GITSEED_DB_URL="postgres:///gitseed"`

LIQUIBASE_COMMAND_URL - The DB url is jdbc format, for example:
`export LIQUIBASE_COMMAND_URL="jdbc:postgresql:gitseed"`
