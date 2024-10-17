pub mod lib;
pub mod structs;

fn main() {
    let serializer: lib::serializer::Serializer = lib::serializer::Serializer;
    let mut table: structs::table::Table = structs::table::Table {
        pages: Vec::new()
    };

    for i in 1..100 {
        table.pages.push(
            structs::page::Page {
                value: i,
                offset: i + 100
            }
        );
    }

    lib::io::File::write("out.ic", &serializer.serialize(table));

    let data: Vec<u8> = lib::io::File::open("out.ic").unwrap();
    let table_result: structs::table::Table = serializer.deserialize(&data);

    println!("{}", table_result.pages.len());
    for page in table_result.pages.iter() {
        println!("{}: {}", page.value, page.offset);
    }

}
