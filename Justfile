set shell := ["bash", "-uc"]

# Build & Test

build:
    cargo build

build-release:
    cargo build --release

test:
    cargo test --lib

test-all:
    cargo test --all

# Linting & Formatting

lint:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Security & Auditing

audit:
    cargo audit

# Documentation (mdBook)

install-mdbook:
    which mdbook || cargo install mdbook

build-book: install-mdbook
    cd book && mdbook build

serve-book: install-mdbook
    cd book && mdbook serve --open

# Changelog

install-git-cliff:
    which git-cliff || cargo install git-cliff

changelog: install-git-cliff
    git-cliff --config cliff.toml -o CHANGELOG.md

# Release workflow (called by CI)

pre-release: test-all lint audit build-release build-book

publish:
    cargo publish --no-verify --allow-dirty

# CI Simulation (local pre-flight checks)

ci: test-all lint audit build-release build-book dry-publish

dry-publish:
    cargo publish --dry-run --allow-dirty

simulate-release tag:
    git fetch --tags
    git checkout tags/{{tag}}
    cargo set-version $(echo {{tag}} | sed 's/^v//')
    just pre-release
    cargo publish --dry-run
