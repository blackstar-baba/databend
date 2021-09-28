// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod config_test;

pub mod config;
mod config_storage;

pub use common_store_api_sdk::RpcClientTlsConfig;
pub use config::Config;
pub use config::LogConfig;
pub use config::MetaConfig;
pub use config::QueryConfig;
pub use config::StoreConfig;
