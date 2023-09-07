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
