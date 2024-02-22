
use serde::{Deserialize, Serialize};
use super::calculate::calculate_height;


#[derive(Debug, Deserialize, Serialize)]
pub struct InputData {
    pub headerRow: HeaderData,
    pub dataRows: RowData,
    pub pageWidth: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeaderData {
    pub fontSize: usize,
    pub title: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RowData {
    pub fontSize: usize,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cell {
    pub cell_content: String,
}

impl Cell {
    pub fn new(cell_content: String) -> Cell {
        Cell { cell_content }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RowType {
    HeaderRow,
    DataRows,
}

#[derive(Debug, Serialize)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub row_height: usize,
    pub row_width: usize,
    pub row_type: RowType,
}

impl Row {
    // pub fn calculate_max_height(cells: &Vec<Cell>) -> usize {
    //     let mut max_height: usize = 0;
    //     for i in 0..cells.len() {
    //         if cells[i].cell_height > max_height {
    //             max_height = cells[i].cell_height;
    //         }
    //     }
    //     max_height
    // }

    pub fn new(
        mut cells: Vec<Cell>,
        row_type: RowType,
        size: usize,
        pagewidth: usize,
        cell_count: &usize,
    ) -> Row {
        println!("{}", cell_count);
        let cell_width = (pagewidth - 10 - 10) / *cell_count;
        let row_width = pagewidth - 10;
        let mut row_height = 0;
        for content in &mut cells {
            let cell_height = calculate_height(&mut content.cell_content, &size, cell_width);
            row_height = std::cmp::max(row_height, cell_height);
            println!("{} : {}", cell_height, row_height);
        }
        println!("{}", row_height);
        Row {
            cells,
            row_height,
            row_width,
            row_type,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub Rows: Vec<Row>,
    pub table_height: usize,
    pub table_width: usize,
}

impl Table {
    pub fn new(Rows: Vec<Row>) -> Table {
        let mut table_height = 0;
        let table_width: usize = Rows.len();
        for i in 0..Rows.len() {
            table_height += Rows[i].row_height
        }
        Table {
            Rows,
            table_height,
            table_width,
        }
    }
}

