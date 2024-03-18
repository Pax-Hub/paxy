# Paxy - *A package manager that gets out of your way*

[![Discord Server](https://dcbadge.vercel.app/api/server/vFG57wDxsd?style=flat)](https://discord.gg/vFG57wDxsd)
[![License](https://img.shields.io/github/license/pax-hub/paxy)](LICENSE)
[![GitHub top language](https://img.shields.io/github/languages/top/pax-hub/paxy)](https://www.rust-lang.org/)
<!-- [![Release](https://github.com/pax-hub/paxy/actions/workflows/release.yml/badge.svg)](https://github.com/pax-hub/paxy/actions/workflows/release.yml)
[![Pre-Release (Git)](https://github.com/pax-hub/paxy/actions/workflows/pre_release.yml/badge.svg)](https://github.com/pax-hub/paxy/actions/workflows/pre_release.yml) -->
[![Validation](https://github.com/pax-hub/paxy/actions/workflows/code_validation.yml/badge.svg)](https://github.com/pax-hub/paxy/actions/workflows/code_validation.yml)
[![API Documentation](https://github.com/pax-hub/paxy/actions/workflows/api_documentation.yml/badge.svg)](https://github.com/pax-hub/paxy/actions/workflows/api_documentation.yml)
[![Security Audit](https://github.com/pax-hub/paxy/actions/workflows/security_audit.yml/badge.svg)](https://github.com/pax-hub/paxy/actions/workflows/security_audit.yml)
[![codecov](https://codecov.io/gh/Pax-Hub/paxy/graph/badge.svg?token=RGDGEIBHBZ)](https://codecov.io/gh/Pax-Hub/paxy)

## Table of Contents

- [Paxy - *A package manager that gets out of your way*](#paxy---a-package-manager-that-gets-out-of-your-way)
  - [Table of Contents](#table-of-contents)
  - [For Users](#for-users)
    - [Commandline Usage](#commandline-usage)
    - [User Guide](#user-guide)
  - [For Developers](#for-developers)
    - [API Documentation](#api-documentation)
    - [Cloning](#cloning)
      - [HTTPS](#https)
      - [SSH](#ssh)
    - [Packaging](#packaging)
      - [Arch Linux](#arch-linux)
    - [Continuous Integration (CI)](#continuous-integration-ci)

## For Users

### Commandline Usage
> **Note** This section will be updated with the actual help text of the tool when we determine that the commandline interface is reasonably stable.

Run 
```sh
paxy --help
```
to see the various commandline options available.

### User Guide
Please visit https://pax-hub.github.io/book-paxy for a detailed user guide.

## For Developers

### API Documentation
Please visit https://pax-hub.github.io/paxy the full API documentation.

### Cloning

To download the source code to your local computer for testing, or for development, you can clone from the remote repository using either SSH, or HTTPS. Below are instructions on how to do so using GitHub hosted code as remote.

#### HTTPS

```sh
git clone https://github.com/pax-hub/paxy.git 
```

OR

#### SSH

```sh
git clone git@github.com:pax-hub/paxy.git
```

### Packaging

#### Arch Linux

Change to the project directory (`cd paxy`) and run any of the below scripts:
- `sh packaging/archlinux/setup.sh <MODE>`: Builds and installs a package
- `sh packaging/archlinux/build-package.sh <MODE>`: Just builds a package without installing it locally
- `sh packaging/archlinux/install-package.sh <MODE>`: Just installs a package locally, except if no built package is detected, a package is built.

- where `<MODE>` can be one of the below
     1. `local`: Selects *paxy-local* from the local project that you have cloned already.
     2. `git`: Selects *paxy-git* from the latest git commit.
     3. `stable`: Selects *paxy* from the git tag corresponding to the [`pkgver` specified in the PKGBUILD](packaging/archlinux/paxy/PKGBUILD#L5). If `pkgver=0.0.1`, then the git tag `v0.0.1` is used for packaging. 
     
> **Note**: Any additional parameters passed to the above scripts are automatically sent to `makepkg` or `pacman` (whichever is applicable).

### Continuous Integration (CI)
> TODO
