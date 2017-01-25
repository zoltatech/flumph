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
use field::Field;
use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use table_persistence::create_table_header;
use table_persistence::read_table_header;
use table_persistence::get_file_length;
use std::error::Error;

pub struct Table {
    pub record_length: u64,
    pub record_count: u64,
    pub first_record_pointer: u64,
    pub table_name: String,
    pub file_path: String,
    pub fields: Vec<Field>
}

impl Table {

    pub fn new(table_name: String, path: String) -> Table {
        Table{table_name: table_name, file_path: path, record_length: 0, record_count: 0, 
            first_record_pointer: 0, fields: vec![]}
    }

    pub fn open(&mut self) -> Result<File, Box<Error>> {
        let path = Path::new(&self.file_path).join(&self.table_name);
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path)
            .expect("Could not open table");


        if get_file_length(&mut file) == 0 {
            try!(create_table_header(self, &mut file));
        } else {
            try!(read_table_header(self, &mut file));
        }

        Ok(file)

    }

}

