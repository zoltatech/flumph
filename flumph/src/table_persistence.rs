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

use table::Table;
use field::Field;
use std::fs::File;
use std::iter::Iterator;
use std::error::Error;
use table_io::*; 

const OFFSET_RECORD_LENGTH: u64 = 4;
const OFFSET_RECORD_COUNT: u64 = 8;
const OFFSET_FIRST_RECORD_POINTER: u64 = 16;
const OFFSET_FIELD_COUNT: u64 = 20;
const OFFSET_FIELD_LIST: u64 = 28;


pub fn read_table_header(table: &mut Table, file: &mut File) -> Result<(), Box<Error>> {

    try!(validate_eyecatcher(file, String::from("FTBL")));

    table.record_length = try!(seek_and_read_u32(file, 0, OFFSET_RECORD_LENGTH));
    table.record_count = try!(seek_and_read_u64(file, 0, OFFSET_RECORD_COUNT));
    table.first_record_pointer = try!(seek_and_read_u64(file, 0, OFFSET_FIRST_RECORD_POINTER));
    let field_count = try!(seek_and_read_u32(file, 0, OFFSET_FIELD_COUNT));

    try!(move_to_offset(file, 0, OFFSET_FIELD_LIST));

    for _ in (0..field_count).enumerate() {
        
        let field_name = try!(read_string(file));
        let mut field = Field::new(field_name);

        field.offset = try!(read_u32(file));
        field.length = try!(read_u32(file));

        table.fields.push(field);

    }

    Ok(())

}

pub fn create_table_header(table: &mut Table, file: &mut File) -> Result<(), Box<Error>> {

    try!(write_eyecatcher(file, String::from("FTBL")));
    try!(write_u32(file, table.record_length));
    try!(write_u64(file, table.record_count));
    try!(write_u64(file, table.first_record_pointer));
    try!(write_u32(file, table.fields.len() as u32));

    let mut record_length: u32 = 0;
    for field in &table.fields {

        try!(write_string(file, field.field_name.len(), &field.field_name));
        try!(write_u32(file, field.offset));
        try!(write_u32(file, field.length));
        record_length = record_length + (field.length);

    }

    let first_record_pointer = try!(get_current_offset(file));
    try!(seek_and_write_u64(file, 0, OFFSET_FIRST_RECORD_POINTER, first_record_pointer));
    
    table.record_length = record_length;
    try!(seek_and_write_u32(file, 0, OFFSET_RECORD_LENGTH, table.record_length));
    
    try!(file.sync_all());

    Ok(())

}