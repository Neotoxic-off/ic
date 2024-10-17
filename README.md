# ic
🗂️ Format made for content indexing using pagging

## Usage

#### Save
```RUST
use ic;

fn save() {
    let output: &str = "example.ic";
    let table: ic::structs::table::Table = build_table();
    let _ = save(output, table);

    assert_eq!(ic::io::File::exists(&output.to_owned()), true);
}
```

#### Open
```RUST
use ic;

fn open() {
    let table: ic::structs::table::Table = open(OUTPUT);

    assert_eq!(table.pages.len(), 10);
}
```

#### Load
```RUST
fn load() {
    let bytes: Vec<u8> = io::File::open(OUTPUT).unwrap();
    let table: ic::structs::table::Table = load(&bytes);

    assert_eq!(table.pages.len(), 10);
}
```
