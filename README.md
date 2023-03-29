<h1 align="center">Welcome to HowTo 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-1.0.0-blue.svg?cacheSeconds=2592000" />
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

> Get directions for how to do anything in the terminal. Powered by ChatGPT. Like TLDR, but smarter.

## Install

```sh
cargo install howto
```

## Usage

```sh
howto query "unpack a tar.gz file"
```

## Advanced Usage

The `howto` command has two subcommands, `query` and `auth`. The `auth` subcommand has two additional subcommands, `login` and `logout`.


### Log in to the ChatGPT API
The `login` subcommand is used to log in to the ChatGPT API and only needs to be run once.
```bash
$ howto auth login
```

### Log out of the ChatGPT API
The `logout` subcommand is used to log out of the ChatGPT API and clear the API key from the local machine.

```bash
$ howto auth logout
```

### Query the ChatGPT API
The `query` subcommand is used to get directions on how to do something in the terminal. The query takes a single argument, which can be a word or a phrase. 

```bash
$ howto query "unpack a tar.gz file"
```

Internally, the query is prefixed with the phrase `Provide a command to do the following in a terminal:` to limit the context of the query to commands in the terminal.

The response from the ChatGPT API will be printed to the terminal.

## Alias
After installing the `howto` command, you can add an alias to your shell's configuration file to make it easier to use. For example, if you use `bash`, you can add the following to your `~/.bashrc` file:

```bash
alias howto="howto query"
```

Just make sure that you have logged in to the ChatGPT API before adding the alias.

---

## Author

👤 **Rich Baird**

* Github: [@richbai90](https://github.com/richbai90)
* LinkedIn: [@richbai90](https://linkedin.com/in/richbai90)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_