# rust-exercism-cubicle 

Work on the Rust track of exercism in a turn-key environment.

[![][ci-badge]][ci-url]
[![][docker-badge]][docker-url]

## Prerequesites

- docker
- docker-compose

## Getting started

```sh
docker-compose pull
docker-compose up
```

## Run tests for one package

```
cargo test -p 'hello-world'
```

## Watch tests for one package

```
cargo watch -x 'test -p hello-world'
```

[ci-badge]: https://img.shields.io/circleci/project/github/sinnerschrader/rust-exercism-cubicle/master.svg?style=flat-square
[ci-url]: https://circleci.com/gh/sinnerschrader/rust-exercism-cubicle

[docker-badge]: https://img.shields.io/docker/cloud/build/marionebl/rust-exercism-cubicle.svg?label=docker&style=flat-square
[docker-url]: https://cloud.docker.com/u/marionebl/repository/docker/marionebl/rust-exercism-cubicle