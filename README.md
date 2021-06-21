# maturin-sample

## Build

```sh
cargo build
```

## Run

```sh
python -m venv venv
. venv/bin/activate
python -m pip install maturin
python -c "import maturin_sample; maturin_sample.hello([3,1,2])"
```

## Test

```sh
python -m unittest
```
