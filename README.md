# [rtx](https://github.com/jdxcode/rtx)

[![Crates.io](https://img.shields.io/crates/v/rtx-cli.svg)](https://crates.io/crates/rtx-cli)
[![License: MIT](https://img.shields.io/github/license/jdxcode/rtx)](https://github.com/jdxcode/rtx/blob/main/LICENSE)
[![CI](https://github.com/jdxcode/rtx/actions/workflows/rtx.yml/badge.svg?branch=main)](https://github.com/jdxcode/rtx/actions/workflows/rtx.yml)
[![Codecov](https://codecov.io/gh/jdxcode/rtx/branch/main/graph/badge.svg?token=XYH3Q0BOO0)](https://codecov.io/gh/jdxcode/rtx)
[![Discord](https://img.shields.io/discord/1066429325269794907)](https://discord.gg/mABnUDvP57)

_Polyglot runtime manager (asdf rust clone)_

## 30 Second Demo

The following shows using rtx to install [nodejs](https://nodejs.org) and
[jq](https://stedolan.github.io/jq/) into a project using a `.tool-versions` file.
[hyperfine](https://github.com/sharkdp/hyperfine) is used to show the performance using
rtx vs asdf. (See [Performance](#performance)).
Note that calling `which node` gives us a real path to the binary, not a shim.

[![demo](./docs/demo.gif)](./docs/demo.gif)

## Features

- **asdf-compatible** - rtx is compatible with asdf plugins and `.tool-versions` files. It can be used as a drop-in replacement.
- **Polyglot** - compatible with any language, so no more figuring out how nvm, nodenv, pyenv, etc work individually—just use 1 tool.
- **Fast** - rtx is written in Rust and is very fast. 20x-200x faster than asdf.
- **No shims** - shims (used by asdf) cause problems, they break `which node`, and add overhead. We don't use them by default.
- **Better UX** - asdf is full of strange UX decisions (like `asdf plugin add` but also `asdf install`). We've taken care to make rtx easy to use.
- **Fuzzy matching and aliases** - no need to specify exact version numbers like with asdf.
- **One command install** - No need to manually install each plugin, just run `rtx install` and it will install all the plugins you need.
- **Arbitrary env vars** - Set custom env vars when in a project directory like `NODE_ENV=production` or `AWS_PROFILE=staging`.

## Quickstart

Install rtx (other methods [here](#installation)):

```sh-session
$ curl https://rtx.pub/rtx-latest-macos-arm64 > ~/bin/rtx
$ chmod +x ~/bin/rtx
$ rtx --version
rtx 1.20.4
```

Hook rtx into to your shell. This will automatically add `~/bin` to `PATH` if it isn't already.
(choose one, and open a new shell session for the changes to take effect):

```sh-session
$ echo 'eval "$(~/bin/rtx activate bash)"' >> ~/.bashrc
$ echo 'eval "$(~/bin/rtx activate zsh)"' >> ~/.zshrc
$ echo '~/bin/rtx activate fish | source' >> ~/.config/fish/config.fish
```

> **Warning**
>
> If you use direnv with `layout python` or other logic that needs to reference rtx runtimes inside
> of an `.envrc`, see the [direnv section](#direnv) below.

Install a runtime and set it as the default:

```sh-session
$ rtx install nodejs@18
$ rtx global nodejs@18
$ node -v
v18.10.9
```

> **Note**
>
> `rtx install` is optional, `rtx global` will prompt to install the runtime if it's not
> already installed. This is configurable in [`~/.config/rtx/config.toml`](#configuration).

## Table of Contents

<!--ts-->
   * [About](#about)
      * [What do I use this for?](#what-do-i-use-this-for)
      * [How it works](#how-it-works)
      * [Common example commands](#common-example-commands)
   * [Installation](#installation)
      * [Standalone](#standalone)
      * [Homebrew](#homebrew)
      * [Cargo](#cargo)
      * [npm](#npm)
      * [GitHub Releases](#github-releases)
      * [apt](#apt)
      * [dnf](#dnf)
      * [yum](#yum)
      * [aur](#aur)
      * [nix](#nix)
   * [Other Shells](#other-shells)
      * [Bash](#bash)
      * [Fish](#fish)
      * [Xonsh](#xonsh)
      * [Something else?](#something-else)
   * [Uninstalling](#uninstalling)
   * [Configuration](#configuration)
      * [.tool-versions](#tool-versions)
      * [Legacy version files](#legacy-version-files)
      * [Global config: ~/.config/rtx/config.toml](#global-config-configrtxconfigtoml)
      * [[experimental] .rtx.toml](#experimental-rtxtoml)
      * [Environment variables](#environment-variables)
         * [RTX_MISSING_RUNTIME_BEHAVIOR](#rtx_missing_runtime_behavior)
         * [RTX_DATA_DIR](#rtx_data_dir)
         * [RTX_CACHE_DIR](#rtx_cache_dir)
         * [RTX_CONFIG_FILE](#rtx_config_file)
         * [RTX_DEFAULT_TOOL_VERSIONS_FILENAME](#rtx_default_tool_versions_filename)
         * [RTX_${PLUGIN}_VERSION](#rtx_plugin_version)
         * [RTX_LEGACY_VERSION_FILE](#rtx_legacy_version_file)
         * [RTX_LOG_LEVEL=trace|debug|info|warn|error](#rtx_log_leveltracedebuginfowarnerror)
         * [RTX_LOG_FILE=~/.rtx/rtx.log](#rtx_log_filertxrtxlog)
         * [RTX_LOG_FILE_LEVEL=trace|debug|info|warn|error](#rtx_log_file_leveltracedebuginfowarnerror)
         * [RTX_VERBOSE=1](#rtx_verbose1)
         * [RTX_ASDF_COMPAT=1](#rtx_asdf_compat1)
         * [RTX_JOBS=1](#rtx_jobs1)
         * [RTX_RAW=1](#rtx_raw1)
         * [RTX_SHORTHANDS_FILE=~/.config/rtx/shorthands.toml](#rtx_shorthands_fileconfigrtxshorthandstoml)
         * [RTX_DISABLE_DEFAULT_SHORTHANDS=1](#rtx_disable_default_shorthands1)
         * [RTX_HIDE_OUTDATED_BUILD=1](#rtx_hide_outdated_build1)
         * [RTX_EXPERIMENTAL=1](#rtx_experimental1)
         * [[experimental] RTX_SHIMS_DIR=~/.local/share/rtx/shims](#experimental-rtx_shims_dirlocalsharertxshims)
   * [Aliases](#aliases)
   * [Plugins](#plugins)
      * [Plugin Options](#plugin-options)
   * [FAQs](#faqs)
      * [I don't want to put a .tool-versions file into my project since git shows it as an untracked file.](#i-dont-want-to-put-a-tool-versions-file-into-my-project-since-git-shows-it-as-an-untracked-file)
      * [rtx is failing or not working right](#rtx-is-failing-or-not-working-right)
      * [Windows support?](#windows-support)
      * [How do I use rtx with http proxies?](#how-do-i-use-rtx-with-http-proxies)
      * [How do the shorthand plugin names map to repositories?](#how-do-the-shorthand-plugin-names-map-to-repositories)
      * [How do I migrate from asdf?](#how-do-i-migrate-from-asdf)
      * [rtx isn't working with tmux](#rtx-isnt-working-with-tmux)
   * [Commands](#commands)
      * [rtx activate](#rtx-activate)
      * [rtx alias get](#rtx-alias-get)
      * [rtx alias ls](#rtx-alias-ls)
      * [rtx alias set](#rtx-alias-set)
      * [rtx alias unset](#rtx-alias-unset)
      * [rtx bin-paths](#rtx-bin-paths)
      * [rtx cache clear](#rtx-cache-clear)
      * [rtx complete](#rtx-complete)
      * [rtx current](#rtx-current)
      * [rtx deactivate](#rtx-deactivate)
      * [rtx direnv activate](#rtx-direnv-activate)
      * [rtx doctor](#rtx-doctor)
      * [rtx env](#rtx-env)
      * [rtx exec](#rtx-exec)
      * [rtx global](#rtx-global)
      * [rtx implode](#rtx-implode)
      * [rtx install](#rtx-install)
      * [rtx latest](#rtx-latest)
      * [rtx local](#rtx-local)
      * [rtx ls](#rtx-ls)
      * [rtx ls-remote](#rtx-ls-remote)
      * [rtx plugins install](#rtx-plugins-install)
      * [rtx plugins ls](#rtx-plugins-ls)
      * [rtx plugins ls-remote](#rtx-plugins-ls-remote)
      * [rtx plugins uninstall](#rtx-plugins-uninstall)
      * [rtx plugins update](#rtx-plugins-update)
      * [rtx reshim](#rtx-reshim)
      * [rtx self-update](#rtx-self-update)
      * [rtx settings get](#rtx-settings-get)
      * [rtx settings ls](#rtx-settings-ls)
      * [rtx settings set](#rtx-settings-set)
      * [rtx settings unset](#rtx-settings-unset)
      * [rtx shell](#rtx-shell)
      * [rtx uninstall](#rtx-uninstall)
      * [rtx version](#rtx-version)
      * [rtx where](#rtx-where)
      * [rtx which](#rtx-which)
   * [Comparison to asdf](#comparison-to-asdf)
      * [Performance](#performance)
      * [Environment variables](#environment-variables-1)
      * [UX](#ux)
      * [CI/CD](#cicd)
      * [GitHub Actions](#github-actions)
   * [Shims](#shims)
   * [direnv](#direnv)
      * [rtx inside of direnv (use rtx in .envrc)](#rtx-inside-of-direnv-use-rtx-in-envrc)
      * [Do you need direnv?](#do-you-need-direnv)
   * [Cache Behavior](#cache-behavior)
      * [Plugin Cache](#plugin-cache)
      * [Legacy File Cache](#legacy-file-cache)
<!--te-->

## About

rtx is a tool for managing programming language and tool versions. For example, use this to install
a particular version of node.js and ruby for a project. Using `rtx activate`, you can have your
shell automatically switch to the correct node and ruby versions when you `cd` into the project's
directory. Other projects on your machine can use a different set of versions.

rtx is inspired by [asdf](https://asdf-vm.com) and uses asdf's vast [plugin ecosystem](https://github.com/asdf-vm/asdf-plugins)
under the hood. However, it is _much_ faster than asdf and has a more friendly user experience.
For more on how rtx compares to asdf, [see below](#comparison-to-asdf). The goal of this project
was to create a better front-end to asdf.

It uses the same `.tool-versions` file that asdf uses. It's also compatible with idiomatic version
files like `.node-version` and `.ruby-version`. See [Legacy Version Files](#legacy-version-files) below.

Come chat about rtx on [discord](https://discord.gg/mABnUDvP57).

### What do I use this for?

Typically, developers would use rtx to manage versions of their dev tools for _local_ development.
The main purpose of using rtx is being able to have different versions of languages for different projects
on the same machine. (For example, one project might require python-3.10 and another python-3.11).

Using rtx in production is less common but still a supported use-case. Usually a production setup
won't have different directories for different projects with different dev tool requirements.
However using `.tool-versions`/`.rtx.toml` config in production provides parity with local development
so rtx is still definitely useful in production setups. See the [GitHub Action](#github-actions) for
an example of using rtx in production.

### How it works

rtx installs as a shell extension (e.g. `rtx activate zsh`) that sets the `PATH`
environment variable to point your shell to the correct runtime binaries. When you `cd` into a
directory containing a `.tool-versions` file, rtx will automatically activate the correct versions.

Every time your prompt starts it will call `rtx hook-env` to fetch new environment variables. This
should be very fast and it exits early if the the directory wasn't changed or the `.tool-versions`
files haven't been updated. On my machine this takes 4ms in the fast case, 14ms in the slow case. See [Performance](#performance) for more on this topic.

Unlike asdf which uses shim files to dynamically locate runtimes when they're called, rtx modifies
`PATH` ahead of time so the runtimes are called directly. This is not only faster since it avoids
any overhead, but it also makes it so commands like `which node` work as expected. This also
means there isn't any need to run `asdf reshim` after installing new runtime binaries.

rtx does not directly install runtimes. Instead, it uses asdf plugins to install runtimes. See
[plugins](#plugins) below.

### Common example commands

    rtx install nodejs@18.0.0       Install a specific version number
    rtx install nodejs@18.0         Install a fuzzy version number
    rtx local nodejs@18             Use node-18.x in current project
    rtx global nodejs@18            Use node-18.x as default

    rtx install nodejs              Install the version specified in .tool-versions
    rtx local nodejs@latest         Use latest node in current directory
    rtx global nodejs@system        Use system node as default

    rtx x nodejs@18 -- node app.js  Run `node app.js` with the PATH pointing to node-18.x

## Installation

### Standalone

Note that it isn't necessary for `rtx` to be on `PATH`. If you run the activate script in your rc
file, rtx will automatically add itself to `PATH`.

```sh-session
$ curl https://rtx.pub/install.sh | sh
```

or if you're allergic to `| sh`:

```sh-session
$ curl https://rtx.pub/rtx-latest-macos-arm64 > /usr/local/bin/rtx
```

It doesn't matter where you put it. So use `~/bin`, `/usr/local/bin`, `~/.local/share/rtx/bin/rtx`
or whatever.

Supported architectures:

- `x64`
- `arm64`

Supported platforms:

- `macos`
- `linux`

If you need something else, compile it with [cargo](#cargo).

### Homebrew

There are 2 ways to install rtx with Homebrew. The recommended method is to use
the custom tap which will always contain the latest release.

```sh-session
$ brew install jdxcode/tap/rtx
```

Alternatively, you can use the built-in tap (homebrew-core), which will be updated
once Homebrew maintainers merge the PR for a new release:

```sh-session
$ brew install rtx
```

### Cargo

Build from source with Cargo:

```sh-session
$ cargo install rtx-cli
```

Do it faster with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```sh-session
$ cargo install cargo-binstall
$ cargo binstall rtx-cli
```

Build from the latest commit in main:

```sh-session
$ cargo install rtx-cli --git https://github.com/jdxcode/rtx --branch main
```

### npm

rtx is available on npm as precompiled binaries. This isn't a node.js package, just distributed
via npm. It can be useful for JS projects that want to setup rtx via `package.json` or `npx`.

```sh-session
$ npm install -g rtx-cli
```

Or use npx if you just want to test it out for a single command without fully installing:

```sh-session
$ npx rtx-cli exec python@3.11 -- python some_script.py
```

### GitHub Releases

Download the latest release from [GitHub](https://github.com/jdxcode/rtx/releases).

```sh-session
$ curl https://github.com/jdxcode/rtx/releases/download/v1.20.4/rtx-v1.20.4-linux-x64 | tar -xJv
$ mv rtx/bin/rtx /usr/local/bin
```

### apt

For installation on Ubuntu/Debian:

```sh-session
wget -qO - https://rtx.pub/gpg-key.pub | gpg --dearmor | sudo tee /usr/share/keyrings/rtx-archive-keyring.gpg 1> /dev/null
echo "deb [signed-by=/usr/share/keyrings/rtx-archive-keyring.gpg arch=amd64] https://rtx.pub/deb stable main" | sudo tee /etc/apt/sources.list.d/rtx.list
sudo apt update
sudo apt install -y rtx
```

> **Warning**
>
> If you're on arm64 you'll need to run the following:
> ```
> echo "deb [signed-by=/usr/share/keyrings/rtx-archive-keyring.gpg arch=arm64] https://rtx.pub/deb stable main" | sudo tee /etc/apt/sources.list.d/rtx.list
> ```

### dnf

For Fedora, CentOS, Amazon Linux, RHEL and other dnf-based distributions:

```sh-session
dnf install -y dnf-plugins-core
dnf config-manager --add-repo https://rtx.pub/rpm/rtx.repo
dnf install -y rtx
```

### yum

```sh-session
yum install -y yum-utils
yum-config-manager --add-repo https://rtx.pub/rpm/rtx.repo
yum install -y rtx
```

### ~~apk~~ (coming soon)

For Alpine Linux:

```sh-session
apk add rtx --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/
```

### aur

For Arch Linux:

```sh-session
git clone https://aur.archlinux.org/rtx.git
cd rtx
makepkg -si
```

### nix

For NixOS or those using the Nix package manager:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rtx-flake = {
      url = "github:chadac/rtx/add-nix-flake";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rtx-flake }:
    flake-utils.lib.eachDefaultSystem(system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rtx-flake.overlay ];
        };
      in {
        devShells.default = pkgs.mkShell {
          name = "my-dev-env";
          nativeBuildInputs = with pkgs; [
            rtx
          ];
        };
      }
    );
}
```

You can also import the package directly using
`rtx-flake.packages.${system}.rtx`. It supports all default Nix
systems.

## Other Shells

### Bash

```sh-session
$ echo 'eval "$(rtx activate bash)"' >> ~/.bashrc
```

### Fish

```sh-session
$ echo 'rtx activate fish | source' >> ~/.config/fish/config.fish
```

### Xonsh

Since `.xsh` files are [not compiled](https://github.com/xonsh/xonsh/issues/3953) you may shave a bit off startup time by using a pure Python import: add the code below to, for example, `~/.config/xonsh/rtx.py` config file and `import rtx` it in `~/.config/xonsh/rc.xsh`:
```xsh
from pathlib        	import Path
from xonsh.built_ins	import XSH

ctx = XSH.ctx
rtx_init = subprocess.run([Path('~/bin/rtx').expanduser(),'activate','xonsh'],capture_output=True,encoding="UTF-8").stdout
XSH.builtins.execx(rtx_init,'exec',ctx,filename='rtx')
```

Or continue to use `rc.xsh`/`.xonshrc`:
```xsh
echo 'execx($(~/bin/rtx activate xonsh))' >> ~/.config/xonsh/rc.xsh # or ~/.xonshrc
```

Given that `rtx` replaces both shell env `$PATH` and OS environ `PATH`, watch out that your configs don't have these two set differently (might throw `os.environ['PATH'] = xonsh.built_ins.XSH.env.get_detyped('PATH')` at the end of a config to make sure they match)

### Something else?

Adding a new shell is not hard at all since very little shell code is
in this project.
[See here](https://github.com/jdxcode/rtx/tree/main/src/shell) for how
the others are implemented. If your shell isn't currently supported
I'd be happy to help you get yours integrated.

## Uninstalling

Use `rtx implode` to uninstall rtx. This will remove the rtx binary and all of its data. Use
`rtx implode --help` for more information.

Alternatively, manually remove the following directories to fully clean up:

* `~/.local/share/rtx` (can also be `RTX_DATA_DIR` or `XDG_DATA_HOME/rtx`)
* `~/.config/rtx` (can also be `RTX_CONFIG_DIR` or `XDG_CONFIG_HOME/rtx`)
* on Linux: `~/.cache/rtx` (can also be `RTX_CACHE_DIR` or `XDG_CACHE_HOME/rtx`)
* on macOS: `~/Library/Caches/rtx` (can also be `RTX_CACHE_DIR`)

## Configuration

### `.tool-versions`

The `.tool-versions` file is used to specify the runtime versions for a project. An example of this
is:

```
nodejs      18.0.0       # comments are allowed
ruby        3            # can be fuzzy version
shellcheck  latest       # also supports "latest"
jq          1.6
erlang      ref:master   # compile from vcs ref
golang      prefix:1.19  # uses the latest 1.19.x version—needed in case "1.19" is an exact match
shfmt       path:./shfmt # use a custom runtime
```

Create `.tool-versions` files manually, or use [`rtx local`](#rtx-local) to create them automatically.
See [the asdf docs](https://asdf-vm.com/manage/configuration.html#tool-versions) for more info on this file format.

### Legacy version files

rtx supports "legacy version files" just like asdf. They're language-specific files like `.node-version`
and `.python-version`. These are ideal for setting the runtime version of a project without forcing
other developers to use a specific tool like rtx/asdf.

They support aliases, which means you can have an `.nvmrc` file with `lts/hydrogen` and it will work
in rtx and nvm. Here are some of the supported legacy version files:

| Plugin    | "Legacy" (Idiomatic) Files                         |
| --------- | -------------------------------------------------- |
| crystal   | `.crystal-version`                                 |
| elixir    | `.exenv-version`                                   |
| golang    | `.go-version`, `go.mod`                            |
| java      | `.java-version`                                    |
| nodejs    | `.nvmrc`, `.node-version`                          |
| python    | `.python-version`                                  |
| ruby      | `.ruby-version`, `Gemfile`                         |
| terraform | `.terraform-version`, `.packer-version`, `main.tf` |
| yarn      | `.yvmrc`                                           |

In rtx these are enabled by default. You can disable them with `rtx settings set legacy_version_file false`.
There is a performance cost to having these when they're parsed as it's performed by the plugin in
`bin/parse-version-file`. However these are [cached](#cache-behavior) so it's not a huge deal.
You may not even notice.

> **Note**
>
> asdf calls these "legacy version files" so we do too. I think this is a bad name since it implies
> that they shouldn't be used—which is definitely not the case IMO. I prefer the term "idiomatic"
> version files since they're version files not specific to asdf/rtx and can be used by other tools.
> (`.nvmrc` being a notable exception, which is tied to a specific tool.)

### Global config: `~/.config/rtx/config.toml`

rtx can be configured in `~/.config/rtx/config.toml`. The following options are available (defaults shown):

```toml
[settings]
# whether to prompt to install plugins and runtimes if they're not already installed
missing_runtime_behavior = 'prompt' # other options: 'ignore', 'warn', 'prompt', 'autoinstall'

# plugins can read the versions files used by other version managers (if enabled by the plugin)
# for example, .nvmrc in the case of nodejs's nvm
legacy_version_file = true         # enabled by default (different than asdf)

# configure `rtx install` to always keep the downloaded archive
always_keep_download = false        # deleted after install by default

# configure how frequently (in minutes) to fetch updated plugin repository changes
# this is updated whenever a new runtime is installed
# (note: this isn't currently implemented but there are plans to add it: https://github.com/jdxcode/rtx/issues/128)
plugin_autoupdate_last_check_duration = 10080 # (one week) set to 0 to disable updates

verbose = false     # set to true to see full installation output, see `RTX_VERBOSE`
asdf_compat = false # set to true to ensure .tool-versions will be compatible with asdf, see `RTX_ASDF_COMPAT`
jobs = 4            # number of plugins or runtimes to install in parallel. The default is `4`.
raw = false         # set to true to directly pipe plugins to stdin/stdout/stderr

shorthands_file = '~/.config/rtx/shorthands.toml' # path to the shorthands file, see `RTX_SHORTHANDS_FILE`
disable_default_shorthands = false # disable the default shorthands, see `RTX_DISABLE_DEFAULT_SHORTHANDS`

experimental = false # enable experimental features such as shims
shims_dir = '~/.local/share/rtx/shims' # [experimental] directory where shims are stored

[alias.nodejs]
my_custom_node = '18'  # makes `rtx install nodejs@my_custom_node` install node-18.x
                       # this can also be specified in a plugin (see below in "Aliases")
```

These settings can also be managed with `rtx settings ls|get|set|unset`.

### [experimental] `.rtx.toml`

`.rtx.toml` is a new config file that replaces both the global config and the `.tool-versions` file.

It allows for functionality that is not possible with `.tool-versions`, such as:

* setting arbitrary env vars while inside the directory
* passing options to plugins like `virtualenv='.venv'` for [rtx-python](https://github.com/jdxcode/rtx-python#virtualenv-support).
* specifying plugin repo url for custom plugins so it does not need to be added manually

Here is what the config looks like:

```toml
[env]
NODE_ENV = 'production' # supports arbitrary env vars so rtx can be used like dotenv

[tools]
# specify single or multiple versions
terraform = '1.0.0'
erlang = ['23.3', '24.0']

# supports everything you can do with .tool-versions currently
nodejs = ['16', 'prefix:18', 'ref:master', 'path:~/.nodes/14']

# send arbitrary options to the plugin, passed as:
# RTX_TOOL_OPTS__VENV=.venv
# RTX_TOOL_OPTS__DEFAULT_PACKAGES__0=ansible
# RTX_TOOL_OPTS__DEFAULT_PACKAGES__1=pipenv
python = { version = '3.10', venv = '.venv', default_packages = ['ansible', 'pipenv'] }

[plugins]
# specify a custom repo url
# note this will only be used if the plugin does not already exist
python = 'https://github.com/jdxcode/rtx-python'

[settings] # project-local settings
verbose = true
missing_runtime_behavior = 'warn'
shims_dir = '~/.rtx/shims'

[alias.nodejs] # project-local aliases
my_custom_node = '18'
```

`.rtx.toml` is currently experimental and may change in minor versions of rtx.

### Environment variables

rtx can also be configured via environment variables. The following options are available:

#### `RTX_MISSING_RUNTIME_BEHAVIOR`

This is the same as the `missing_runtime_behavior` config option in `~/.config/rtx/config.toml`.

```sh-session
$ RTX_MISSING_RUNTIME_BEHAVIOR=ignore rtx install nodejs@18
$ RTX_NODEJS_VERSION=18 rtx exec -- node --version
```

#### `RTX_DATA_DIR`

This is the directory where rtx stores its data. The default is `~/.local/share/rtx`.

#### `RTX_CACHE_DIR`

This is the directory where rtx stores cache. The default is `~/.cache/rtx` on Linux and `~/Library/Caches/rtx` on macOS.

#### `RTX_CONFIG_FILE`

This is the path to the config file. The default is `~/.config/rtx/config.toml`.
(Or `$XDG_CONFIG_HOME/config.toml` if that is set)

#### `RTX_DEFAULT_TOOL_VERSIONS_FILENAME`

Set to something other than ".tool-versions" to have rtx look for configuration with alternate names.

#### `RTX_${PLUGIN}_VERSION`

Set the version for a runtime. For example, `RTX_NODEJS_VERSION=18` will use nodejs@18.x regardless
of what is set in `.tool-versions`.

#### `RTX_LEGACY_VERSION_FILE`

Plugins can read the versions files used by other version managers (if enabled by the plugin)
for example, .nvmrc in the case of nodejs's nvm.

#### `RTX_LOG_LEVEL=trace|debug|info|warn|error`

Can also use `RTX_DEBUG=1`, `RTX_TRACE=1`, and `RTX_QUIET=1`. These adjust the log
output to the screen.

#### `RTX_LOG_FILE=~/.rtx/rtx.log`

Output logs to a file.

#### `RTX_LOG_FILE_LEVEL=trace|debug|info|warn|error`

Same as `RTX_LOG_LEVEL` but for the log file output level. This is useful if you want
to store the logs but not have them litter your display.

#### `RTX_VERBOSE=1`

This shows the installation output during `rtx install` and `rtx plugin install`.
This should likely be merged so it behaves the same as `RTX_DEBUG=1` and we don't have
2 configuration for the same thing, but for now it is it's own config.

#### `RTX_ASDF_COMPAT=1`

Only output `.tool-versions` files in `rtx local|global` which will be usable by asdf.

#### `RTX_JOBS=1`

Set the number plugins or runtimes to install in parallel. The default is `4`.

#### `RTX_RAW=1`

Set to "1" to directly pipe plugin scripts to stdin/stdout/stderr. By default stdin is disabled
because when installing a bunch of plugins in parallel you won't see the prompt. Use this if a
plugin accepts input or otherwise does not seem to be installing correctly.

Sets `RTX_JOBS=1` because only 1 plugin script can be executed at a time.

#### `RTX_SHORTHANDS_FILE=~/.config/rtx/shorthands.toml`

Use a custom file for the shorthand aliases. This is useful if you want to share plugins within
an organization.

The file should be in toml format:

```toml
elixir = "https://github.com/my-org/rtx-elixir.git"
nodejs = "https://github.com/my-org/rtx-nodejs.git"
```

#### `RTX_DISABLE_DEFAULT_SHORTHANDS=1`

Disables the shorthand aliases for installing plugins. You will have to specify full urls when
installing plugins, e.g.: `rtx plugin install nodejs https://github.com/asdf-vm/asdf-nodejs.git`

Currently this disables the following:

* `--fuzzy` as default behavior (`rtx local nodejs@18` will save exact version)

#### `RTX_HIDE_OUTDATED_BUILD=1`

If a release is 12 months old, it will show a warning message every time it launches:

```
rtx has not been updated in over a year. Please update to the latest version.
```

You likely do not want to be using rtx if it is that old. I'm doing this instead of
autoupdating. If, for some reason, you want to stay on some old version, you can hide
this message with `RTX_HIDE_OUTDATED_BUILD=1`.

#### `RTX_EXPERIMENTAL=1`

Enables experimental features such as shims.

#### [experimental] `RTX_SHIMS_DIR=~/.local/share/rtx/shims`

Set a directory to output shims when running `rtx reshim`. Requires `experimental = true`.

## Aliases

rtx supports aliasing the versions of runtimes. One use-case for this is to define aliases for LTS
versions of runtimes. For example, you may want to specify `lts/hydrogen` as the version for nodejs@18.x.
So you can use the runtime with `nodejs lts/hydrogen` in `.tool-versions`.

User aliases can be created by adding an `alias.<PLUGIN>` section to `~/.config/rtx/config.toml`:

```toml
[alias.nodejs]
my_custom_18 = '18'
```

Plugins can also provide aliases via a `bin/list-aliases` script. Here is an example showing node.js
versions:

```bash
#!/usr/bin/env bash

echo "lts/hydrogen 18"
echo "lts/gallium 16"
echo "lts/fermium 14"
```

> **Note:**
>
> Because this is rtx-specific functionality not currently used by asdf it isn't likely to be in any
> plugin currently, but plugin authors can add this script without impacting asdf users.

## Plugins

rtx uses asdf's plugin ecosystem under the hood. These plugins contain shell scripts like
`bin/install` (for installing) and `bin/list-all` (for listing all of the available versions).

See https://github.com/asdf-vm/asdf-plugins for the list of built-in plugins shorthands. See asdf's
[Create a Plugin](https://asdf-vm.com/plugins/create.html) for how to create your own or just learn
more about how they work.

### Plugin Options

rtx has support for "plugin options" which is configuration specified in `.rtx.toml` to change behavior
of plugins. One example of this is virtualenv on python runtimes:

```toml
[tools]
python = {version='3.11', virtualenv='.venv'}
```

This will be passed to all plugin scripts as `RTX_TOOL_OPTS__VIRTUALENV=.venv`. The user can specify
any option and it will be passed to the plugin in that format.

Currently this only supports simple strings, but we can make it compatible with more complex types
(arrays, tables) fairly easily if there is a need for it.

## FAQs

### I don't want to put a `.tool-versions` file into my project since git shows it as an untracked file.

You can make git ignore these files in 3 different ways:

- Adding `.tool-versions` to project's `.gitignore` file. This has the downside that you need to commit the change to the ignore file.
- Adding `.tool-versions` to project's `.git/info/exclude`. This file is local to your project so there is no need to commit it.
- Adding `.tool-versions` to global gitignore (`core.excludesFile`). This will cause git to ignore `.tool-versions` files in all projects. You can explicitly add one to a project if needed with `git add --force .tool-versions`.

### rtx is failing or not working right

First try setting `RTX_DEBUG=1` or `RTX_TRACE=1` and see if that gives you more information.
You can also set `RTX_LOG_FILE_LEVEL=debug RTX_LOG_FILE=/path/to/logfile` to write logs to a file.

If something is happening with the activate hook, you can try disabling it and calling `eval "$(rtx hook-env)"` manually.
It can also be helpful to use `rtx env` which will just output environment variables that would be set.
Also consider using [shims](#shims) which can be more compatible.

If runtime installation isn't working right, try using the `--raw` flag which will install things in
series and connect stdin/stdout/stderr directly to the terminal. If a plugin is trying to interact
with you for some reason this will make it work.

Of course check the version of rtx with `rtx --version` and make sure it is the latest. Use `rtx self-update`
to update it. `rtx cache clean` can be used to wipe the internal cache and `rtx implode` can be used
to remove everything except config.

Before submitting a ticket, it's a good idea to test what you were doing with asdf. That way we can rule
out if the issue is with rtx or if it's with a particular plugin. For example, if `rtx install python@latest`
doesn't work, try running `asdf install python latest` to see if it's an issue with asdf-python.

Lastly, there is `rtx doctor` which will show diagnostic information and any warnings about issues
detected with your setup. If you submit a bug report, please include the output of `rtx doctor`.

### Windows support?

This is something we'd like to add! https://github.com/jdxcode/rtx/discussions/66

It's not a near-term goal and it would require plugin modifications, but it should be feasible.

### How do I use rtx with http proxies?

Short answer: just set `http_proxy` and `https_proxy` environment variables. These should be lowercase.

rtx doesn't really do anything with http itself. The only exception to that is checking for new versions
and `rtx self-update`. It uses `git` to clone plugins and the plugins themselves generally will download
files with `curl` or `wget`.

However this is really up to the plugin. If you're having a proxy-related issue installing something
you should post an issue on the plugin's repo.

### How do the shorthand plugin names map to repositories?

e.g.: how does `rtx plugin install nodejs` know to fetch [https://github.com/asdf-vm/asdf-nodejs](https://github.com/asdf-vm/asdf-nodejs)?

asdf maintains [an index](https://github.com/asdf-vm/asdf-plugins) of shorthands that rtx uses as a base.
This is regularly updated every time that rtx has a release. This repository is stored directly into
the codebase [here](./src/default_shorthands.rs). The bottom of that file contains modifications that
rtx makes. For example, we add `node` which points to the same plugin as `nodejs` and change `python`
to point to [rtx-python](https://github.com/jdxcode/rtx-python) which is a fork of [asdf-python](https://github.com/danhper/asdf-python)
with some rtx features like virtualenv support.

Over time I suspect that more plugins will be forked like rtx-python as we're able to offer more rtx-specific
enhancements.

### How do I migrate from asdf?

First, just install rtx with `rtx activate` like in the getting started guide and remove asdf from your
shell rc file.

Then you can just run `rtx install` in a directory with an asdf `.tool-versions` file and it will
install the runtimes. You could attempt to avoid this by copying the internal directory from asdf over
to rtx with `cp -r ~/.asdf ~/.local/share/rtx`. That _should_ work because they use the same structure,
however this isn't officially supported or regularly tested. Alternatively you can set `RTX_DATA_DIR=~/.asdf`
and see what happens.

If you need to switch to/from asdf or work in a project with asdf users, you can set [`RTX_ASDF_COMPAT=1`](#rtx_asdf_compat1).

### rtx isn't working with tmux

It's been reported that PATH doesn't work correctly with tmux. The fix seems to be calling `hook-env`
right after activating:

```bash
eval "$(rtx activate bash)"
eval "$(rtx hook-env)"
```

This can also be useful if you need to use a runtime right away in an rc file. The default behavior
of `rtx activate` is that it will only run `hook-env` when the shell is about to be displayed, not
immediately after activating. Not calling `hook-env` immediately appears to work better with direnv.

<!-- RTX:COMMANDS -->
## Commands

### `rtx activate`

```
Initializes rtx in the current shell

This should go into your shell's rc file.
Otherwise, it will only take effect in the current session.
(e.g. ~/.bashrc)

Usage: activate [OPTIONS] [SHELL_TYPE]

Arguments:
  [SHELL_TYPE]
          Shell type to generate the script for
          
          [possible values: bash, fish, xonsh, zsh]

Options:
      --status
          Show "rtx: <PLUGIN>@<VERSION>" message when changing directories

Examples:
    $ eval "$(rtx activate bash)"
    $ eval "$(rtx activate zsh)"
    $ rtx activate fish | source
    $ execx($(rtx activate xonsh))
```
### `rtx alias get`

```
Show an alias for a plugin

This is the contents of an alias.<PLUGIN> entry in ~/.config/rtx/config.toml

Usage: get <PLUGIN> <ALIAS>

Arguments:
  <PLUGIN>
          The plugin to show the alias for

  <ALIAS>
          The alias to show

Examples:
  $ rtx alias get nodejs lts/hydrogen
  18.0.0
```
### `rtx alias ls`

```
List aliases
Shows the aliases that can be specified.
These can come from user config or from plugins in `bin/list-aliases`.

For user config, aliases are defined like the following in `~/.config/rtx/config.toml`:

  [alias.nodejs]
  lts = "18.0.0"

Usage: ls [OPTIONS]

Options:
  -p, --plugin <PLUGIN>
          Show aliases for <PLUGIN>

Examples:
  $ rtx aliases
  nodejs    lts/hydrogen   18.0.0
```
### `rtx alias set`

```
Add/update an alias for a plugin

This modifies the contents of ~/.config/rtx/config.toml

Usage: set <PLUGIN> <ALIAS> <VALUE>

Arguments:
  <PLUGIN>
          The plugin to set the alias for

  <ALIAS>
          The alias to set

  <VALUE>
          The value to set the alias to

Examples:
  $ rtx alias set nodejs lts/hydrogen 18.0.0
```
### `rtx alias unset`

```
Clears an alias for a plugin

This modifies the contents of ~/.config/rtx/config.toml

Usage: unset <PLUGIN> <ALIAS>

Arguments:
  <PLUGIN>
          The plugin to remove the alias from

  <ALIAS>
          The alias to remove

Examples:
  $ rtx alias unset nodejs lts/hydrogen
```
### `rtx bin-paths`

```
List all the active runtime bin paths

Usage: bin-paths
```
### `rtx cache clear`

```
Deletes all cache files in rtx

Usage: clear
```
### `rtx complete`

```
Generate shell completions

Usage: complete --shell <SHELL>

Options:
  -s, --shell <SHELL>
          shell type
          
          [possible values: bash, elvish, fish, powershell, zsh]

Examples:
  $ rtx complete -s bash > /etc/bash_completion.d/rtx
  $ rtx complete -s zsh  > /usr/local/share/zsh/site-functions/_rtx
  $ rtx complete -s fish > ~/.config/fish/completions/rtx.fish
```
### `rtx current`

```
Shows current active and installed runtime versions

This is similar to `rtx ls --current`, but this only shows the runtime
and/or version. It's designed to fit into scripts more easily.

Usage: current [PLUGIN]

Arguments:
  [PLUGIN]
          Plugin to show versions of e.g.: ruby, nodejs

Examples:
  # outputs `.tool-versions` compatible format
  $ rtx current
  python 3.11.0 3.10.0
  shfmt 3.6.0
  shellcheck 0.9.0
  nodejs 18.13.0

  $ rtx current nodejs
  18.13.0

  # can output multiple versions
  $ rtx current python
  3.11.0 3.10.0
```
### `rtx deactivate`

```
Disable rtx for current shell session

This can be used to temporarily disable rtx in a shell session.

Usage: deactivate

Examples:
  $ rtx deactivate bash
  $ rtx deactivate zsh
  $ rtx deactivate fish
  $ execx($(rtx deactivate xonsh))
```
### `rtx direnv activate`

```
Output direnv function to use rtx inside direnv

See https://github.com/jdxcode/rtx#direnv for more information

Because this generates the legacy files based on currently installed plugins,
you should run this command after installing new plugins. Otherwise
direnv may not know to update environment variables when legacy file versions change.

Usage: activate

Examples:
  $ rtx direnv activate > ~/.config/direnv/lib/use_rtx.sh
  $ echo 'use rtx' > .envrc
  $ direnv allow
```
### `rtx doctor`

```
Check rtx installation for possible problems.

Usage: doctor

Examples:
  $ rtx doctor
  [WARN] plugin nodejs is not installed
```
### `rtx env`

```
Exports env vars to activate rtx a single time

Use this if you don't want to permanently install rtx. It's not necessary to
use this if you have `rtx activate` in your shell rc file.

Usage: env [OPTIONS] [RUNTIME]...

Arguments:
  [RUNTIME]...
          Runtime version to use

Options:
  -s, --shell <SHELL>
          Shell type to generate environment variables for
          
          [possible values: bash, fish, xonsh, zsh]

Examples:
  $ eval "$(rtx env -s bash)"
  $ eval "$(rtx env -s zsh)"
  $ rtx env -s fish | source
  $ execx($(rtx env -s xonsh))
```
### `rtx exec`

```
Execute a command with runtime(s) set

use this to avoid modifying the shell session or running ad-hoc commands with the rtx runtimes
set.

Runtimes will be loaded from .tool-versions, though they can be overridden with <RUNTIME> args
Note that only the plugin specified will be overridden, so if a `.tool-versions` file
includes "nodejs 18" but you run `rtx exec python@3.11`; it will still load nodejs@18.

The "--" separates runtimes from the commands to pass along to the subprocess.

Usage: exec [OPTIONS] [RUNTIME]... [-- <COMMAND>...]

Arguments:
  [RUNTIME]...
          Runtime(s) to start e.g.: nodejs@18 python@3.10

  [COMMAND]...
          Command string to execute (same as --command)

Options:
  -c, --command <C>
          Command string to execute

Examples:
  rtx exec nodejs@18 -- node ./app.js  # launch app.js using node-18.x
  rtx x nodejs@18 -- node ./app.js     # shorter alias

  # Specify command as a string:
  rtx exec nodejs@18 python@3.11 --command "node -v && python -V"
```
### `rtx global`

```
Shows/sets the global runtime version(s)

Displays the contents of ~/.tool-versions after writing.
The file is `$HOME/.tool-versions` by default.
Use `rtx local` to set a runtime version locally in the current directory.

Usage: global [OPTIONS] [RUNTIME]...

Arguments:
  [RUNTIME]...
          Runtime(s) to add to .tool-versions
          e.g.: nodejs@18
          If this is a single runtime with no version, the current value of the global
          .tool-versions will be displayed

Options:
      --pin
          Save exact version to `~/.tool-versions`
          e.g.: `rtx local --pin nodejs@18` will save `nodejs 18.0.0` to ~/.tool-versions

      --fuzzy
          Save fuzzy version to `~/.tool-versions`
          e.g.: `rtx local --fuzzy nodejs@18` will save `nodejs 18` to ~/.tool-versions
          this is the default behavior unless RTX_ASDF_COMPAT=1

      --remove <PLUGIN>
          Remove the plugin(s) from ~/.tool-versions

Examples:
  # set the current version of nodejs to 18.x
  # will use a fuzzy version (e.g.: 18) in .tool-versions file
  $ rtx global --fuzzy nodejs@18

  # set the current version of nodejs to 18.x
  # will use a precise version (e.g.: 18.0.0) in .tool-versions file
  $ rtx global --pin nodejs@18

  # show the current version of nodejs in ~/.tool-versions
  $ rtx global nodejs
  18.0.0
```
### `rtx implode`

```
Removes rtx CLI and all related data

Skips config directory by default.

Usage: implode [OPTIONS]

Options:
      --config
          Also remove config directory

      --dry-run
          List directories that would be removed without actually removing them
```
### `rtx install`

```
Install a runtime

This will install a runtime to `~/.local/share/rtx/installs/<PLUGIN>/<VERSION>`
It won't be used simply by being installed, however.
For that, you must set up a `.tool-version` file manually or with `rtx local/global`.
Or you can call a runtime explicitly with `rtx exec <PLUGIN>@<VERSION> -- <COMMAND>`.

Runtimes will be installed in parallel. To disable, set `--jobs=1` or `RTX_JOBS=1`

Usage: install [OPTIONS] [RUNTIME]...

Arguments:
  [RUNTIME]...
          Runtime(s) to install e.g.: nodejs@18

Options:
  -p, --plugin <PLUGIN>
          Only install runtime(s) for <PLUGIN>

  -f, --force
          Force reinstall even if already installed

  -v, --verbose...
          Show installation output

Examples:
  $ rtx install nodejs@18.0.0  # install specific nodejs version
  $ rtx install nodejs@18      # install fuzzy nodejs version
  $ rtx install nodejs         # install version specified in .tool-versions
  $ rtx install                # installs all runtimes specified in .tool-versions for installed plugins
  $ rtx install --all          # installs all runtimes and all plugins
```
### `rtx latest`

```
Gets the latest available version for a plugin

Usage: latest <RUNTIME>

Arguments:
  <RUNTIME>
          Runtime to get the latest version of

Examples:
  $ rtx latest nodejs@18  # get the latest version of nodejs 18
  18.0.0

  $ rtx latest nodejs     # get the latest stable version of nodejs
  20.0.0
```
### `rtx local`

```
Sets .tool-versions to include a specific runtime

then displays the contents of .tool-versions
use this to set the runtime version when within a directory
use `rtx global` to set a runtime version globally

Usage: local [OPTIONS] [RUNTIME]...

Arguments:
  [RUNTIME]...
          Runtimes to add to .tool-versions
          e.g.: nodejs@18
          if this is a single runtime with no version,
          the current value of .tool-versions will be displayed

Options:
  -p, --parent
          Recurse up to find a .tool-versions file rather than using the current directory only
          by default this command will only set the runtime in the current directory ("$PWD/.tool-versions")

      --pin
          Save exact version to `.tool-versions`
          e.g.: `rtx local --pin nodejs@18` will save `nodejs 18.0.0` to .tool-versions

      --fuzzy
          Save fuzzy version to `.tool-versions` e.g.: `rtx local --fuzzy nodejs@18` will save `nodejs 18` to .tool-versions This is the default behavior unless RTX_ASDF_COMPAT=1

      --remove <PLUGIN>
          Remove the plugin(s) from .tool-versions

Examples:
  # set the current version of nodejs to 18.x for the current directory
  # will use a precise version (e.g.: 18.0.0) in .tool-versions file
  $ rtx local nodejs@18

  # set nodejs to 18.x for the current project (recurses up to find .tool-versions)
  $ rtx local -p nodejs@18

  # set the current version of nodejs to 18.x for the current directory
  # will use a fuzzy version (e.g.: 18) in .tool-versions file
  $ rtx local --fuzzy nodejs@18

  # removes nodejs from .tool-versions
  $ rtx local --remove=nodejs

  # show the current version of nodejs in .tool-versions
  $ rtx local nodejs
  18.0.0
```
### `rtx ls`

```
List installed runtime versions

The "arrow (->)" indicates the runtime is installed, active, and will be used for running commands.
(Assuming `rtx activate` or `rtx env` is in use).

Usage: ls [OPTIONS]

Options:
  -p, --plugin <PLUGIN>
          Only show runtimes from [PLUGIN]

  -c, --current
          Only show runtimes currently specified in .tool-versions

Examples:
  $ rtx list
  -> nodejs     18.0.0 (set by ~/src/myapp/.tool-versions)
  -> python     3.11.0 (set by ~/.tool-versions)
     python     3.10.0

  $ rtx list --current
  -> nodejs     18.0.0 (set by ~/src/myapp/.tool-versions)
  -> python     3.11.0 (set by ~/.tool-versions)
```
### `rtx ls-remote`

```
List runtime versions available for install

note that these versions are cached for commands like `rtx install nodejs@latest`
however _this_ command will always clear that cache and fetch the latest remote versions

Usage: ls-remote <PLUGIN> [PREFIX]

Arguments:
  <PLUGIN>
          Plugin to get versions for

  [PREFIX]
          The version prefix to use when querying the latest version
          same as the first argument after the "@"

Examples:
  $ rtx ls-remote nodejs
  18.0.0
  20.0.0

  $ rtx ls-remote nodejs@18
  18.0.0
  18.1.0

  $ rtx ls-remote nodejs 18
  18.0.0
  18.1.0
```
### `rtx plugins install`

```
Install a plugin

note that rtx automatically can install plugins when you install a runtime
e.g.: `rtx install nodejs@18` will autoinstall the nodejs plugin

This behavior can be modified in ~/.config/rtx/config.toml

Usage: install [OPTIONS] [NAME] [GIT_URL]

Arguments:
  [NAME]
          The name of the plugin to install
          e.g.: nodejs, ruby
          Can specify multiple plugins: `rtx plugins install nodejs ruby python`

  [GIT_URL]
          The git url of the plugin

Options:
  -f, --force
          Reinstall even if plugin exists

  -a, --all
          Install all missing plugins
          This will only install plugins that have matching shorthands.
          i.e.: they don't need the full git repo url

  -v, --verbose...
          Show installation output

Examples:
  # install the nodejs via shorthand
  $ rtx install nodejs

  # install the nodejs plugin using a specific git url
  $ rtx install nodejs https://github.com/asdf-vm/asdf-nodejs.git

  # install the nodejs plugin using the git url only
  # (nodejs is inferred from the url)
  $ rtx install https://github.com/asdf-vm/asdf-nodejs.git
```
### `rtx plugins ls`

```
List installed plugins

Can also show remotely available plugins to install.

Usage: ls [OPTIONS]

Options:
  -a, --all
          List all available remote plugins
          Same as `rtx plugins ls-remote`

  -u, --urls
          Show the git url for each plugin
          e.g.: https://github.com/asdf-vm/asdf-nodejs.git

Examples:
  $ rtx plugins ls
  nodejs
  ruby

  $ rtx plugins ls --urls
  nodejs                        https://github.com/asdf-vm/asdf-nodejs.git
  ruby                          https://github.com/asdf-vm/asdf-ruby.git
```
### `rtx plugins ls-remote`

```
List all available remote plugins

These are fetched from https://github.com/asdf-vm/asdf-plugins

Examples:
  $ rtx plugins ls-remote


Usage: ls-remote [OPTIONS]

Options:
  -u, --urls
          Show the git url for each plugin e.g.: https://github.com/asdf-vm/asdf-nodejs.git

      --only-names
          Only show the name of each plugin by default it will show a "*" next to installed plugins
```
### `rtx plugins uninstall`

```
Removes a plugin

Usage: uninstall <PLUGIN>...

Arguments:
  <PLUGIN>...
          Plugin(s) to remove

Examples:
  $ rtx uninstall nodejs
```
### `rtx plugins update`

```
Updates a plugin to the latest version

note: this updates the plugin itself, not the runtime versions

Usage: update [OPTIONS] [PLUGIN]...

Arguments:
  [PLUGIN]...
          Plugin(s) to update

Options:
  -a, --all
          Update all plugins

Examples:
  $ rtx plugins update --all   # update all plugins
  $ rtx plugins update nodejs  # update only nodejs
```
### `rtx reshim`

```
[experimental] rebuilds the shim farm

this requires that the shims_dir is set

Usage: reshim

Examples:
  $ rtx settings set experimental true
  $ rtx settings set shims_dir ~/.rtx/shims
  $ rtx reshim
  $ ~/.rtx/shims/node -v
  v18.0.0
```
### `rtx self-update`

```
Updates rtx itself

Usage: self-update
```
### `rtx settings get`

```
Show a current setting

This is the contents of a single entry in ~/.config/rtx/config.toml

Note that aliases are also stored in this file
but managed separately with `rtx aliases get`

Usage: get <KEY>

Arguments:
  <KEY>
          The setting to show

Examples:
  $ rtx settings get legacy_version_file
  true
```
### `rtx settings ls`

```
Show current settings

This is the contents of ~/.config/rtx/config.toml

Note that aliases are also stored in this file
but managed separately with `rtx aliases`

Usage: ls

Examples:
  $ rtx settings
  legacy_version_file = false
```
### `rtx settings set`

```
Add/update a setting

This modifies the contents of ~/.config/rtx/config.toml

Usage: set <KEY> <VALUE>

Arguments:
  <KEY>
          The setting to set

  <VALUE>
          The value to set

Examples:
  $ rtx settings set legacy_version_file true
```
### `rtx settings unset`

```
Clears a setting

This modifies the contents of ~/.config/rtx/config.toml

Usage: unset <KEY>

Arguments:
  <KEY>
          The setting to remove

Examples:
  $ rtx settings unset legacy_version_file
```
### `rtx shell`

```
Sets a tool version for the current shell session

Only works in a session where rtx is already activated.

Usage: shell [RUNTIME]...

Arguments:
  [RUNTIME]...
          Runtime version(s) to use

Examples:
  $ rtx shell nodejs@18
  $ node -v
  v18.0.0
```
### `rtx uninstall`

```
Removes runtime versions

Usage: uninstall <RUNTIME>...

Arguments:
  <RUNTIME>...
          Runtime(s) to remove

Examples:
  $ rtx uninstall nodejs@18.0.0 # will uninstall specific version
  $ rtx uninstall nodejs        # will uninstall current nodejs version
```
### `rtx version`

```
Show rtx version

Usage: version
```
### `rtx where`

```
Display the installation path for a runtime

Must be installed.

Usage: where <RUNTIME>

Arguments:
  <RUNTIME>
          Runtime(s) to look up
          e.g.: ruby@3
          if "@<PREFIX>" is specified, it will show the latest installed version that matches the prefix
          otherwise, it will show the current, active installed version

Examples:
  # Show the latest installed version of nodejs
  # If it is is not installed, errors
  $ rtx where nodejs@18
  /home/jdx/.local/share/rtx/installs/nodejs/18.0.0

  # Show the current, active install directory of nodejs
  # Errors if nodejs is not referenced in any .tool-version file
  $ rtx where nodejs
  /home/jdx/.local/share/rtx/installs/nodejs/18.0.0
```
### `rtx which`

```
Shows the path that a bin name points to

Usage: which [OPTIONS] <BIN_NAME>

Arguments:
  <BIN_NAME>
          

Options:
      --plugin
          Show the plugin name instead of the path

      --version
          Show the version instead of the path

Examples:
  $ rtx which node
  /home/username/.local/share/rtx/installs/nodejs/18.0.0/bin/node
  $ rtx which node --plugin
  nodejs
  $ rtx which node --version
  18.0.0
```
<!-- RTX:COMMANDS -->


## Comparison to asdf

rtx is mostly a clone of asdf, but there are notable areas where improvements have been made.

### Performance

asdf made (what I consider) a poor design decision to use shims that go between a call to a runtime
and the runtime itself. e.g.: when you call `node` it will call an asdf shim file `~/.asdf/shims/node`,
which then calls `asdf exec`, which then calls the correct version of node.

These shims have terrible performance, adding ~120ms to every runtime call. rtx does not use shims and instead
updates `PATH` so that it doesn't have any overhead when simply calling binaries. These shims are the main reason that I wrote this. Note that in the demo gif at the top of this README
that `rtx` isn't actually used when calling `node -v` for this reason. The performance is
identical to running node without using rtx.

I don't think it's possible for asdf to fix these issues. The author of asdf did a great writeup
of [performance problems](https://stratus3d.com/blog/2022/08/11/asdf-performance/). asdf is written
in bash which certainly makes it challenging to be performant, however I think the real problem is the
shim design. I don't think it's possible to fix that without a complete rewrite.

rtx does call an internal command `rtx hook-env` every time the directory has changed, but because
it's written in Rust, this is very quick—taking ~10ms on my machine. 4ms if there are no changes, 14ms if it's
a full reload.

tl;dr: asdf adds overhead (~120ms) when calling a runtime, rtx adds a small amount of overhead (~10ms)
when the prompt loads.

### Environment variables

asdf only helps manage runtime executables. However, some tools are managed via environment variables
(notably Java which switches via `JAVA_HOME`). This isn't supported very well in asdf and requires
a separate shell extension just to manage.

However asdf _plugins_ have a `bin/exec-env` script that is used for exporting environment variables
like [`JAVA_HOME`](https://github.com/halcyon/asdf-java/blob/master/bin/exec-env). rtx simply exports
the environment variables from the `bin/exec-env` script in the plugin but places them in the shell
for _all_ commands. In asdf it only exports those commands when the shim is called. This means if you
call `java` it will set `JAVA_HOME`, but not if you call some Java tool like `mvn`.

This means we're just using the existing plugin script but because rtx doesn't use shims it can be
used for more things. It would be trivial to make a plugin that exports arbitrary environment
variables like [dotenv](https://github.com/motdotla/dotenv) or [direnv](https://github.com/direnv/direnv).

### UX

Some commands are the same in asdf but others have been changed. Everything that's possible
in asdf should be possible in rtx but may use slightly different syntax. rtx has more forgiving commands,
such as using fuzzy-matching, e.g.: `rtx install nodejs@18`. While in asdf you _can_ run
`asdf install nodejs latest:18`, you can't use `latest:18` in a `.tool-versions` file or many other places.
In `rtx` you can use fuzzy-matching everywhere.

asdf requires several steps to install a new runtime if the plugin isn't installed, e.g.:

```sh-session
$ asdf plugin add nodejs
$ asdf install nodejs latest:18
$ asdf local nodejs latest:18
```

In `rtx` this can all be done in a single step to set the local runtime version. If the plugin
and/or runtime needs to be installed it will prompt:

[![asciicast](https://asciinema.org/a/564031.svg)](https://asciinema.org/a/564031)

I've found asdf to be particularly rigid and difficult to learn. It also made strange decisions like
having `asdf list all` but `asdf latest --all` (why is one a flag and one a positional argument?).
`rtx` makes heavy use of aliases so you don't need to remember if it's `rtx plugin add nodejs` or
`rtx plugin install nodejs`. If I can guess what you meant, then I'll try to get rtx to respond
in the right way.

That said, there are a lot of great things about asdf. It's the best multi-runtime manager out there
and I've really been impressed with the plugin system. Most of the design decisions the authors made
were very good. I really just have 2 complaints: the shims and the fact it's written in Bash.

### CI/CD

Using rtx in CI/CD is a great way to synchronize tool versions for dev/build.

### GitHub Actions

Use [`jdxcode/rtx-action`](https://github.com/jdxcode/rtx-action):

```yaml
- uses: jdxcode/rtx-action@v1
- run: node -v # will be the node version from `.tool-versions`
```

## Shims

While the PATH design of rtx works great in most cases, there are some situations where shims are
preferable. One example is when calling rtx binaries from an IDE.

To support this, there is experimental support for using rtx in a "shim" mode. To use:

```
$ rtx settings set experimental true
$ rtx settings set shims_dir ~/.rtx/shims
$ rtx i nodejs@18.0.0
$ rtx reshim
$ ~/.rtx/shims/node -v
v18.0.0
```

## direnv

[direnv](https://direnv.net) and rtx both manage environment variables based on directory. Because they both analyze
the current environment variables before and after their respective "hook" commands are run, they can conflict with each other.
As a result, there were a [number of issues with direnv](https://github.com/jdxcode/rtx/issues/8).
However, we think we've mitigated these. If you find that rtx and direnv are not working well together,
please comment on that ticket ideally with a good description of your directory layout so we can
reproduce the problem.

If there are remaining issues, they're likely to do with the ordering of PATH. This means it would
really only be a problem if you were trying to manage the same runtime with direnv and rtx. For example,
you may use `layout python` in an `.envrc` but also be maintaining a `.tool-versions` file with python
in it as well.

A more typical usage of direnv would be to set some arbitrary environment variables, or add unrelated
binaries to PATH. In these cases, rtx will not interfere with direnv.

### rtx inside of direnv (`use rtx` in `.envrc`)

If you do encounter issues with `rtx activate`, or just want to use direnv in an alternate way,
this is a simpler setup that's less likely to cause issues—at the cost of functionality.

This may be required if you want to use direnv's `layout python` with rtx. Otherwise there are
situations where rtx will override direnv's PATH. `use rtx` ensures that direnv always has control.

To do this, first use `rtx` to build a `use_rtx` function that you can use in `.envrc` files:

```sh-session
$ rtx direnv activate > ~/.config/direnv/lib/use_rtx.sh
```

Now in your `.envrc` file add the following:

```sh-session
use rtx
```

direnv will now call rtx to export its environment variables. You'll need to make sure to add `use_rtx`
to all projects that use rtx (or use direnv's `source_up` to load it from a subdirectory). You can also add `use rtx` to `~/.config/direnv/direnvrc`.

Note that in this method direnv typically won't know to refresh `.tool-versions` files
unless they're at the same level as a `.envrc` file. You'll likely always want to have
a `.envrc` file next to your `.tool-versions` for this reason. To make this a little
easier to manage, I encourage _not_ actually using `.tool-versions` at all, and instead
setting environment variables entirely in `.envrc`:

```
export RTX_NODEJS_VERSION=18.0.0
export RTX_PYTHON_VERSION=3.11
```

Of course if you use `rtx activate`, then these steps won't have been necessary and you can use rtx
as if direnv was not used.

If you continue to struggle, you can also try using the [experimental shims feature](#shims).

### Do you need direnv?

While making rtx compatible with direnv is, and will always be a major goal of this project, I also
want rtx to be capable of replacing direnv if needed. This is why rtx includes support for managing
env vars and virtualenv for python using `.rtx.toml`.

If you find you continue to need direnv, please open an issue and let me know what it is to see if
it's something rtx could support. rtx will never be as capable as direnv with a DSL like `.envrc`,
but I think we can handle enough common use cases to make that unnecessary for most people.

## Cache Behavior

rtx makes use of caching in many places in order to be efficient. The details about how long to keep
cache for should eventually all be configurable. There may be gaps in the current behavior where
things are hardcoded but I'm happy to add more settings to cover whatever config is needed.

Below I explain the behavior it uses around caching. If you're seeing behavior where things don't appear
to be updating, this is a good place to start.

### Plugin Cache

Each plugin has a cache that's stored in `~/$RTX_CACHE_DIR/plugins/<PLUGIN>`. It stores
the list of versions available for that plugin (`rtx ls-remote <PLUGIN>`), the legacy filenames (see below),
the list of aliases, and the bin directories within each runtime installation.

Remote versions are updated daily by default or anytime that `rtx ls-remote` is called explicitly. The file is
zlib messagepack, if you want to view it you can run the following (requires [msgpack-cli](https://github.com/msgpack/msgpack-cli)).

```sh-session
cat ~/$RTX_CACHE_DIR/nodejs/remote_versions.msgpack.z | perl -e 'use Compress::Raw::Zlib;my $d=new Compress::Raw::Zlib::Inflate();my $o;undef $/;$d->inflate(<>,$o);print $o;' | msgpack-cli decode
```

### Legacy File Cache

If enabled, rtx will read the legacy filenames such as `.node-version` for
[asdf-nodejs](https://github.com/asdf-vm/asdf-nodejs). This leverages cache in 2 places where the
plugin is called:

- [`list-legacy-filenames`](https://github.com/asdf-vm/asdf-nodejs/blob/master/bin/list-legacy-filenames)
    In every plugin I've seen this simply returns a static list of filenamed like ".nvmrc .node-version".
    It's cached alongside the standard "runtime" cache which is refreshed daily by default.
- [`parse-legacy-file`](https://github.com/asdf-vm/asdf-nodejs/blob/master/bin/parse-legacy-file)
    This plugin binary is called to parse a legacy file to get the version out of it. It's relatively
    expensive so every file that gets parsed as a legacy file is cached into `~/.local/share/rtx/legacy_cache`.
    It will remain cached until the file is modified. This is a simple text file that has the path to the
    legacy file stored as a hash for the filename.
