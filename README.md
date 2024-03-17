A small Rust application that polls a Net2Grid energy meter and stores the data in a PostgreSQL database.

## Config
`./config.toml.example` contains the configuration template.

## Database schema
The database should have the following schema:

 |    Column      |           Type           | Nullable |
 |----------------|--------------------------|----------|
 | time           | timestamp with time zone | not null |
 | power          | integer                  |          |
 | consumption    | bigint                   |          |
 | production     | bigint                   |          |  
 | gas_consumption| bigint                   |          |

 You could also modify the SQL queries to use your own schema.

