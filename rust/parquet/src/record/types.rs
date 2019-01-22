// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

mod array;
mod decimal;
mod group;
mod list;
mod map;
mod numbers;
mod option;
mod time;
mod tuple;
mod value;

use super::schemas::ValueSchema;
use crate::errors::ParquetError;

pub use self::{
    array::*, decimal::*, group::*, list::*, map::*, numbers::*, option::*, time::*, tuple::*,
    value::*,
};

pub trait Downcast<T> {
    fn downcast(self) -> Result<T, ParquetError>;
}

fn downcast<T>((name, schema): (String, ValueSchema)) -> Result<(String, T), ParquetError>
where
    ValueSchema: Downcast<T>,
{
    schema.downcast().map(|schema| (name, schema))
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Root<T>(pub T);
