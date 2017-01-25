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
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Write;
use utils::convert_u64_to_bytes;
use utils::convert_u32_to_bytes;
use utils::convert_bytes_to_u64;
use utils::convert_bytes_to_u32;
use std::io::Read;
use std::iter::Iterator;
use std::error::Error;

const OFFSET_EYECATCHER: u64 = 0;
const OFFSET_RECORD_LENGTH: u64 = 4;
const OFFSET_RECORD_COUNT: u64 = 12;
const OFFSET_FIRST_RECORD_POINTER: u64 = 20;
const OFFSET_FIELD_COUNT: u64 = 28;
const OFFSET_FIELD_LIST: u64 = 32;


pub fn get_file_length(file: &mut File) -> u64 {

    file.seek(SeekFrom::End(0)).unwrap()

}

pub fn read_table_header(table: &mut Table, file: &mut File) -> Result<(), Box<Error>> {

    try!(file.seek(SeekFrom::Start(OFFSET_EYECATCHER)));

    let mut buffer = [0; 4];
    try!(file.read_exact(&mut buffer));
    let eyecatcher = String::from_utf8_lossy(&buffer).into_owned();

    if eyecatcher != "FTBL" {
        panic!("This table does not appear to be valid!");
        //Err(From::from("This table does not appear to be valid!"))
    } 

    let mut buffer = [0; 8];
    try!(file.read_exact(&mut buffer));
    table.record_length = convert_bytes_to_u64(buffer);

    try!(file.read_exact(&mut buffer));
    table.record_count = convert_bytes_to_u64(buffer);
    
    try!(file.read_exact(&mut buffer));
    table.first_record_pointer = convert_bytes_to_u64(buffer);

    let mut buffer = [0; 4];
    try!(file.read_exact(&mut buffer));
    let field_count = convert_bytes_to_u32(buffer);

    for _ in (0..field_count).enumerate() {
        
        let mut buffer = [0; 4];
        try!(file.read_exact(&mut buffer));
        let mut field = Field::new(String::from_utf8_lossy(&buffer).into_owned());

        try!(file.read_exact(&mut buffer));
        field.offset = convert_bytes_to_u32(buffer);

        try!(file.read_exact(&mut buffer));
        field.length = convert_bytes_to_u32(buffer);

        table.fields.push(field);

    }

    Ok(())

}

pub fn create_table_header(table: &mut Table, file: &mut File) -> Result<(), Box<Error>> {

    try!(file.seek(SeekFrom::Start(OFFSET_EYECATCHER)));
    try!(write!(file, "FTBL"));
    
    let buffer = convert_u64_to_bytes(table.record_length);
    try!(file.write(&buffer));

    let buffer = convert_u64_to_bytes(table.record_count);
    try!(file.write(&buffer));

    let buffer = convert_u64_to_bytes(table.first_record_pointer);
    try!(file.write(&buffer));

    let buffer = convert_u32_to_bytes(table.fields.len() as u32);
    try!(file.write(&buffer));
    
    let mut record_length: u64 = 0;
    for field in &table.fields {
        
        let buffer = convert_u32_to_bytes(field.field_name.len() as u32);
        try!(file.write(&buffer));
        try!(file.write(field.field_name.as_bytes()));

        let buffer = convert_u32_to_bytes(field.offset);
        try!(file.write(&buffer));

        let buffer = convert_u32_to_bytes(field.length);
        try!(file.write(&buffer));

        record_length = record_length + (field.length as u64);

    }

    let first_record_pointer = try!(file.seek(SeekFrom::Current(0)));
    let buffer = convert_u64_to_bytes(first_record_pointer);
    try!(file.seek(SeekFrom::Start(OFFSET_FIRST_RECORD_POINTER)));
    try!(file.write(&buffer));

    table.record_length = record_length;
    let buffer = convert_u64_to_bytes(record_length);
    try!(file.seek(SeekFrom::Start(OFFSET_RECORD_LENGTH)));
    try!(file.write(&buffer));

    try!(file.sync_all());

    Ok(())

}