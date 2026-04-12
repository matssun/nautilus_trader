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

//! Testing helpers shared across dYdX adapter tests.

use std::path::PathBuf;

use serde_json::Value;

/// Returns the path to the adapter's `test_data` directory.
///
/// See `crates/adapters/binance/src/common/testing.rs` for the Bazel
/// runfiles pattern used under `--cfg=bazel_build`.
pub fn test_data_path() -> PathBuf {
    #[cfg(not(bazel_build))]
    return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    #[cfg(bazel_build)]
    {
        use std::ffi::OsStr;

        let rlocpath = std::env::var("NT_FIXTURE_DYDX_TEST_DATA")
            .expect("NT_FIXTURE_DYDX_TEST_DATA not set by nt_rust_test fixture_files");
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

/// Loads a JSON fixture from the adapter's `test_data` directory.
///
/// # Panics
///
/// Panics if the file cannot be read or contains invalid JSON.
pub fn load_json_fixture(filename: &str) -> Value {
    let path = test_data_path().join(filename);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read test data file: {}", path.display()));
    serde_json::from_str(&content).expect("Invalid JSON in test data file")
}

/// Loads a JSON fixture and returns the `result` field when present.
///
/// Many dYdX HTTP endpoints wrap payloads in a `result` envelope. This helper
/// returns the inner payload if it exists, otherwise the root JSON value.
///
/// # Panics
///
/// Panics if the file cannot be read or contains invalid JSON.
pub fn load_json_result_fixture(filename: &str) -> Value {
    let json = load_json_fixture(filename);
    json.get("result").cloned().unwrap_or(json)
}
