_:
    just --list

test:
    cargo test --all-features
    pytest -v bindings/python