//! Compact bar charts in the terminal.
//!
//! # Examples
//! labeled comparison
//! ```
//! use chart_relative::{params::*, Chart};
//!
//! let chart = Chart::new(
//!    &[
//!        0, 22, 2, 9, 223, 34, 33, 66, 76, 122, 199, 33, 12, 89, 1222, 100,
//!    ],
//!    Some(ChartComparison {
//!        data: &[
//!            14, 20, 1, 8, 223, 12, 56, 79, 69, 100, 1122, 33, 45, 9, 9000, 78,
//!        ],
//!    }),
//!    ChartOptions {
//!        height: 16,
//!        view: ViewPreference::Bottom,
//!        display: DisplayMode::Portrait {
//!            labels: &[
//!                "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
//!                "eighth", "nineth", "tenth", "eleventh", "twelfth", "thirteenth",
//!                "fourteenth", "fifteenth", "sixteenth",
//!            ],
//!        },
//!    },
//!);
//!println!("{chart}");
//! ```
//! output (with color)
//! ```text
//! 122│            🢁🢁             ▇  🢁🢁          🢁🢁
//!    │            🢁🢁             █  🢁🢁          🢁🢁
//!    │            🢁🢁             █  🢁🢁          🢁🢁
//!    │            🢁🢁             ██ 🢁🢁          🢁🢁 █
//!    │            🢁🢁             ██ 🢁🢁       ▅  🢁🢁 █
//!    │            🢁🢁        ▂    ██ 🢁🢁       █  🢁🢁 █▁
//!    │            🢁🢁        █ ▇  ██ 🢁🢁       █  🢁🢁 ██
//!    │            🢁🢁       ▅█ ██ ██ 🢁🢁       █  🢁🢁 ██
//!    │            🢁🢁     ▂ ██ ██ ██ 🢁🢁       █  🢁🢁 ██
//!    │            🢁🢁     █ ██ ██ ██ 🢁🢁       █  🢁🢁 ██
//!    │            🢁🢁     █ ██ ██ ██ 🢁🢁     ▇ █  🢁🢁 ██
//!    │            🢁🢁 ▃  ▂█ ██ ██ ██ 🢁🢁 ▂▂  █ █  🢁🢁 ██
//!    │            🢁🢁 █  ██ ██ ██ ██ 🢁🢁 ██  █ █  🢁🢁 ██
//!    │   ▇▄       🢁🢁 █  ██ ██ ██ ██ 🢁🢁 ██  █ █  🢁🢁 ██
//!    │ ▆ ██    ▁  🢁🢁 █▄ ██ ██ ██ ██ 🢁🢁 ██ ▄█ █▁ 🢁🢁 ██
//!   1│⨯█ ██ ▂▁ ██ 🢁🢁 ██ ██ ██ ██ ██ 🢁🢁 ██ ██ ██ 🢁🢁 ██
//!     0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15
//!  0: first         6: seventh      12: thirteenth
//!  1: second        7: eighth       13: fourteenth
//!  2: third         8: nineth       14: fifteenth
//!  3: fourth        9: tenth        15: sixteenth
//!  4: fifth        10: eleventh
//!  5: sixth        11: twelfth
//! ```

#![warn(unused_lifetimes, missing_docs)]

use colored::Colorize;

/// Parameters for creating a `Chart`.
pub mod params {

    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum ViewPreference {
        /// If any values smaller than `options.height * 8` exist in `data + compare.data`,
        /// then display them. Larger values will be indicated by `🢁`.
        /// If smaller values don't exist, then show the large ones.
        Bottom,
        /// If any values larger than `options.height * 8` exist in `data + compare.data`,
        /// then display them. Smaller values will be indicated by `🢃`.
        /// If larger values don't exist, then show the small ones.
        Top,
    }

    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum DisplayMode<'a> {
        /// Just the chart.
        Compact,
        /// Chart with labels at the bottom. One label is expected for each data point.
        Portrait { labels: &'a [&'a str] },
    }

    #[derive(Debug)]
    #[allow(missing_docs)]
    pub struct ChartOptions<'a> {
        /// The vertical size of the chart, in lines of text.
        pub height: u16,
        /// Determines how outliers are displayed.
        pub view: ViewPreference,
        /// Determines how space surrounding the chart is used.
        pub display: DisplayMode<'a>,
    }

    impl<'a> Default for ChartOptions<'a> {
        fn default() -> Self {
            Self {
                height: 8,
                view: ViewPreference::Top,
                display: DisplayMode::Compact,
            }
        }
    }

    #[derive(Debug)]
    #[allow(missing_docs)]
    pub struct ChartComparison<'a> {
        /// Another slice of values to display next to `chart.data`.
        pub data: &'a [u32],
    }
}

