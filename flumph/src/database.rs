// Copyright 2017 Robert L Snyder, Ithaca, NY <zoltatech@gmail.com> 
//                                            <robscary@gmail.com>
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use structures::Database;
use structures::Table;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use utils::convert_i32_to_bytes;

fn save_database_to_file(database: &Database, path: &Path) {
    
    let mut db_file = File::create(path).expect("Error writing database file.");
    write!(db_file, "FLUMPHDB").unwrap();
    
    let len_buffer = convert_i32_to_bytes(database.database_name().len() as u32);
    let table_count = convert_i32_to_bytes(database.tables().len() as u32);
    db_file.write(&len_buffer).unwrap();
    db_file.write(database.database_name().as_bytes()).unwrap();
    db_file.write(&table_count).unwrap();

    for table in database.tables() {
        let len_buffer = convert_i32_to_bytes(table.table_name().len() as u32);
        db_file.write(&len_buffer).unwrap();
        db_file.write(table.table_name().as_bytes()).unwrap();

        {
            let table_path = Path::new(table.table_name());
            if !table_path.exists() {

            }
        }
    }



}



pub fn save_database(database: Database) -> Database {
    
    {
        let database_path = Path::new(database.database_name());
        if database_path.exists() {
            
        } else {
            save_database_to_file(&database, &database_path);
        }
    }

    database

}
