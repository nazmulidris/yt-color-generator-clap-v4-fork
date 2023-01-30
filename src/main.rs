/*
 *   Copyright (c) 2023 Nazmul Idris
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use clap::Parser;
use gen_color::{gradient, Cli, Commands};
use miette::{Context, Result};
use owo_colors::OwoColorize;
use rand::Rng;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Gradient(gradient_options) => {
            gradient::generate(gradient_options)
                .wrap_err("when generating gradient")
        }
        Commands::Random => {
            let mut rng = rand::thread_rng();

            let color = owo_colors::Rgb(
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );
            let debug_str = "    ";
            print!(
                "{} #{:x}{:x}{:x}",
                debug_str.on_color(color),
                color.0,
                color.1,
                color.2
            );
            Ok(())
        }
    }
}
