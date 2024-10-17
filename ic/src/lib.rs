mod io;
mod structs;
mod serializer;

pub fn save(path: &str, table: structs::table::Table) -> std::io::Result<()> {
    let serializer: serializer::Serializer = serializer::Serializer;
    let serialized: Vec<u8> = serializer.serialize(table);

    io::File::write(path, &serialized)
}

pub fn load(data: &[u8]) -> structs::table::Table {
    let serializer: serializer::Serializer = serializer::Serializer;

    serializer.deserialize(data)
}

pub fn open(path: &str) -> structs::table::Table {
    let serializer: serializer::Serializer = serializer::Serializer;
    let raw: Vec<u8> = io::File::open(path).unwrap();

    serializer.deserialize(&raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    static OUTPUT: &str = "out.ic";

    fn build_table() -> structs::table::Table {
        let mut table: structs::table::Table = structs::table::Table {
            pages: Vec::new()
        };

        for i in 0..10 {
            table.pages.push(
                structs::page::Page {
                    value: i,
                    offset: i + 10
                }
            );
        }

        table
    }

    #[test]
    fn save_test() {
        let table: structs::table::Table = build_table();
        let _ = save(OUTPUT, table);

        assert_eq!(io::File::exists(&OUTPUT.to_owned()), true);
    }

    #[test]
    fn open_test() {
        let _skip_table: structs::table::Table = build_table();
        let _ = save(OUTPUT, _skip_table);

        let table: structs::table::Table = open(OUTPUT);

        assert_eq!(table.pages.len(), 10);
    }

    #[test]
    fn load_test() {
        let _skip_table: structs::table::Table = build_table();
        let _ = save(OUTPUT, _skip_table);
    
        let bytes: Vec<u8> = io::File::open(OUTPUT).unwrap();
        let table: structs::table::Table = load(&bytes);

        assert_eq!(table.pages.len(), 10);
    }
}
