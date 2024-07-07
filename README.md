# ✂️ Git Sweep

## Overview

`git-sweep` is a command-line tool designed to help developers maintain a clean and organized Git repository by removing all non-current selected branches from a local Git repo. This tool automates the process of deleting outdated branches, ensuring that your repository stays tidy and manageable.

You will find everything you need to install and use the `git-sweep` tool by following our sections:

- [Installation](#toolbox-installation)
- [Getting Started](#rocket-getting-started)
- [Features](#bulb-features)
- [License](#page_with_curl-license)

## :toolbox: Installation

### Install from package

Pre-built packages for Windows, macOS, and Linux are found on the [Releases](https://github.com/julienandreu/git-sweep/releases/) page.

### Using Homebrew

```sh
brew tap julienandreu/tap
brew install git-sweep
```

### Manual Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/julienandreu/git-sweep.git
   ```

2. Build the project using Cargo:

   ```sh
   cd git-sweep
   cargo build --release
   ```

3. Add the binary to your PATH:

   ```sh
   export PATH=$PATH:/path/to/git-sweep/target/release
   ```

## :rocket: Getting Started

This tool is designed to run on any system with Git installed.

### Requirements

- [Git](https://github.com/git-guides/install-git)

### Usage

Navigate to your preferred Git repository and run the `git-sweep` command:

```shell
git-sweep
```

## :bulb: Features

- **Automatic Fetching**: Runs `git fetch` to ensure you have the latest branch information from the remote repository.
- **Branch Listing**: Lists all branches in the repository and identifies the non-current branches.
- **Batch Deletion**: Deletes all non-current branches in one go, reducing clutter and keeping your repository clean.
- **Performance Tracking**: Provides feedback on the operation's duration, so you know exactly how long the cleanup took.

## :page_with_curl: License

This project is licensed under the Apache 2 License.
