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

//! Shared testing utilities for OKX adapter unit tests.

use std::{fs, path::PathBuf};

/// Returns the path to the adapter's `test_data` directory.
///
/// See `crates/adapters/binance/src/common/testing.rs` for the Bazel
/// runfiles pattern used under `--cfg=bazel_build`.
#[cfg(test)]
fn test_data_dir() -> PathBuf {
    #[cfg(not(bazel_build))]
    return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    #[cfg(bazel_build)]
    {
        use std::ffi::OsStr;

        let rlocpath = std::env::var("NT_FIXTURE_OKX_TEST_DATA")
            .expect("NT_FIXTURE_OKX_TEST_DATA not set by nt_rust_test fixture_files");
        let r = runfiles::Runfiles::create().expect("failed to init runfiles");
        let abs_sentinel = r
            .rlocation(&rlocpath)
            .unwrap_or_else(|| panic!("could not resolve runfile: {rlocpath}"));
        let mut dir = abs_sentinel.clone();
        while dir.file_name() != Some(OsStr::new("test_data")) {
            if !dir.pop() {
                panic!(
                    "could not find 'test_data' ancestor of {}",
                    abs_sentinel.display()
                );
            }
        }
        dir
    }
}

#[cfg(test)]
#[must_use]
/// Loads a JSON fixture from the adapter test data directory.
pub fn load_test_json(file_name: &str) -> String {
    let path = test_data_dir().join(file_name);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read test JSON file {}: {e}", path.display()))
}
