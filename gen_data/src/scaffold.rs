use std::fs::File;
use std::fs;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{gen_common::get_col_type, structs::{DataColumn, DataTable}};

static FILE_LOCATION: &str = "init_db.sql";
static CREATE_TABLE: &str = "CREATE TABLE";
static EXISTS: &str = "EXISTS";

pub fn read_files() {
    println!("---- Reading SQL Schema from: {} ----", FILE_LOCATION);

    let contents = fs::read_to_string(FILE_LOCATION).expect("There was an error reading the file");
    println!("---- Reading file from database schema ----");

    let tables = parse_db_file(&contents);

    write_to_rust_file(tables);
}

//not sure if its reall the best idea to read the entire database file out as a string.
//Is there any better way to do it????
fn parse_db_file(sql_file: &str) -> Vec<DataTable> {
    //-----Tables-----
    let mut tables: Vec<DataTable> = Vec::new();
    let raw_tables: Vec<&str> = sql_file.split(EXISTS).collect(); //Gets the table name. 
    let count: usize = raw_tables.len();
    println!("Table Count: {}", count);

    for table_as_string in raw_tables {
        let table_details: Vec<&str> = table_as_string.split("(").collect();
        let table_name: &str = table_details[0].trim();

        if table_name.to_string().is_empty() {
            continue;
        }

        //-----Columns-----
        println!("-------- {} --------", table_name);
        let mut table_cols: Vec<&str> = table_as_string.trim().split(",").collect();
        table_cols.remove(0);
        table_cols.pop();

        let mut columns: Vec<DataColumn> = Vec::new();

        for col in table_cols {
            // println!("col Details: {}", col.trim_start());
            let col_details: Vec<&str> = col.trim_start().split_ascii_whitespace().collect();

            if col_details.len() < 2 {
                panic!(
                    "Column in {} is missing type. Please fix the table. ",
                    table_name
                )
            }

            let col_type = get_col_type(col_details[1]);

            columns.push(DataColumn::new(col_type, col_details[1].to_string()))
        }
        tables.push(DataTable::new(table_name.to_owned(), columns))
    }

    tables
}

fn write_to_rust_file(sql_tables: Vec<DataTable>) {
    let path = "TestModels/";

    sql_tables.par_iter().for_each(|x: &DataTable| {
        let file_path: String = path.to_owned() + &x.table_name + ".rs";
        let mut _file = File::create(file_path);
        
        //write the contents to the file.
    });
}



