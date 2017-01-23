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

pub fn convert_i32_to_bytes(data: u32) -> [u8;4] {
    let mut buffer: [u8;4] = [0;4];
    buffer[0] = ((data & 0xFF000000) >> 24) as u8;
    buffer[1] = ((data & 0x00FF0000) >> 16) as u8;
    buffer[2] = ((data & 0x0000FF00) >> 8) as u8;
    buffer[3] = (data & 0x000000FF) as u8;

    buffer
}

// pub fn convert_string_to_bytes<'a>(data: &String)-> &'a [u8] {
//     let mut buffer: vec![];

// }