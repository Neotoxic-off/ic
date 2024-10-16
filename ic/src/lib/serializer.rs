use crate::structs::table;
use crate::structs::page;
pub struct Serializer;

impl Serializer {
    pub fn serialize(&self, table: table::Table) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(name_bytes.len() as u8);
        bytes.extend_from_slice(name_bytes);

        bytes.push(self.age);

        bytes
    }

    fn get_table_pages(&self, pages: Vec<page::Page>) -> Vec<u8> {
        let table_page: Vec<u8> = 

        table_page
    }

    pub fn deserialize(&self, raw: Vec<u8>) -> table::Table {

    }
}
