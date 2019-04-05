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

## Exercises

- [x] hello-world (core) - 1
- [ ] leap  - 1
- [ ] raindrops  - 1
- [ ] nth-prime  - 1
- [ ] beer-song  - 1
- [ ] proverb  - 1
- [ ] difference-of-squares  - 1
- [ ] sum-of-multiples  - 1
- [ ] grains  - 1
- [ ] prime-factors  - 1
- [ ] armstrong-numbers  - 1
- [ ] reverse-string (core) - 1
- [ ] gigasecond (core) - 1
- [ ] bob (core) - 1
- [ ] bracket-push  - 1
- [ ] clock (core) - 4
- [ ] dot-dsl  - 4
- [ ] simple-linked-list  - 4
- [ ] pascals-triangle  - 4
- [ ] paasio  - 4
- [ ] nucleotide-count  - 4
- [ ] etl  - 4
- [ ] acronym  - 4
- [ ] sieve  - 4
- [ ] rna-transcription  - 4
- [ ] triangle  - 4
- [ ] grade-school  - 4
- [ ] binary-search  - 4
- [ ] robot-simulator  - 7
- [ ] queen-attack  - 4
- [ ] bowling  - 4
- [ ] tournament  - 4
- [ ] alphametics  - 4
- [ ] two-bucket  - 4
- [ ] spiral-matrix  - 4
- [ ] palindrome-products  - 4
- [ ] saddle-points  - 4
- [ ] isogram  - 4
- [ ] say  - 4
- [ ] run-length-encoding  - 4
- [ ] isbn-verifier  - 4
- [ ] perfect-numbers  - 4
- [ ] hamming  - 4
- [ ] scrabble-score  - 4
- [ ] pangram  - 4
- [ ] all-your-base  - 4
- [ ] allergies  - 4
- [ ] variable-length-quantity  - 4
- [ ] pig-latin  - 4
- [ ] atbash-cipher (core) - 4
- [ ] crypto-square  - 4
- [ ] rotational-cipher  - 4
- [ ] simple-cipher  - 4
- [ ] rail-fence-cipher  - 4
- [ ] anagram (core) - 4
- [ ] protein-translation  - 7
- [ ] robot-name  - 4
- [ ] ocr-numbers  - 10
- [ ] react  - 10
- [ ] space-age (core) - 7
- [ ] wordy  - 4
- [ ] sublist (core) - 7
- [ ] custom-set  - 4
- [ ] minesweeper (core) - 7
- [ ] rectangles  - 10
- [ ] circular-buffer  - 10
- [ ] luhn (core) - 7
- [ ] luhn-from  - 4
- [ ] luhn-trait  - 4
- [ ] largest-series-product  - 4
- [ ] word-count  - 4
- [ ] phone-number  - 4
- [ ] diamond  - 4
- [ ] accumulate  - 4
- [ ] roman-numerals  - 4
- [ ] pythagorean-triplet  - 7
- [ ] series  - 1
- [ ] collatz-conjecture  - 1
- [ ] diffie-hellman  - 1
- [ ] parallel-letter-frequency (core) - 10
- [ ] macros (core) - 10
- [ ] poker (core) - 10
- [ ] grep  - 7
- [ ] scale-generator  - 7
- [ ] decimal  - 7
- [ ] book-store  - 7
- [ ] dominoes  - 10
- [ ] forth (core) - 10

[ci-badge]: https://img.shields.io/circleci/project/github/brainpower/rust-exercism-cubicle/master.svg?style=flat-square
[ci-url]: https://circleci.com/gh/brainpower/rust-exercism-cubicle

[docker-badge]: https://img.shields.io/docker/cloud/build/brainpower/rust-exercism-cubicle.svg?label=docker&style=flat-square
[docker-url]: https://cloud.docker.com/u/brainpower/repository/docker/brainpower/rust-exercism-cubicle