use params::*;

/// Display a slice of up to 100 `u32` values.
pub struct Chart<'a> {
    data: &'a [u32],
    compare: Option<ChartComparison<'a>>,
    options: ChartOptions<'a>,
}

impl<'a> Chart<'a> {
    /// `data` and `compare.data` should have the same length.
    pub fn new(
        data: &'a [u32],
        compare: Option<ChartComparison<'a>>,
        options: ChartOptions<'a>,
    ) -> Self {
        assert!(
            (1..=100).contains(&data.len()),
            // supports charts up to 200 characters in width
            "data should contain no more than 100 values"
        );
        if let Some(ref compare) = compare {
            assert_eq!(
                compare.data.len(),
                data.len(),
                "compare data length should equal primary data length",
            )
        }
        if let DisplayMode::Portrait { labels } = options.display {
            assert_eq!(
                labels.len(),
                data.len(),
                "label count should equal data length",
            );
        }

        Self {
            data,
            compare,
            options,
        }
    }

    fn render(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ChartOptions {
            height: height_lines,
            display,
            ..
        } = &self.options;

        // determine the character width to use for each bar based on
        // how many characters are required to label it with a numeric offset
        let bar_width_chars = if self.data.len() <= 10 { 1 } else { 2 };

        // display heights are calculated in terms of "steps"
        // a step is the height of this character: ▁ (8 per line)
        let chars = " ▁▂▃▄▅▆▇█🢃🢁⨯".chars().collect::<Vec<_>>();

        let (data_steps, cmp_data_steps) = self.scale_to_steps();
        let steps_zipped: Vec<(&i16, Option<&i16>)> = match cmp_data_steps {
            Some(ref cmp_data_steps) => data_steps
                .iter()
                .zip(cmp_data_steps.iter())
                .map(|(a, b)| (a, Some(b)))
                .collect(),
            None => data_steps.iter().map(|a| (a, None)).collect(),
        };

        // determine the largest and smallest values displayed,
        // for indicating the range of values next to the chart
        let find_min_max = |data: &[u32], steps: &Vec<i16>| -> (u32, u32) {
            let mut min = u32::MAX;
            let mut max = u32::MIN;
            for i in 0..data.len() {
                if steps[i] > 0 {
                    if data[i] < min {
                        min = data[i];
                    }
                    if data[i] > max {
                        max = data[i];
                    }
                }
            }
            (min, max)
        };
        let (mut min_visible, mut max_visible) = find_min_max(self.data, &data_steps);
        if let (Some(c), Some(steps)) = (self.compare.as_ref(), &cmp_data_steps) {
            let (min_visible_cmp, max_visible_cmp) = find_min_max(c.data, steps);
            min_visible = std::cmp::min(min_visible, min_visible_cmp);
            max_visible = std::cmp::max(max_visible, max_visible_cmp);
        }

        let tick_spacer = max_visible
            .to_string()
            .chars()
            .map(|_| " ")
            .collect::<String>();

        // print data_steps in layers, from top to bottom.
        // each layer corresponds to a line of text, or 8 steps
        for layer_num in (0..*height_lines).rev() {
            if layer_num == height_lines - 1 {
                write!(f, "{max_visible}│")?;
            } else if layer_num == 0 {
                let gap = (0..tick_spacer.len() - min_visible.to_string().len())
                    .map(|_| " ")
                    .collect::<String>();
                write!(f, "{gap}{min_visible}│")?;
            } else {
                write!(f, "{tick_spacer}│")?;
            };

            // determine the range of steps corresponding to this layer
            // examples: (16,24] (8,16] (0,8]
            let print_steps_start = (layer_num * 8) as i16;
            let print_steps_end = ((layer_num + 1) * 8) as i16;

            // print a layer of each bar, from left to right
            let to_print_char = |steps_count: i16| -> char {
                match steps_count {
                    0 if layer_num == 0 => chars[11],
                    -1 if layer_num == 0 => chars[9],
                    -2 => chars[10],
                    below if below <= print_steps_start => chars[0],
                    above if above > print_steps_end => chars[8],
                    value => chars[(value - print_steps_start) as usize],
                }
            };
            for (i, &(&pri_steps, cmp_steps)) in steps_zipped.iter().enumerate() {
                match cmp_steps {
                    None => {
                        let pri_char = if i % 2 == 0 {
                            to_print_char(pri_steps).to_string().bright_white()
                        } else {
                            to_print_char(pri_steps).to_string().white()
                        };
                        for _ in 0..bar_width_chars {
                            write!(f, "{pri_char}")?;
                        }
                    }
                    // if comparison, each bar only needs to be 1 character wide
                    // for offsets to fit at the bottom
                    Some(&cmp_steps) => {
                        write!(f, "{}", to_print_char(pri_steps).to_string().bright_white())
                            .unwrap();

                        let pri_value = self.data[i];
                        let cmp_value = self.compare.as_ref().unwrap().data[i];
                        let cmp_char = if cmp_value <= pri_value {
                            to_print_char(cmp_steps).to_string().bright_green()
                        } else {
                            to_print_char(cmp_steps).to_string().bright_red()
                        };
                        write!(f, "{cmp_char} ",).unwrap();
                    }
                }
            }

            // move to layer below
            writeln!(f)?;
        }

        // print offsets
        write!(f, "{tick_spacer} ")?;
        let mut chart_width = tick_spacer.len() as u16;
        for i in 0..self.data.len() {
            write!(f, "{i}")?;
            let label_width = i.to_string().len() as u16;
            chart_width += label_width;
            let offset_width = match (self.compare.is_some(), bar_width_chars) {
                (false, width) => width,
                (true, _) => 3,
            };
            for _ in label_width..offset_width {
                write!(f, " ")?;
                chart_width += 1;
            }
        }

        if let DisplayMode::Portrait { labels } = display {
            writeln!(f)?;

            // split labels into evenly-sized columns
            // so as to fill horizontal space below the chart
            // each label is allowed 12 characters before being truncated to fit
            let col_count = std::cmp::max((chart_width as f32 / 17f32).floor() as usize, 1usize);
            let col_length = labels.len().div_ceil(col_count);
            let enumerated_labels = labels.iter().enumerate().collect::<Vec<_>>();
            let label_cols = enumerated_labels.chunks(col_length).collect::<Vec<_>>();
            let max_rows = label_cols.iter().map(|c| c.len()).max().unwrap();

            for i in 0..max_rows {
                for col in &label_cols {
                    if let Some((offset, label)) = col.get(i) {
                        write!(
                            f,
                            // each column requires 17 characters
                            "{offset:>2}: {:<12} ",
                            label.chars().take(12).collect::<String>()
                        )?;
                    }
                }
                writeln!(f)?;
            }
        } else {
            writeln!(f)?;
        }

        Ok(())
    }

