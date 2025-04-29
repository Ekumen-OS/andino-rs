# DevContainers

- [DevContainers](#devcontainers)
  - [Rust-Dev](#rust-dev)
      - [Image Details](#image-details)
      - [Getting Started](#getting-started)

## Rust-Dev

#### Image Details

* Base Image: `jammy` (ubuntu)
* Rust:
  * Installed via rustup.
  * The rustc version is configured via `./rust-dev/devcontainer.json` [1]

[1] Override if necessary. At a later date, we might configure the bazel version via a marker file at the project root.

#### Getting Started

Locally:

* [Install VSCode](https://code.visualstudio.com/docs/setup/linux#_debian-and-ubuntu-based-distributions)
* Install [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
* Open the project in VSCode
* CTRL-SHIFT-P &rarr; Reopen in Container
* Open a terminal in the container and run

```
(docker) dev@rust-dev:/workspaces/andino-rs$ cargo build
```

CodeSpaces:

* Go to Codespaces
* Select `New with Options`
* Select `Rust Dev` from the `Dev Container Configuration`

* Open a terminal in the container and run

```
@<github-username> ➜ /workspaces/andino-rs (main) $ cargo build
```
