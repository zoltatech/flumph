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

extern crate flumph;

use flumph::table::Table;
use flumph::field::Field;
use std::string::String;
use std::path::Path;
use std::fs::remove_file;

#[test]
fn test_table_header() {

    let mut table = Table::new(String::from("test.tbl"), String::from("."));
    let mut field = Field::new(String::from("ID"));
    let mut another_field = Field::new(String::from("Name"));
    let path = Path::new("test.tbl");

    if path.exists() {         
        match remove_file(path) {
            Ok(_) => (),
            Err(e) => panic!("Found an existing test file and could not remove it: {}", e.to_string())
        }
    }

    field.length = 40;
    field.offset = 0;
    table.fields.push(field);

    another_field.length=60;
    another_field.offset=40;
    table.fields.push(another_field);

    // You would never do this, but we're testing the header creation so we need some
    // non-zero values. Note that record length should get replaced with the actual
    // length (100)

    table.record_length = 0xF00FBABE;
    table.record_count = 0xFF00FF00DDEEDDEE;    

    // First time through should create the header
    let _ = match table.open() {
        Ok(table_file) => table_file,
        Err(e) => panic!("Failed creating table header: {}", e.to_string())
    };

    if !path.exists() {
        panic!("File was not created, but no error was raised...");
    }

    let mut table = Table::new(String::from("test.tbl"), String::from("."));
    let _ = table.open();

    assert_eq!(table.record_length, 100);
    assert_eq!(table.record_count, 0xFF00FF00DDEEDDEE);
    assert_eq!(table.fields.len(), 2);

    let field = &table.fields[0];
    assert_eq!(field.length, 40);
    assert_eq!(field.offset, 0);
    assert_eq!(field.field_name, "ID");

    let field = &table.fields[1];
    assert_eq!(field.length, 60);
    assert_eq!(field.offset, 40);
    assert_eq!(field.field_name, "Name");

    

}