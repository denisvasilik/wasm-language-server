## Usage

First, you need to have `cargo-fuzz` installed.

### Listing the fuzz targets

List the different fuzz targets with the following command:

```bash
cargo fuzz list
```

### Running the fuzz target

Run the fuzz target with the following command:

```bash
cargo +nightly fuzz run --features "fuzz" FUZZ_TARGET_NAME 1> /dev/null
```

Currently this only works on linux targets.
