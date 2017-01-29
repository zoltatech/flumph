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

use std::fs::File;
use std::io::SeekFrom;
use std::io::Write;
use std::io::Seek;
use std::io::Read;
use std::error::Error;
use utils::*;

// **************************************************************************
// 
// Utilites to help with positioning
//
// **************************************************************************


pub fn get_current_offset(file: &mut File) -> Result<u64, Box<Error>> {

	let position: u64 = try!(file.seek(SeekFrom::Current(0)));
	Ok(position)

}

pub fn move_to_offset(file: &mut File, base: u64, offset: u64) ->
	Result<(), Box<Error>> {

	try!(file.seek(SeekFrom::Start(base + offset)));
	Ok(())

}

// **************************************************************************
// 
// Functions to read and write strings. Strings are written as a four-byte
// length followed by the string data, and read with this expectation.
//
// **************************************************************************

pub fn read_string(file: &mut File) -> Result<String, Box<Error>> {

    let mut buffer = [0; 4];
    try!(file.read_exact(&mut buffer));
    let string_length = convert_bytes_to_u32(buffer) as usize;
    let mut buffer = vec![0; string_length];
    try!(file.read_exact(&mut buffer[0..string_length]));

    Ok(String::from_utf8_lossy(&buffer[0..string_length]).into_owned())

}


pub fn seek_and_read_string(file: &mut File, base: u64, offset: u64) -> 
	Result<String, Box<Error>> {

    try!(file.seek(SeekFrom::Start(base + offset)));
    Ok(try!(read_string(file)))

}


pub fn write_string(file: &mut File, length: usize, data: &String) -> 
	Result<(), Box<Error>> {

	let buffer:[u8; 4] = convert_u32_to_bytes(length as u32);
	try!(file.write(&buffer[..]));

    let mut buffer = data.as_bytes().to_vec();
    buffer.truncate(length);

    while buffer.len() < length {
        buffer.push(0);
    }

    try!(file.write(&buffer[0..length]));
    Ok(())

}

pub fn seek_and_write_string(file: &mut File, base: u64, offset: u64, 
	length: usize, data: &String) -> Result<(), Box<Error>> {

	try!(file.seek(SeekFrom::Start(base + offset)));
	Ok(try!(write_string(file, length, data)))

}

pub fn write_eyecatcher(file: &mut File, data: String) ->
	Result<(), Box<Error>> {

		try!(file.seek(SeekFrom::Start(0)));
		try!(file.write(data.as_bytes()));
		Ok(())		

}

pub fn validate_eyecatcher(file: &mut File, expected: String) -> 
	Result<(), Box<Error>> {

	try!(file.seek(SeekFrom::Start(0)));
    let mut buffer = [0; 4];
    try!(file.read_exact(&mut buffer));
    let eyecatcher = String::from_utf8_lossy(&buffer).into_owned();

	if !(eyecatcher == expected) {
		panic!("Corruption detected in file header.");
	}	

	Ok(())

}

// **************************************************************************
// 
// Functions to read and write u32
//
// **************************************************************************

pub fn read_u32(file: &mut File) -> Result<u32, Box<Error>> {    

    let mut buffer = [0; 4];
    try!(file.read_exact(&mut buffer));
    Ok(convert_bytes_to_u32(buffer))

}

pub fn seek_and_read_u32(file: &mut File, base: u64, offset: u64) ->
	Result<u32, Box<Error>> {
	
	try!(file.seek(SeekFrom::Start(base + offset)));
	Ok(try!(read_u32(file)))

}

pub fn write_u32(file: &mut File, data: u32) -> Result<(), Box<Error>> {

    let buffer: [u8;4] = convert_u32_to_bytes(data);
    try!(file.write(&buffer[..]));
    Ok(())

}

pub fn seek_and_write_u32(file: &mut File, base: u64, offset: u64, 
	data: u32) -> Result<(), Box<Error>> {

    try!(file.seek(SeekFrom::Start(base + offset)));
    Ok(try!(write_u32(file, data)))

}

// **************************************************************************
// 
// Functions to read and write u32
//
// **************************************************************************

pub fn read_u64(file: &mut File) -> Result<u64, Box<Error>> {    

    let mut buffer = [0; 8];
    try!(file.read_exact(&mut buffer));
    Ok(convert_bytes_to_u64(buffer))

}

pub fn seek_and_read_u64(file: &mut File, base: u64, offset: u64) ->
	Result<u64, Box<Error>> {
	
	try!(file.seek(SeekFrom::Start(base + offset)));
	Ok(try!(read_u64(file)))

}

pub fn write_u64(file: &mut File, data: u64) -> Result<(), Box<Error>> {

    let buffer: [u8;8] = convert_u64_to_bytes(data);
    try!(file.write(&buffer[..]));
    Ok(())

}

pub fn seek_and_write_u64(file: &mut File, base: u64, offset: u64, 
	data: u64) -> Result<(), Box<Error>> {

    try!(file.seek(SeekFrom::Start(base + offset)));
    Ok(try!(write_u64(file, data)))

}

