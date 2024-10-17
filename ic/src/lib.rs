mod structs;

use structs::page::Page;
use structs::table::Table;

pub fn index<T>(content: &[T], page_size: usize) -> Table {
    let mut current_page_value: usize = 0;
    let mut content_buffer: Vec<usize> = Vec::new();
    let mut table: Table = Table {
        pages: Vec::new(),
        page_size: page_size
    };

    for element in content {
        content_buffer.push(&*element as *const T as usize);

        if content_buffer.len() == table.page_size {
            table.pages.push(
                Page {
                    value: current_page_value,
                    content: content_buffer.split_off(0)
                }
            );
            current_page_value += 1;
        }
    }

    if !content_buffer.is_empty() {
        table.pages.push(
            Page {
                value: current_page_value,
                content: content_buffer
            }
        );
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn index_u8_test() {
        let content: Vec<u8> = vec![0; 100];
        let table: Table = index(&content, 10);
    
        for page in &table.pages {
            assert_eq!(page.content.len(), 10);
        }

        assert_eq!(table.pages.len(), 10);

        for page in &table.pages {
            assert_eq!(page.content.len(), 10);
        }
    }
}
