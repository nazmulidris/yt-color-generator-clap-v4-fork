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

use crate::ColorGenerationError;
use clap::{arg, Args};
use owo_colors::OwoColorize;
use palette::{rgb::Rgb, Gradient, LinSrgb};
use rand::Rng;
use std::{iter::zip, str::FromStr};

#[derive(Args)]
pub struct GradientOptions {
    /// an ordered list of colors
    #[arg(short = 'c', long = "color")]
    colors: Vec<String>,
    /// number of steps to use in gradient
    #[arg(short, long, default_value_t = 10)]
    num_steps: usize,
    /// stop positions
    #[arg(short, long)]
    stops: Vec<f32>,
}

pub fn generate(
    GradientOptions {
        colors,
        num_steps,
        stops,
    }: &GradientOptions,
) -> Result<(), ColorGenerationError> {
    if colors.len() > 0 {
        if colors.len() != stops.len() {
            let color_str = format!("{:?}", colors);
            let stops_str = format!("{:?}", stops);
            return Err(
                ColorGenerationError::ColorsAndStepsMustMatch {
                    input: format!("{}\n{}\n{}", &color_str, num_steps, stops_str),
                    advice: format!("match number of colors: `{}` with number of stops: `{}`", colors.len(), stops.len()),
                    color_src: (0,color_str.len()),
                    stops_src: (color_str.len()+4, stops_str.len()),
                });
        }

        // let acc = colors.iter().map(|color| {
        //     LinSrgb::from_str(color)
        //         .expect("hex code")
        //         .into_format()
        // }).collect::<Vec<LinSrgb>>();

        // let gradient = Gradient::new(acc);
        // gradient.take(5).map(|Rgb { red, green, blue, standard: _ }| {
        //     owo_colors::Rgb((red * 255.0) as u8, (green * 255.0) as u8, (blue * 255.0) as u8)
        // }).for_each(|color| {
        //     let debug_str = "    ";
        //     print!("{}", debug_str.on_color(color));
        // });

        let color_list = zip(stops, colors)
            .map(|(&stop, color)| {
                (
                    stop,
                    LinSrgb::from_str(color)
                        .expect("hex code")
                        .into_format(),
                )
            })
            .collect::<Vec<(f32, LinSrgb)>>();

        let gradient = Gradient::with_domain(color_list);

        for color in gradient.take(*num_steps).map(
            |Rgb {
                 red,
                 green,
                 blue,
                 standard: _,
             }| {
                owo_colors::Rgb(
                    (red * 255.0) as u8,
                    (green * 255.0) as u8,
                    (blue * 255.0) as u8,
                )
            },
        ) {
            let debug_str = "    ";
            print!("{}", debug_str.on_color(color));
        }
        Ok(())
    } else {
        let mut rng = rand::thread_rng();
        let gradient = Gradient::new(vec![
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            LinSrgb::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
        ]);

        let taken_colors: Vec<_> = gradient
            .take(*num_steps)
            .map(
                |Rgb {
                     red,
                     green,
                     blue,
                     standard: _,
                 }| {
                    owo_colors::Rgb(
                        (red * 255.0) as u8,
                        (green * 255.0) as u8,
                        (blue * 255.0) as u8,
                    )
                },
            )
            .collect();

        for color in taken_colors.iter() {
            let debug_str = "    ";
            print!("{}", debug_str.on_color(*color));
        }
        Ok(())
    }
}
