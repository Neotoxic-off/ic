use crate::structs::page::Page;

pub struct Table {
    pub pages: Vec<Page<usize>>,
    pub page_size: usize
}
