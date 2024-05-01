# rust-template

> My opinionated Rust template

## Getting Started

1. Install Cargo Binstall

https://github.com/cargo-bins/cargo-binstall#installation

```console
$ curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# or if you dont like piping random crap into your shell
$ cargo install cargo-binstall
```

2. Install Cargo Make

https://github.com/sagiegurari/cargo-make

```console
$ cargo binstall -y cargo-make
```

3. Initialize the project

```console
$ makers setup
```

## Running and testing the project

```console
$ makers dev
```

```console
$ makers test
```

## Linting and formatting

Effectful formatting and linting was merged into one task:

```console
$ makers format
```

You can run `makers lint` just to spit out errors and warnings.

```console
$ makers lint
```

## Deploying

[Dockerfile](./Dockerfile) compiles the project to MUSL for a minimal image.

You can build it with:

```console
$ docker buildx build -f ./Dockerfile .
```

But the easies way is with [Docker Compose](https://docs.docker.com/compose/).

```console
$ docker compose up --build
```

Additionally, there's a [GitHub Action](./.github/workflows/deploy.yml) that builds and pushes the image to [GitHub Container Registry](https://ghcr.io) on every push to `main` after you uncomment the `on` trigger.

## Integration

The testing [GitHub Action](./.github/workflows/test.yml) runs on every push to `main` and `pull_request` and formats, tests and builds the project in that order.
