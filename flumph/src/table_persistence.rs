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

const OFFSET_EYECATCHER: u64 = 0;
const OFFSET_RECORD_LENGTH: u64 = 4;
const OFFSET_RECORD_COUNT: u64 = 12;
const OFFSET_FIRST_RECORD_POINTER: u64 = 20;
const OFFSET_FIELD_COUNT: u64 = 28;
const OFFSET_FIELD_LIST: u64 = 32;

pub fn get_file_length(file: &mut File) -> u64 {

    file.seek(SeekFrom::End(0)).unwrap()

}

pub fn create_table_header(table: &Table, file: &mut File) {

    file.seek(SeekFrom::Start(OFFSET_EYECATCHER)).unwrap();
    write!(file, "FTBL").unwrap();
    
    let buffer = convert_u64_to_bytes(table.record_length());
    file.write(&buffer).unwrap();

    let buffer = convert_u64_to_bytes(table.record_count());
    file.write(&buffer).unwrap();

    let buffer = convert_u64_to_bytes(table.first_record_pointer());
    file.write(&buffer).unwrap();

    let buffer = convert_u32_to_bytes(table.fields().len() as u32);
    file.write(&buffer).unwrap();
    
    for field in table.fields() {
        
        let buffer = convert_u32_to_bytes(field.field_name().len() as u32);
        file.write(&buffer).unwrap();
        file.write(field.field_name().as_bytes()).unwrap();

        let buffer = convert_u32_to_bytes(field.offset());
        file.write(&buffer).unwrap();

        let buffer = convert_u32_to_bytes(field.length());
        file.write(&buffer).unwrap();

    }

    let first_record_pointer = file.seek(SeekFrom::Current(0)).unwrap();
    let buffer = convert_u64_to_bytes(first_record_pointer);
    file.seek(SeekFrom::Start(OFFSET_FIRST_RECORD_POINTER)).unwrap();
    file.write(&buffer).unwrap();

    file.sync_all().expect("Failure while syncing file data.");

}