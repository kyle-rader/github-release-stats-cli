# ghrs (Github Release Stats)

Fetch stats about Releases on Github.

## Install

Not yet published, but you can install from source:

```shell
cargo install --git https://github.com/kyle-rader/github-release-stats-cli
```

## Usage

```shell
ghrs rust-lang rust --latest
```

```shell
ghrs azuread microsoft-authentication-cli
```

Example output:

```
> ghrs azuread microsoft-authentication-cli
Release: 0.4.0
Tag: 0.4.0
Assets:
  azureauth-0.4.0-osx-arm64.tar.gz
    MB: 24.07
    downloads: 0.20k
  azureauth-0.4.0-osx-x64.tar.gz
    MB: 26.97
    downloads: 2.29k
  azureauth-0.4.0-win10-x64.zip
    MB: 79.37
    downloads: 25.71k

From: https://api.github.com/repos/azuread/microsoft-authentication-cli/releases?per_page=1
Timings:
fetch : 1372 ms
parse : 0 ms
```
