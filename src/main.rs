use auto_args::AutoArgs;

use chart_relative::{Chart, params::*};

#[derive(Debug, AutoArgs)]
enum ViewOpt {
    Bottom,
    Top,
}

#[derive(Debug, AutoArgs)]
struct Opt {
    /// Maximum number of lines to use for display before scaling. default: 16
    max_height: Option<u16>,
    /// Which end of range to show, if too large to show all. default: bottom
    view: Option<ViewOpt>,
}

fn main() {
    help_intercept();
    let args = Opt::from_args();
    let cols = StdinColumns::new();

    let data_primary = match cols.try_integers(0) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Invalid line. First column should be integers.");
            eprintln!("For more information try --help");
            std::process::exit(1);
        }
    };

    let mut data_compare: Vec<u32> = vec![];
    #[allow(unused_assignments)]
    let mut labels: Vec<&str> = vec![];

    if let Ok(ints) = cols.try_integers(1) {
        data_compare = ints;
        labels = cols.strings(2);
    } else {
        labels = cols.strings(1);
        if !cols.strings(2).is_empty() {
            eprintln!("Invalid line. found: integer string string");
            eprintln!("For more information try --help");
            std::process::exit(1);
        }
    }

    let comparison = if !data_compare.is_empty() {
        Some(ChartComparison {
            data: &data_compare,
        })
    } else {
        None
    };

    let display = if !labels.is_empty() {
        DisplayMode::Portrait { labels: &labels }
    } else {
        DisplayMode::Compact
    };

    let view = match args.view {
        Some(ViewOpt::Top) => ViewPreference::Top,
        _ => ViewPreference::Bottom,
    };

    let mut max = data_primary.iter().max().unwrap();
    if let Some(ChartComparison { data }) = comparison {
        max = std::cmp::max(max, data.iter().max().unwrap());
    }
    let chart = Chart::new(
        &data_primary,
        comparison,
        ChartOptions {
            height: std::cmp::min(*max as u16, args.max_height.unwrap_or(16)),
            view,
            display,
        },
    );

    print!("{chart}");
}

struct StdinColumns {
    cols: [Vec<String>; 3],
}

impl StdinColumns {
    const DOCS: &str = r#"
    The standard input stream should contain 1-3 columns, separated by spaces.
    Input is truncated after 100 lines.

    Either:

    1. Only data
    integer

    2. Labeled data
    integer string

    3. Unlabeled comparison data
    integer integer

    4. Labeled comparison data
    integer integer string
    "#;

    fn new() -> Self {
        let mut col_count = 0;
        let mut col1 = vec![];
        let mut col2 = vec![];
        let mut col3 = vec![];
        for (line_count, line) in std::io::stdin().lines().map_while(Result::ok).enumerate() {
            let row = line.split(' ').map(str::trim).collect::<Vec<_>>();
            if line_count == 0 {
                col_count = row.len();
            } else if line_count > 100 {
                eprintln!("Data truncated to 100 lines");
                break;
            }

            match (col_count, row.as_slice()) {
                (1, [c1]) => {
                    col1.push(c1.to_string());
                }
                (2, [c1, c2]) => {
                    col1.push(c1.to_string());
                    col2.push(c2.to_string());
                }
                (3, [c1, c2, c3]) => {
                    col1.push(c1.to_string());
                    col2.push(c2.to_string());
                    col3.push(c3.to_string());
                }
                (cols, _) => {
                    if cols > 3 {
                        eprintln!(
                            "Invalid line '{line}': found {col_count} items but expected no more than 3."
                        );
                    } else {
                        eprintln!(
                            "Invalid line '{line}': expected {col_count} items, based on the first line of input."
                        );
                    }
                    eprintln!("For more information try --help");
                    std::process::exit(2);
                }
            }
        }

        Self {
            cols: [col1, col2, col3],
        }
    }

    fn try_integers(&self, i: usize) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut result = vec![];
        for c in self.cols[i].iter() {
            result.push(c.parse()?);
        }
        Ok(result)
    }

    fn strings(&self, i: usize) -> Vec<&str> {
        self.cols[i].iter().map(|s| s.as_str()).collect()
    }
}

fn help_intercept() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.contains(&"--help".to_string()) {
        let msg_lines = Opt::help();
        let mut msg_lines = msg_lines
            .lines()
            .filter(|l| *l != "For more information try --help")
            .collect::<Vec<_>>();
        msg_lines.append(&mut vec!["Standard Input:", StdinColumns::DOCS]);
        eprintln!("{}", msg_lines.join("\n"));
        std::process::exit(1);
    }
}
