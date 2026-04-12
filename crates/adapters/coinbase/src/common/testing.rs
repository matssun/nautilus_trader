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

/// Loads a test fixture file by name from the crate's `test_data/` directory.
///
/// Under Cargo, `CARGO_MANIFEST_DIR` points at the actual source tree and is
/// resolved at compile time via `env!`.
///
/// Under Bazel (`--cfg=bazel_build`), the test binary runs in a sandbox where
/// the compile-time `CARGO_MANIFEST_DIR` path no longer exists. A sentinel
/// fixture file is declared via `nt_rust_test(fixture_files = ...)` in the
/// BUILD file, exposed to the test process as `NT_FIXTURE_COINBASE_TEST_DATA`
/// containing the sentinel's `$(rlocationpath ...)`. We resolve it with the
/// runfiles crate and take the parent directory.
///
/// # Panics
///
/// Panics if the fixture file does not exist or cannot be read.
pub fn load_test_fixture(name: &str) -> String {
    #[cfg(not(bazel_build))]
    let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_data");

    #[cfg(bazel_build)]
    let base = {
        let rlocpath = std::env::var("NT_FIXTURE_COINBASE_TEST_DATA")
            .expect("NT_FIXTURE_COINBASE_TEST_DATA not set by nt_rust_test fixture_files");
        let r = runfiles::Runfiles::create().expect("failed to init runfiles");
        let sentinel = r
            .rlocation(&rlocpath)
            .unwrap_or_else(|| panic!("could not resolve runfile: {rlocpath}"));
        sentinel
            .parent()
            .expect("sentinel fixture should have a parent dir")
            .to_path_buf()
    };

    let path = base.join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load test fixture '{}': {e}", path.display()))
}
