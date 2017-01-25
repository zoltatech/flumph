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

pub fn convert_u32_to_bytes(data: u32) -> [u8;4] {

    let mut buffer: [u8;4] = [0;4];

    buffer[0] = ((data & 0xFF000000) >> 24) as u8;
    buffer[1] = ((data & 0x00FF0000) >> 16) as u8;
    buffer[2] = ((data & 0x0000FF00) >> 8)  as u8;
    buffer[3] = ((data & 0x000000FF) >> 0)  as u8;

    buffer
}

pub fn convert_bytes_to_u32(buffer: [u8; 4]) -> u32 {

    (buffer[0] as u32) >> 24 | (buffer[1] as u32) >> 16 | (buffer[2] as u32) >> 8 | (buffer[3] as u32)

}

pub fn convert_u64_to_bytes(data: u64) -> [u8;8] {

    let mut buffer: [u8;8] = [0;8];

    buffer[0] = ((data & 0xFF00000000000000) >> 56) as u8;
    buffer[1] = ((data & 0x00FF000000000000) >> 48) as u8;
    buffer[2] = ((data & 0x0000FF0000000000) >> 40) as u8;
    buffer[3] = ((data & 0x000000FF00000000) >> 32) as u8;
    buffer[4] = ((data & 0x00000000FF000000) >> 24) as u8;
    buffer[5] = ((data & 0x0000000000FF0000) >> 16) as u8;
    buffer[6] = ((data & 0x000000000000FF00) >> 8)  as u8;
    buffer[7] = ((data & 0x00000000000000FF) >> 0)  as u8;

    buffer

}

pub fn convert_bytes_to_u64(buffer: [u8; 8]) -> u64 {

    ((buffer[0] as u64) << 56) | ((buffer[1] as u64) << 48) | ((buffer[2] as u64) << 40) | 
        ((buffer[3] as u64) << 32) | ((buffer [4] as u64) << 24) | ((buffer[5] as u64) << 16) |
        ((buffer[6] as u64) << 8) | ((buffer[7] as u64))

}



