use serde::Serialize;
use std::{fs, io};
#[derive(Debug, Serialize)]
struct Cell {
    height: i32,
    width: i32,
    content: String,
}

impl Cell {
    fn new(height: i32, width: i32, content: String) -> Cell {
        Cell {
            height,
            width, 
            content,
        }
    }
}

#[derive(Debug, Serialize)]
struct Row {
    height: i32,
    width: i32,
    cells: Vec<Cell>,
}

impl Row {
    fn new(height: i32, width: i32, cells: Vec<Cell>) -> Row {
        Row {
            height,
            width,
            cells,
        }
    }

    fn calculate_width(&self) -> i32 {
        let mut width: i32 = 0;
        for i in 0..self.cells.len() {
            width += self.cells[i].width as i32
        }
        width
    }

    fn calculate_height(&self) -> i32 {
        let mut max = -1;
        for i in 0..self.cells.len() {
            if self.cells[i].height > max {
                max = self.cells[i].height;
            }
        }
        max
    }
}



#[derive(Debug, Serialize)]
struct Table {
    height: i32,
    width: i32,
    rows: Vec<Row>,
}

impl Table {
    fn new(height: i32, width: i32, rows: Vec<Row>) -> Table {
        Table {
            height,
            width,
            rows,
        }
    }

    fn calculate_table_height(&self) -> i32{
        let mut height = 0;
        for i in  0..self.rows.len(){
            height += self.rows[i].height;
        }
        height
    }
}

fn change_hight(table : &mut Table){
    let mut temp = String::new();
        println!("changed Height:");
        io::stdin().read_line(&mut temp).expect("Failed to take input");
        let height:i32 = temp.trim().parse().expect("Invalid Input");

        let mut temp = String::new();
        println!("Enter Row No:");
        io::stdin().read_line(&mut temp).expect("Failed to take input");
        let row:i32 = temp.trim().parse().expect("Invalid Input");

        let mut temp = String::new();
        println!("Enter Cell No:");
        io::stdin().read_line(&mut temp).expect("Failed to take input");
        let no:i32 = temp.trim().parse().expect("Invalid Input");

        table.rows[row as usize].cells[no as usize].height = height;

        table.rows[row as usize].height = table.rows[row as usize].calculate_height();

        table.height = table.calculate_table_height();
}

fn main() {
    println!("How many cells you want to enter");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    let parsed_input: u32 = input
        .trim()
        .parse()
        .expect("Invalid input , Enter valid positive number");

    // println!("{:?}" , parsed_input);

    //Taking all cells from the input
    let mut i = 1;
    let mut cells: Vec<Cell> = Vec::new();
    while i <= parsed_input {
        println!("cell-{}", i);

        let mut temp = String::new();
        println!("Height:");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to take input");
        let height: i32 = temp.trim().parse().expect("Invalid Input");

        // let mut width = String::new();
        // println!("Width:");
        // io::stdin().read_line(&mut width).expect("Failed to take input");
        // let width:i32 = input.trim().parse().expect("Invalid Input");

        let mut content = String::new();
        println!("content:");
        io::stdin()
            .read_line(&mut content)
            .expect("Failed to take input");

        let cell = Cell::new(height, 20, content);
        cells.push(cell);
        i += 1;
    }

    //making Row out of all the cells
    let mut rows: Vec<Row> = Vec::new();
    let mut row = Row::new(0, 0, Vec::new());
    rows.push(row);
    let mut i = 1;
    let mut count = 0;

    //adding cells to the rows
    for cell in cells {
        if i <= 5 {
            rows[count].cells.push(cell);
            i += 1;
        } else {
            rows.push(Row::new(0, 0, Vec::new()));
            count += 1;
            rows[count].cells.push(cell); //for loss data when else is run
            i = 1;
        }
        rows[count].width = rows[count].calculate_width();
        rows[count].height = rows[count].calculate_height();
    }

    let mut table = Table::new(0 , 100 ,rows);
    table.height = table.calculate_table_height();

    change_hight(&mut table);
    // println!("{:#?}" , table);

    match serde_json::to_string_pretty(&table) {
        Ok(data) => {
            fs::write("src/table.json", data);
        }

        Err(err) => {
            println!("{:?}", err);
        }

    }
}
