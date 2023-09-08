use assert_cmd::Command;
use assert_fs::prelude::{FileWriteStr, PathChild};
use predicates::{
    prelude::{predicate, PredicateBooleanExt},
    Predicate,
};

#[test]
fn test_cli_info_cmd() -> Result<(), Box<dyn std::error::Error>> {
    use predicate::str as pstr;

    {
        let data_dir_path = ".local/share/archwiki-rs";
        let cache_dir_description = "cache directory      | stores caches of ArchWiki pages after download to speed up future requests                 |";
        let headers = "NAME                 | DESCRIPTION                                                                                | VALUE";

        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.arg("info");

        cmd.assert().success().stdout(
            pstr::contains(data_dir_path)
                .and(pstr::contains(cache_dir_description).and(pstr::contains(headers))),
        );
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["info", "-c", "-o"]);

        cmd.assert()
            .success()
            .stdout(pstr::ends_with("/.cache/archwiki-rs\n"));
    }

    Ok(())
}

#[test]
fn test_cli_read_page_cmd() -> Result<(), Box<dyn std::error::Error>> {
    use predicate::str as pstr;

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["read-page", "-i", "Neovim"]);

        cmd.assert()
            .success()
            .stdout(pstr::contains("Installation"));
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["read-page", "-i", "Neovi"]);

        cmd.assert().failure().stderr(pstr::starts_with("Neovim"));
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["read-page", "-i", "https://wiki.archlinux.org/title/Emacs"]);

        cmd.assert()
            .success()
            .stdout(pstr::contains("Installation"));
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["read-page", "-i", "https://google.com"]);

        cmd.assert().failure();
    }

    Ok(())
}

#[test]
fn test_cli_search_cmd() -> Result<(), Box<dyn std::error::Error>> {
    use predicate::str as pstr;

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["search", "Neovim"]);

        cmd.assert().success().stdout(pstr::starts_with(
"PAGE                 | URL                                                                                       
Neovim               | https://wiki.archlinux.org/title/Neovim"));
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["search", "Neovim", "-L", "2"]);

        cmd.assert().success().stdout(pstr::contains("\n").count(3));
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["search", "Installation", "-t"]);

        cmd.assert().success().stdout(
            pstr::contains("\n")
                .count(6)
                .and(pstr::contains("PAGE                 | SNIPPET")),
        );
    }

    Ok(())
}

#[test]
fn test_cli_list_languages_cmd() -> Result<(), Box<dyn std::error::Error>> {
    use predicate::str as pstr;

    let mut cmd = Command::cargo_bin("archwiki-rs")?;
    cmd.arg("list-languages");

    cmd.assert()
        .success()
        .stdout(pstr::contains("en                   | English"));

    Ok(())
}

#[test]
fn test_cli_local_wiki_info() -> Result<(), Box<dyn std::error::Error>> {
    use predicate::str as pstr;

    let stdout = {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["sync-wiki", "-p", "-m", "10"]);

        let stdout = String::from_utf8(cmd.assert().success().get_output().stdout.clone()).unwrap();
        pstr::contains("About Arch").eval(&stdout);

        stdout
    };

    let tmp_dir = assert_fs::TempDir::new().unwrap();
    tmp_dir.child("pages.yml").write_str(&stdout).unwrap();

    let tmp_file_path = tmp_dir.path().join("pages.yml");

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["list-pages", "-p", tmp_file_path.to_str().unwrap()]);

        cmd.assert().success().stdout(pstr::contains(
            "About Arch:
───┤Arch boot process
───┤Arch build system",
        ));
    }

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["list-categories", "-p", tmp_file_path.to_str().unwrap()]);

        cmd.assert()
            .success()
            .stdout(pstr::contains("\n").count(10));
    }

    Ok(())
}
