use crate::structs::table;
use crate::structs::page;

pub struct Serializer;

impl Serializer {
    pub fn serialize(&self, table: table::Table) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.get_table_pages(table.pages);

        bytes
    }

    fn get_table_pages(&self, pages: Vec<page::Page>) -> Vec<u8> {
        let mut table_page: Vec<u8> = vec![
            pages.len() as u8,
        ];

        for page in pages.iter() {
            table_page.extend_from_slice(
                &[page.value, page.offset]
            );
        }

        table_page
    }

    pub fn deserialize(&self, data: &[u8]) -> table::Table {
        let pages = self.get_table_pages_from_bytes(data);
        table::Table { pages }
    }
    
    fn get_table_pages_from_bytes(&self, data: &[u8]) -> Vec<page::Page> {
        let mut pages = Vec::new();
        let num_pages: usize = data[0] as usize;

        let mut index = 1;
        for _ in 0..num_pages {
            let value = data[index];
            let offset = data[index + 1];
            pages.push(page::Page { value, offset });
            index += 2;
        }
    
        pages
    }
}
