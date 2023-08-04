# ArchWiki CLI 📖
A CLI tool to read pages from the ArchWiki

## Installation
Currently you can only install this tool from [ crates.io ](https://crates.io/crates/archwiki-rs) 
or build it from source

### crates.io

```sh
cargo install archwiki-rs
```
### Source

```sh
git clone https://github.com/jackboxx/archwiki-rs
cd archwiki-rs
cargo build --release
cp ./target/release/archwiki-rs $SOME_DIRECTORY_IN_YOUR_PATH # e.g. $HOME/.cargo/bin
```

## Usage

### Reading Pages

```sh
archwiki-rs read-page Neovim
```

#### Using a different format
```sh
archwiki-rs read-page Neovim --format markdown
```

#### 404 page not found (-̥̥̥n-̥̥̥ )

If the page you are searching for doesn't exists a list of the pages who are most similar
(in name) to the page you asked for will be output instead of the page content. The
categories are stored locally and can be fetched with the [update-all](#updating-everything) 
command.

```sh
archwiki-rs read-page Neovi

# output
Neovim
...
```

Unlike the output when the page name does exist this output is written to stderr instead
of stdout. If you want to you can create a program that checks if no page was found and 
uses stderr to give the user suggestions on what they might have wanted to type.


An example shell script to do something like this is available in the [repository](https://github.com/jackboxx/archwiki-rs)
under the name `example.sh`.

### Downloading page info

Page names are stored locally to prevent having to scrape the entire table of contents of
the ArchWiki with every command

#### Updating everything 

```sh
archwiki-rs update-all
```

Be warned since this scrapes multiple thousand links this is very slow (-, - )…zzzZZ

#### Updating a specific category 

```sh
archwiki-rs update-category Xorg_commands
```

### Listing pages and categories

```sh
archwiki-rs list-pages
```

This is outputs a styled human readable tree of categories and pages but if you need an easily 
parseable list for a different program to use you can use the `-f` flag to flatten the output 
into a newline separated list that only contains the names of all pages

```sh
archwiki-rs list-pages -f
```

### Page file location

The file containing all pages is stored using the [directories](https://crates.io/crates/directories)
crate. See [this](https://docs.rs/directories/latest/directories/struct.BaseDirs.html#method.data_local_dir)
for more information. 