    fn scale_to_steps(&self) -> (Vec<i16>, Option<Vec<i16>>) {
        // determine the largest possible measurement that can be expressed within
        // `height` lines, in terms of steps.
        let max_step_count: u16 = self.options.height * 8;

        // determine the factor by which to scale all measurements,
        // so that the largest one fills the available vertical space.
        let all_measurements = self
            .data
            .iter()
            .chain(self.compare.as_ref().map(|c| c.data).unwrap_or(&[]).iter())
            .filter(|&&m| m > 0);
        let all_max = all_measurements.clone().max().unwrap();
        let unit_height_steps: u16 = std::cmp::max(
            (max_step_count as f32 / *all_max as f32).floor() as u16,
            1u16,
        );

        // determine which measurements can not be expressed in terms of steps
        // without additional scaling
        let (excessive, unexcessive) = all_measurements.clone().partition::<Vec<&u32>, _>(|&&m| {
            m > u16::MAX as u32 || m as u16 * unit_height_steps > max_step_count
        });
        let low_max = unexcessive.iter().max();
        let high_max = excessive.iter().max();

        // additional scale factor
        let (show_excessive, scale_factor) = match (&self.options.view, low_max, high_max) {
            // fit the chart to the largest small value
            (ViewPreference::Bottom, Some(&&low_max), _)
            | (ViewPreference::Top, Some(&&low_max), None) => {
                (false, max_step_count as f32 / low_max as f32)
            }
            // the fit the chart to the largest large value
            (ViewPreference::Top, _, Some(&&high_max))
            | (ViewPreference::Bottom, _, Some(&&high_max)) => {
                (true, max_step_count as f32 / high_max as f32)
            }
            _ => unimplemented!(),
        };

        // convert measurements to step counts, expressed as signed integer
        // to allow indication of too small (-1) and too large (-2)
        let measurement_to_step_count = |m: &u32| -> i16 {
            if *m == 0 {
                return 0;
            }
            match (excessive.is_empty(), show_excessive, excessive.contains(&m)) {
                // some are excessive and we don't want them and this is one of them
                (false, false, true) => -2i16,
                // some are excessive and we want them, but this isn't one of them
                (false, true, false) => -1i16,
                // otherwise, use the scale factor
                _ => {
                    let step_count = (*m as f32 * scale_factor) as i16;
                    if step_count == 0 {
                        // excessive measurement, but still invisible next to max
                        -1i16
                    } else {
                        step_count
                    }
                }
            }
        };
        let scaled_data: Vec<i16> = self.data.iter().map(measurement_to_step_count).collect();
        let scaled_data_cmp: Option<Vec<i16>> = self
            .compare
            .as_ref()
            .map(|c| c.data.iter().map(measurement_to_step_count).collect());

        (scaled_data, scaled_data_cmp)
    }
}

