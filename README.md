# iic
🗂️ I index content (iic) is a content indexer using paging

## Example
```RUST
let content: Vec<u8> = vec![0; 100];
let table: Table = index(&content, 10);

for page in &table.pages {
    assert_eq!(page.content.len(), 10);
}

assert_eq!(table.pages.len(), 10);

for page in &table.pages {
    assert_eq!(page.content.len(), 10);
}
```
