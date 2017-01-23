//! Copyright 2017 Robert L Snyder, Ithaca, NY <zoltatech@gmail.com> 
//!                                            <robscary@gmail.com>
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//!        http://www.apache.org/licenses/LICENSE-2.0

//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

extern crate flumph;

use flumph::database::save_database;
use std::string::String;
use flumph::structures::Database;
use flumph::structures::Table;

#[test]
fn test_db_create() {
    
    let mut database = Database::new(String::from("test.db"));
    let mut table = Table::new(String::from("test_table"));

    let test_table_index = database.add_table(table);

    save_database(database);

}