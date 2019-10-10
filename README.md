# cargo-dockerimage
A Cargo Plugin for Rust to give support to Docker with Rust Programming.
This plugin is supposed to create a docker file first and then build a docker image from the created docker file.
As of now, this plugin works only for ubuntu (Linux Distribution).

## Install

You can install this plugin by using install command of Cargo.

```
$ cargo install cargo-dockerimage
```

To upgrade:

```
$ cargo install --force cargo-dockerimage
```

## Usage

```
$ cargo dockerimage docker_image_name
```

# Example:

```
# Build Docker Image

# To create binary(executable)
$ cargo build
# To create and build docker image. [Note: You need to run cargo build first]
$ cargo dockerimage test

```
# Contribution

We thrive for the best and want you to contribute towards a better Project. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for giving your valuable feedback and contributions.

# License

Apache License, Version 2.0 (see [LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>)