impl<'a> std::fmt::Display for Chart<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.render(f)
    }
}

#[cfg(test)]
mod tests {
    // cargo test -- --nocapture

    use super::*;

    #[test]
    fn test_two_digit_width() {
        let chart = Chart::new(
            &[23, 32, 44, 0, 2, 44, 5, 23, 42, 29, 16],
            None,
            ChartOptions::default(),
        );
        println!("\ntwo_digit_width\n{chart}");
    }

    #[test]
    fn test_excessive_value_too_small_for_height() {
        let chart = Chart::new(
            &[0, 6837, 18067, 352038],
            None,
            ChartOptions {
                height: 5,
                view: ViewPreference::Top,
                display: DisplayMode::Compact,
            },
        );
        println!("\nexcessive_value_too_small_for_height\n{chart}");
    }

    #[test]
    fn test_prefer_small_but_only_large() {
        let chart = Chart::new(
            &[2332, 3232, 3244, 0],
            None,
            ChartOptions {
                height: 5,
                view: ViewPreference::Bottom,
                display: DisplayMode::Compact,
            },
        );
        println!("\nprefer_small_but_large\n{chart}");
    }

    #[test]
    fn test_prefer_large_but_only_small() {
        let chart = Chart::new(
            &[23, 32, 44, 0],
            None,
            ChartOptions {
                height: 10,
                view: ViewPreference::Top,
                display: DisplayMode::Compact,
            },
        );
        println!("\nprefer_large_but_small\n{chart}");
    }

    #[test]
    fn test_comparison_portrait() {
        let chart = Chart::new(
            &[
                0, 22, 2, 9, 223, 34, 33, 66, 76, 122, 199, 33, 12, 89, 1222, 100,
            ],
            Some(ChartComparison {
                data: &[
                    14, 20, 1, 8, 223, 12, 56, 79, 69, 100, 1122, 33, 45, 9, 9000, 78,
                ],
            }),
            ChartOptions {
                height: 16,
                view: ViewPreference::Bottom,
                display: DisplayMode::Portrait {
                    labels: &[
                        "first",
                        "second",
                        "third",
                        "fourth",
                        "fifth",
                        "sixth",
                        "seventh",
                        "eighth",
                        "nineth",
                        "tenth",
                        "eleventh",
                        "twelfth",
                        "thirteenth",
                        "fourteenth",
                        "fifteenth",
                        "sixteenth",
                    ],
                },
            },
        );
        println!("\ncomparison_portrait\n{chart}");
    }
}
