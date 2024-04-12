# archwiki-rs 📖

A CLI tool to read pages from the ArchWiki

<!-- toc -->

- [Installation](#installation)
  - [crates.io](#cratesio)
  - [Source](#source)
- [Usage](#usage)
  - [Reading Pages](#reading-pages)
    - [Basic request](#basic-request)
    - [Using a different format](#using-a-different-format)
    - [Caching](#caching)
    - [404 page not found (-̥̥̥n-̥̥̥ )](#404-page-not-found--%CC%A5%CC%A5%CC%A5n-%CC%A5%CC%A5%CC%A5-)
  - [Searching the ArchWiki](#searching-the-archwiki)
    - [Search by title](#search-by-title)
    - [Search for text](#search-for-text)
  - [Downloading wiki info](#downloading-wiki-info)
  - [Listing ArchWiki information](#listing-archwiki-information)
    - [Listing pages](#listing-pages)
    - [Listing categories](#listing-categories)
    - [Listing languages](#listing-languages)
  - [Downloading a local copy of the ArchWiki](#downloading-a-local-copy-of-the-archwiki)
    - [Possible speed-ups](#possible-speed-ups)
  - [Other Information](#other-information)
  - [Setup shell completion](#setup-shell-completion)
- [Plugins](#plugins)
- [Alternatives](#alternatives)

<!-- tocstop -->

## Installation

Currently, you can only install this tool from [ crates.io ](https://crates.io/crates/archwiki-rs)
or build it from source.

After installation you might want to run the [`sync-wiki`](#downloading-wiki-info) command.

### crates.io

```sh
cargo install archwiki-rs
```

### Source

```sh

git clone https://gitlab.com/Jackboxx/archwiki-rs
cd archwiki-rs
cargo install --path .
```

## Usage

### Reading Pages

#### Basic request

```sh
archwiki-rs read-page Neovim
```

#### Using a different format

```sh
archwiki-rs read-page Neovim --format markdown
```

#### Caching

By default, pages are cached in the file system after they are fetched and subsequent
request for that page then use that cache. The cache is invalidated if the cached file hasn't
been modified in the last 14 days.

#### 404 page not found (-̥̥̥n-̥̥̥ )

If the page you are searching for doesn't exist, a list of the pages that are most similar
(in name) to the page you asked for will be output instead of the page content

```sh
archwiki-rs read-page Neovi

# output
Neovim
...
```

Unlike the output when the page name does exist, this output is written to stderr instead
of stdout. If you want to, you can create a program that checks if no page was found and
uses stderr to give the user suggestions on what they might have wanted to type.

An example shell script to do something like this is available in the [repository](https://gitlab.com/jackboxx/archwiki-rs)
under the name `example.sh` which can be used like this `sh example.sh <page-name>`.

### Searching the ArchWiki

#### Search by title

```sh
archwiki-rs search "Emacs"
```

This returns a table of pages with a similar title and their URLs

#### Search for text

```sh
archwiki-rs search "shell" -t
```

This returns a table of pages which contain the search term and the snippet of text
that the search term is in

### Downloading wiki info

Page and category names are stored locally for faster look-ups.
Use this command to fetch all page and category names.

```sh
archwiki-rs sync-wiki
```

### Listing ArchWiki information

#### Listing pages

```sh
archwiki-rs list-pages
```

This outputs a styled tree of categories and pages but if you need an easily parseable
list for a different program to use, you can use the `-f` flag to flatten the output into a
newline separated list that only contains the names of all pages

```sh
archwiki-rs list-pages -f
```

You can also limit the list to only include pages that belong to a specific category

```sh
archwiki-rs list-pages -c "Xorg commands"
```

#### Listing categories

To do the same for categories you can run

```sh
archwiki-rs list-categories
```

#### Listing languages

And the same for available languages

```sh
archwiki-rs list-languages
```

### Downloading a local copy of the ArchWiki

Use this command to download a local copy of the ArchWiki. Be warned, this command makes over
10,000 requests to the ArchWiki so it takes a while to finish (-, -)…zzzZZ

```sh
archwiki-rs local-wiki ~/local-archwiki --format markdown
```

#### Possible speed-ups

If you don't mind your CPU and network becoming a bit saturated you can increase the
amount of threads used to fetch data from the wiki.

Keep in mind that you might get rate limited by the ArchWiki if you make too many requests at once.

```sh
archwiki-rs local-wiki -t 8
```

### Application Information

Other information such as the value/location of the `cache directory` can be obtained
using the `info` command

```sh
archwiki-rs info
```

To only get the value of an entry and not the name and description that belong to it you
can use the `-o` flag

```sh
archwiki-rs info -o
```

### Setup shell completion

You can generate a completion file to allow tab completion for most popular shells
([list of supported shells](https://docs.rs/clap_complete/latest/clap_complete/shells/enum.Shell.html)).

The following example shows how to setup completion for ZSH (with the [oh my zsh](https://github.com/ohmyzsh/ohmyzsh)).

```sh
archwiki-rs completions > /home/iusearchbtw/.oh-my-zsh/completions/_archwiki-rs
```

## Plugins

Here's a list of programs that have plugins for `archwiki-rs` to make your life easier

- [Neovim](https://github.com/Jackboxx/archwiki-nvim)
- [Obsidian](https://github.com/Jackboxx/archwiki-obsidian) (only supported up to v2.2.3)

### Useful info for plugin developers

#### Outputting JSON

For the following commands you can use the `--json` and `--json-raw` flags to get the
output as easily parseable JSON for your program to use:

- [info](#application-information)
- [search](#searching-the-archwiki)
- [list-pages](#listing-pages)
- [list-categories](#listing-categories)
- [list-languages](#listing-languages)

## Alternatives

If you are using Arch Linux a great alternative for this tool is the `wikiman` CLI tool
in combination with the `arch-wiki-docs` package
