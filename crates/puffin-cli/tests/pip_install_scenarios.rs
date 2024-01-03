#![cfg(all(feature = "python", feature = "pypi"))]
/// Generated by `scripts/scenarios/generate.py`
use std::process::Command;

use anyhow::Result;
use insta_cmd::_macro_support::insta;
use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

use common::{create_venv_py312, BIN_NAME, INSTA_FILTERS};

mod common;

/// requires-package-does-not-exist
///
/// The user requires any version of package `a` which does not exist.
#[test]
fn requires_package_does_not_exist() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-package-does-not-exist-59108293")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 2
        ----- stdout -----

        ----- stderr -----
        error: Package `requires-package-does-not-exist-59108293-a` was not found in the registry.
        "###);
    });

    Ok(())
}

/// requires-exact-version-does-not-exist
///
/// The user requires an exact version of package `a` but only other versions exist
#[test]
fn requires_exact_version_does_not_exist() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-exact-version-does-not-exist-bc5f5f6d")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
          × No solution found when resolving dependencies:
          ╰─▶ Because there is no version of
              requires-exact-version-does-not-exist-bc5f5f6d-a available matching
              ==2.0.0 and requires-exact-version-does-not-exist-bc5f5f6d==0.0.0
              depends on requires-exact-version-does-not-exist-bc5f5f6d-a==2.0.0,
              requires-exact-version-does-not-exist-bc5f5f6d==0.0.0 is forbidden.
              And because there is no version of
              requires-exact-version-does-not-exist-bc5f5f6d
              available matching <0.0.0 | >0.0.0 and root depends on
              requires-exact-version-does-not-exist-bc5f5f6d, version solving failed.
        "###);
    });

    Ok(())
}

/// requires-greater-version-does-not-exist
///
/// The user requires a version of `a` greater than `1.0.0` but only smaller or equal versions exist
#[test]
fn requires_greater_version_does_not_exist() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-greater-version-does-not-exist-670431f9")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
          × No solution found when resolving dependencies:
          ╰─▶ Because there is no version of
              requires-greater-version-does-not-exist-670431f9-a available matching
              >1.0.0 and requires-greater-version-does-not-exist-670431f9==0.0.0
              depends on requires-greater-version-does-not-exist-670431f9-a>1.0.0,
              requires-greater-version-does-not-exist-670431f9==0.0.0 is forbidden.
              And because there is no version of
              requires-greater-version-does-not-exist-670431f9
              available matching <0.0.0 | >0.0.0 and root depends on
              requires-greater-version-does-not-exist-670431f9, version solving
              failed.
        "###);
    });

    Ok(())
}

/// requires-less-version-does-not-exist
///
/// The user requires a version of `a` less than `1.0.0` but only larger versions exist
#[test]
fn requires_less_version_does_not_exist() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-less-version-does-not-exist-9a75991b")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
          × No solution found when resolving dependencies:
          ╰─▶ Because there is no version of
              requires-less-version-does-not-exist-9a75991b-a available matching
              <2.0.0 and requires-less-version-does-not-exist-9a75991b==0.0.0
              depends on requires-less-version-does-not-exist-9a75991b-a<2.0.0,
              requires-less-version-does-not-exist-9a75991b==0.0.0 is forbidden.
              And because there is no version of
              requires-less-version-does-not-exist-9a75991b
              available matching <0.0.0 | >0.0.0 and root depends on
              requires-less-version-does-not-exist-9a75991b, version solving failed.
        "###);
    });

    Ok(())
}

/// transitive-requires-package-does-not-exist
///
/// The user requires package `a` but `a` requires package `b` which does not exist
#[test]
fn transitive_requires_package_does_not_exist() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("transitive-requires-package-does-not-exist-ca79eaa2")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 2
        ----- stdout -----

        ----- stderr -----
        error: Package `transitive-requires-package-does-not-exist-ca79eaa2-b` was not found in the registry.
        "###);
    });

    Ok(())
}

/// requires-direct-incompatible-versions
///
/// The user requires two incompatible, existing versions of package `a`
#[test]
fn requires_direct_incompatible_versions() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-direct-incompatible-versions-350bd4b0")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 2
        ----- stdout -----

        ----- stderr -----
        error: Conflicting versions for `requires-direct-incompatible-versions-350bd4b0-a`: `requires-direct-incompatible-versions-350bd4b0-a==1.0.0` does not intersect with `requires-direct-incompatible-versions-350bd4b0-a==2.0.0`
        "###);
    });

    Ok(())
}

