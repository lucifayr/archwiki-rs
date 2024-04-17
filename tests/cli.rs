use assert_cmd::Command;
use predicates::prelude::{predicate, PredicateBooleanExt};

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

        cmd.assert()
            .failure()
            .stderr(pstr::starts_with("SIMILAR PAGES\nNeovim"));
    }

    Ok(())
}

#[test]
fn test_cli_search_cmd() -> Result<(), Box<dyn std::error::Error>> {
    use predicate::str as pstr;

    {
        let mut cmd = Command::cargo_bin("archwiki-rs")?;
        cmd.args(["search", "Neovim"]);

        cmd.assert()
            .success()
            .stdout(pstr::starts_with("PAGE                 | URL"))
            .stdout(pstr::contains(
                "Neovim               | https://wiki.archlinux.org/title/Neovim",
            ));
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
