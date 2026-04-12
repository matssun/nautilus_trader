// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2026 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Shared helpers for adapter unit tests.

use std::{fs, path::PathBuf};

#[cfg(test)]
#[must_use]
/// Loads the named JSON fixture from the Bybit `test_data` directory.
///
/// See `crates/adapters/deribit/src/common/testing.rs` for the Bazel runfiles
/// pattern used under `--cfg=bazel_build`.
///
/// # Panics
/// Panics if the fixture file cannot be read.
pub fn load_test_json(file_name: &str) -> String {
    #[cfg(not(bazel_build))]
    let dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    #[cfg(bazel_build)]
    let dir: PathBuf = {
        let rlocpath = std::env::var("NT_FIXTURE_BYBIT_TEST_DATA")
            .expect("NT_FIXTURE_BYBIT_TEST_DATA not set by nt_rust_test fixture_files");
        let r = runfiles::Runfiles::create().expect("failed to init runfiles");
        let sentinel = r
            .rlocation(&rlocpath)
            .unwrap_or_else(|| panic!("could not resolve runfile: {rlocpath}"));
        sentinel
            .parent()
            .expect("sentinel fixture should have a parent dir")
            .to_path_buf()
    };

    let path = dir.join(file_name);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to load Bybit test fixture {}: {e}", path.display()))
}
