use serde::*;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Nurikabe {
    pub width: usize,
    pub height: usize,
    pub solved: bool,
    pub iteration: usize,
    pub data: Vec<i32>,
    pub verbose: String,
}

impl Nurikabe {
    pub fn new(width: usize, height: usize, data: Vec<i32>) -> Self {
        Nurikabe {
            width,
            height,
            solved: false,
            iteration: 0,
            data,
            verbose: String::from(""),
        }
    }
}

pub fn load_nurikabe(input: &str) -> color_eyre::Result<Nurikabe> {
    let mut width: usize = 0;
    let mut height: usize = 0;

	const UNKNOWN: i32 = -3;

    let input = input
        .lines()
        .map(|line: &str| -> Result<Vec<_>, _> {
            let values = line
                .split(',')
                .map(|v| {
                    v.trim()
                        .parse::<i32>()
                        .map(|v| if v <= 0 { UNKNOWN } else { v })
                })
                .collect::<Result<Vec<_>, _>>();

            if values.is_ok() && width == 0 {
                width = values.as_ref().ok().unwrap().len();
            }
            height += 1;

            values
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Nurikabe::new(width, height, input.concat()))
}
