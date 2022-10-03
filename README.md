![](https://external-preview.redd.it/gZJnF8uBFF2cy-LgHRk0ZR86e3iVKtWMQdZJbatwhXo.jpg?width=960&crop=smart&auto=webp&s=2643af6aebb925ab3cd294e3d2c506c73c847177)

# Rick And Morty

## 👀 CLI

> A command line interface using Clap

### 🛣️ Features

* [x] Characters (Get all (with filters), Get by id, Get by multiple ids)
* [x] Episodes (Get all (with filters), Get by id, Get by multiple ids)
* [x] Locations (Get all (with filters), Get by id, Get by multiple ids)
* [x] Proxy Server - Spin up API server using Actix Web

---

## Characters

#### 1) Help
```shell
$ cargo run characters -h
```

```shell
Get all or a single character

Usage: rick_and_morty_cli characters <COMMAND>

Commands:
  get-all       Get all
  get           Get by id
  get-multiple  Get multiple by ids
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

#### 1) Get all 
```shell
$ cargo run characters get-all -h 
```

```shell
Get all

Usage: rick_and_morty_cli characters get-all [OPTIONS]

Options:
  -p, --page <PAGE>      Optional: Page number
  -s, --status <STATUS>  Optional: Filter by the given status - Values (alive, dead or unknown)
  -g, --gender <GENDER>  Optional: Filter by the given gender  - Values (female, male, genderless or unknown)
  -h, --help             Print help information
```

#### 2) Get by Id 
```shell
$ cargo run characters get -h 
```

```shell
Get by id

Usage: rick_and_morty_cli characters get --id <ID>

Options:
  -i, --id <ID>  ID
  -h, --help     Print help information
```

#### 3) Get Multiple ids
```shell
$ cargo run characters get-multiple -h 
```

```shell
Get multiple by ids

Usage: rick_and_morty_cli characters get-multiple [OPTIONS]

Options:
  -i, --ids <IDS>  ID - Example 1,2,3
  -h, --help       Print help information
```

## Episodes

#### 1) Help
```shell
$ cargo run episodes -h
```

```shell
Get all or a single episode

Usage: rick_and_morty_cli episodes <COMMAND>

Commands:
  get-all       Get all
  get           Get by id
  get-multiple  Get multiple by ids
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

#### 1) Get all
```shell
$ cargo run episodes get-all -h 
```

```shell
Get all

Usage: rick_and_morty_cli episodes get-all [OPTIONS]

Options:
  -p, --page <PAGE>        Optional: Page number
  -n, --name <NAME>        Optional: Filter by the given name
  -e, --episode <EPISODE>  Optional: Filter by the given episode code
  -h, --help               Print help information
```

#### 2) Get by Id
```shell
$ cargo run episodes get -h 
```

```shell
Get by id

Usage: rick_and_morty_cli episodes get --id <ID>

Options:
  -i, --id <ID>  ID
  -h, --help     Print help information
```

#### 3) Get Multiple ids
```shell
$ cargo run episodes get-multiple -h 
```

```shell
Get multiple by ids

Usage: rick_and_morty_cli episodes get-multiple [OPTIONS]

Options:
  -i, --ids <IDS>  ID - Example 1,2,3
  -h, --help       Print help information
```

## Locations

#### 1) Help
```shell
$ cargo run locations -h
```

```shell
Get all or a single location

Usage: rick_and_morty_cli locations <COMMAND>

Commands:
  get-all       Get all
  get           Get by id
  get-multiple  Get multiple by ids
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

#### 1) Get all
```shell
$ cargo run episodes get-all -h 
```

```shell
Get all

Usage: rick_and_morty_cli locations get-all [OPTIONS]

Options:
  -p, --page <PAGE>            Optional: Page number
  -n, --name <NAME>            Optional: Filter by the given name
  -d, --dimension <DIMENSION>  Optional: Filter by the given dimension
  -h, --help                   Print help information
```

#### 2) Get by Id
```shell
$ cargo run locations get -h 
```

```shell
Get by id

Usage: rick_and_morty_cli locations get --id <ID>

Options:
  -i, --id <ID>  ID
  -h, --help     Print help information
```

#### 3) Get Multiple ids
```shell
$ cargo run locations get-multiple -h 
```

```shell
Get multiple by ids

Usage: rick_and_morty_cli locations get-multiple [OPTIONS]

Options:
  -i, --ids <IDS>  ID - Example 1,2,3
  -h, --help       Print help information
```

## Proxy

```shell
$ cargo run proxy spin-up 
```

Postman Collection inside of `./doc`


