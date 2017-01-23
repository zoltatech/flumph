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

use std::string::String;

// pub enum FieldType {
//     Int8,
//     Int32,
//     Int64,
//     Flag,
//     Float32,
//     Float64,
//     Character
// }

pub struct Field {
//     index: u32,
//     length: u32,
//     datatype: FieldType,
//     field_name: String
}

pub struct Table {
    record_length: u64,
    record_count: u64,
    table_name: String,
    fields: Vec<Field>
}

pub struct Database {
    database_name: String,    
    tables: Vec<Table>
}

impl Table {
    pub fn new(table_name: String) -> Table {
        Table{record_length: 0, record_count: 0, table_name: table_name, fields: vec![]}
    }
    pub fn table_name(&self) -> &String {
        &self.table_name
    }
}

impl Database {
    pub fn new(database_name: String) -> Database {
        Database{database_name: database_name, tables: vec![]}
    }
    pub fn database_name(&self) -> &String {
        &self.database_name
    }
    pub fn tables(&self) -> &Vec<Table> {
        &self.tables
    }
    pub fn add_table(&mut self, table: Table) -> usize {
        self.tables.push(table);
        self.tables.len()
    }
}