/// requires-transitive-incompatible-with-root-version
///
/// The user requires packages `a` and `b` but `a` requires a different version of `b`
#[test]
fn requires_transitive_incompatible_with_root_version() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-transitive-incompatible-with-root-version-3240dab1")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
          × No solution found when resolving dependencies:
          ╰─▶ Because
              requires-transitive-incompatible-with-root-version-3240dab1-a==1.0.0
              depends on
              requires-transitive-incompatible-with-root-version-3240dab1-b==2.0.0
              and there is no version of
              requires-transitive-incompatible-with-root-version-3240dab1-a
              available matching <1.0.0 | >1.0.0,
              requires-transitive-incompatible-with-root-version-3240dab1-a depends on
              requires-transitive-incompatible-with-root-version-3240dab1-b==2.0.0.
              And because
              requires-transitive-incompatible-with-root-version-3240dab1==0.0.0
              depends on
              requires-transitive-incompatible-with-root-version-3240dab1-b==1.0.0
              and requires-transitive-incompatible-with-root-version-3240dab1==0.0.0
              depends on requires-transitive-incompatible-with-root-version-3240dab1-a,
              requires-transitive-incompatible-with-root-version-3240dab1==0.0.0 is
              forbidden.
              And because there is no version of
              requires-transitive-incompatible-with-root-version-3240dab1
              available matching <0.0.0 | >0.0.0 and root depends on
              requires-transitive-incompatible-with-root-version-3240dab1, version
              solving failed.
        "###);
    });

    Ok(())
}

/// requires-transitive-incompatible-with-transitive
///
/// The user requires package `a` and `b`; `a` and `b` require different versions of `c`
#[test]
fn requires_transitive_incompatible_with_transitive() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;
    let cache_dir = assert_fs::TempDir::new()?;
    let venv = create_venv_py312(&temp_dir, &cache_dir);

    insta::with_settings!({
        filters => INSTA_FILTERS.to_vec()
    }, {
        assert_cmd_snapshot!(Command::new(get_cargo_bin(BIN_NAME))
            .arg("pip-install")
            .arg("requires-transitive-incompatible-with-transitive-8329cfc0")
            .arg("--extra-index-url")
            .arg("https://test.pypi.org/simple")
            .arg("--cache-dir")
            .arg(cache_dir.path())
            .env("VIRTUAL_ENV", venv.as_os_str())
            .current_dir(&temp_dir), @r###"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
          × No solution found when resolving dependencies:
          ╰─▶ Because there is no version of
              requires-transitive-incompatible-with-transitive-8329cfc0-a
              available matching <1.0.0 | >1.0.0 and
              requires-transitive-incompatible-with-transitive-8329cfc0-a==1.0.0
              depends on
              requires-transitive-incompatible-with-transitive-8329cfc0-c==1.0.0,
              requires-transitive-incompatible-with-transitive-8329cfc0-a depends on
              requires-transitive-incompatible-with-transitive-8329cfc0-c==1.0.0.
              And because
              requires-transitive-incompatible-with-transitive-8329cfc0-b==1.0.0
              depends on
              requires-transitive-incompatible-with-transitive-8329cfc0-c==2.0.0
              and there is no version of
              requires-transitive-incompatible-with-transitive-8329cfc0-b
              available matching <1.0.0 | >1.0.0,
              requires-transitive-incompatible-with-transitive-8329cfc0-a *,
              requires-transitive-incompatible-with-transitive-8329cfc0-b * are
              incompatible.
              And because
              requires-transitive-incompatible-with-transitive-8329cfc0==0.0.0
              depends on requires-transitive-incompatible-with-transitive-8329cfc0-a
              and requires-transitive-incompatible-with-transitive-8329cfc0==0.0.0
              depends on requires-transitive-incompatible-with-transitive-8329cfc0-b,
              requires-transitive-incompatible-with-transitive-8329cfc0==0.0.0 is
              forbidden.
              And because there is no version of
              requires-transitive-incompatible-with-transitive-8329cfc0
              available matching <0.0.0 | >0.0.0 and root depends on
              requires-transitive-incompatible-with-transitive-8329cfc0, version
              solving failed.
        "###);
    });

    Ok(())
}