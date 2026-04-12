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

//! Test utilities for the Polymarket adapter.
//!
//! Upstream tests load fixtures with `format!("test_data/{filename}")`, which
//! relies on the working directory being the crate source root — true under
//! `cargo test` but not under Bazel, which runs tests from the runfiles root.
//! `test_data_path(filename)` resolves the fixture regardless of build system.

use std::path::PathBuf;

/// Returns the absolute path to a fixture file in the adapter's `test_data/`
/// directory.
///
/// Under Cargo, `CARGO_MANIFEST_DIR` points at the actual source tree and is
/// resolved at compile time. Under Bazel (`--cfg=bazel_build`), the
/// compile-time manifest path is a stale sandbox path; we instead resolve a
/// sentinel fixture (passed via `nt_rust_test(fixture_files = ...)`) through
/// the runfiles crate and walk up to its `test_data` ancestor.
#[must_use]
pub fn test_data_path(filename: &str) -> PathBuf {
    #[cfg(not(bazel_build))]
    return PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_data")
        .join(filename);

    #[cfg(bazel_build)]
    {
        use std::ffi::OsStr;

        let rlocpath = std::env::var("NT_FIXTURE_POLYMARKET_TEST_DATA")
            .expect("NT_FIXTURE_POLYMARKET_TEST_DATA not set by nt_rust_test fixture_files");
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
        dir.join(filename)
    }
}
