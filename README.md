# sql

[![Build Status](https://travis-ci.org/JamesOwenHall/sql.svg?branch=master)](https://travis-ci.org/JamesOwenHall/sql)

This project is a command line utility for running SQL queries over local files.

### Usage

##### Select individual columns

```sh
sql 'select id, name, balance from "fixtures/accounts.json"'
```

```
"id"	"name"	"balance"
1000	Alice	15.5
1001	Bob	-50.67
1002	Charlie	0
1003	Denise	-1024.64
```

##### Order columns

```sh
sql 'select id, name, balance from "fixtures/accounts.json" order by balance'
```

```
"id"	"name"	"balance"
1003	Denise	-1024.64
1001	Bob	-50.67
1002	Charlie	0
1000	Alice	15.5
```

##### Run aggregate queries

```sh
sql 'select sum(balance) from "fixtures/accounts.json"'
```

```
sum("balance")
-1059.8100000000002
```

##### Filter rows

```sh
sql 'select sum(balance) from "fixtures/accounts.json" where frozen'
```

```
sum("balance")
-1075.3100000000002
```
