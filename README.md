# maturin-sample

- [maturin-sample](#maturin-sample)
	- [Install by wheel (Python >= 3.6)](#install-by-wheel-python--36)
	- [Install by pip (Python >= 3.6, pip >= 21.1)](#install-by-pip-python--36-pip--211)
	- [Build (For developers)](#build-for-developers)
	- [Test](#test)

## Install by wheel (Python >= 3.6)

```sh
python -m pip install maturin==0.11.0.b4
maturin build -i python
pip install target/wheels/<built .whl>
```

## Install by pip (Python >= 3.6, pip >= 21.1)

```sh
pip install . --use-feature=in-tree-build
```

## Build (For developers)

First, create a virtualenv using `poetry`:

```sh
python -m pip install poetry
potery install
poetry shell
```

Then compile the bindings module:

```sh
maturin develop
```

Test that it works as follows:

```sh
python -c "import maturin_sample; maturin_sample.hello([3, 1, 2])"
```

## Test

```sh
python -m unittest
```
