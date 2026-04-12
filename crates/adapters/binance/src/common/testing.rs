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

//! Test fixtures and utilities for Binance adapter tests.

use std::{fs, path::PathBuf};

use serde_json::Value;

/// Returns the path to the adapter's `test_data` directory.
///
/// Under Cargo, resolved at compile time via `env!("CARGO_MANIFEST_DIR")`.
/// Under Bazel (`--cfg=bazel_build`), the compile-time manifest path is a
/// stale sandbox path; we instead resolve a sentinel fixture (passed via
/// `nt_rust_test(fixture_files = ...)`) through the runfiles crate and walk
/// up to its `test_data` ancestor.
pub fn test_data_path() -> PathBuf {
    #[cfg(not(bazel_build))]
    return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    #[cfg(bazel_build)]
    {
        use std::ffi::OsStr;

        let rlocpath = std::env::var("NT_FIXTURE_BINANCE_TEST_DATA")
            .expect("NT_FIXTURE_BINANCE_TEST_DATA not set by nt_rust_test fixture_files");
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

/// Loads a text fixture from the adapter's `test_data` directory.
///
/// # Panics
///
/// Panics if the fixture file cannot be read.
pub fn load_fixture_string(relpath: &str) -> String {
    let path = test_data_path().join(relpath);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read Binance test fixture: {}", path.display()))
}

/// Loads a JSON fixture from the adapter's `test_data` directory.
///
/// # Panics
///
/// Panics if the fixture file cannot be read or contains invalid JSON.
pub fn load_json_fixture(relpath: &str) -> Value {
    let content = load_fixture_string(relpath);
    serde_json::from_str(&content).expect("Invalid JSON in Binance test fixture")
}

/// Loads a binary fixture from the adapter's `test_data` directory.
///
/// # Panics
///
/// Panics if the fixture file cannot be read.
pub fn load_fixture_bytes(relpath: &str) -> Vec<u8> {
    let path = test_data_path().join(relpath);
    fs::read(&path)
        .unwrap_or_else(|_| panic!("Failed to read Binance test fixture: {}", path.display()))
}

/// Loads a JSON fixture and returns the nested `event` object when present.
///
/// Binance docs examples for user-data streams wrap the payload in an
/// outer object containing `subscriptionId` and `event`.
///
/// # Panics
///
/// Panics if the fixture file cannot be read or contains invalid JSON.
pub fn load_event_fixture(relpath: &str) -> Value {
    let json = load_json_fixture(relpath);
    json.get("event").cloned().unwrap_or(json)
}
