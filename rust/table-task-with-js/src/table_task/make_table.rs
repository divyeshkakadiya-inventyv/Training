use std::{fs, ops::Index};
// use serde_json::from_str;
use super::structs::{Cell, HeaderData, InputData, Row, RowType, Table};

use super::calculate;

///making table structure with help of Row and cells with calculation of height according to cells content length and finally write it into the json file
pub fn make(data:&str)->String {
   
            let data: Result<InputData, serde_json::Error> = serde_json::from_str(&data);
            match data {
                Ok(data) => {
                    let mut cells: Vec<Vec<Cell>> = Vec::new();
                    let mut headercells = Vec::new();

                    //adding headerows cells into the vector
                    for data in data.headerRow.title {
                        headercells.push(Cell::new(data));
                    }
                    cells.push(headercells);

                    //adding datarows into the vector
                    for data in data.dataRows.rows {
                        let mut temp = Vec::new();
                        for content in data {
                            temp.push(Cell::new(content));
                        }
                        cells.push(temp);
                    }

                    let cell_count = &cells[1].len();
                    // println!("{:#?}" , cells);

                    //making row from the vector of cells
                    let mut rows: Vec<Row> = Vec::new();
                    let mut count = 0;
                    for content in cells {
                        rows.push(Row::new(
                            content,
                            if count == 0 {
                                RowType::HeaderRow
                            } else {
                                RowType::DataRows
                            },
                            if count == 0 {
                                data.headerRow.fontSize
                            } else {
                                data.dataRows.fontSize
                            },
                            data.pageWidth,
                            cell_count,
                        ));
                        count += 1;
                    }

                    let table = Table::new(rows);

                  match serde_json::to_string_pretty(&table){
                    Ok(data)=>{
                        data
                    }
                    Err(e)=>{
                        format!("{}",e)
                    }
                  }

                    // println!("{:#?}" , table);
                }
                Err(err) => {
                    println!("{}", err);
                    "failed to send data".to_string()
                }
            
    }
}