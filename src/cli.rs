// Copyright 2025 natsuu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short = 'c',
        long = "current",
        required = true,
        help = "Path to the new version compressed file"
    )]
    pub current_version_path: String,
    #[arg(
        short = 'p',
        long = "previous",
        required = true,
        help = "Path to the previous version compressed file"
    )]
    pub previous_version_path: String,
    #[arg(
        short = 'o',
        long = "output",
        required = false,
        help = "Output path for the generated patch file"
    )]
    pub output_path: Option<String>,
    #[arg(
        long = "temp",
        required = false,
        help = "Temporary directory path for extraction"
    )]
    pub temp_dir_path: Option<String>,
    #[arg(
        long = "format",
        required = false,
        help = "Patch file format (e.g., bsdiff, zstd)"
    )]
    pub format: Option<String>,
}
