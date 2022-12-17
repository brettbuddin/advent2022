use anyhow::Error;
use anyhow::{Context, Result};

#[derive(rust_embed::RustEmbed)]
#[folder = "data/"]
struct Data;

fn main() -> Result<()> {
    let file = Data::get("example.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    assert_eq!(part1(&data)?, 24);

    let file = Data::get("input.txt").context("file not found")?;
    let data = std::str::from_utf8(file.data.as_ref())?;
    println!("Part 1: {:?}", part1(&data)?);

    Ok(())
}

#[derive(Clone, Debug)]
enum Material {
    Air,
    Sand,
    Rock,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Sand => write!(f, "o"),
            Self::Rock => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn min(&self, o: &Self) -> Self {
        Self(self.0.min(o.0), self.1.min(o.1))
    }

    fn max(&self, o: &Self) -> Self {
        Self(self.0.max(o.0), self.1.max(o.1))
    }
}

fn part1(data: &str) -> Result<u32> {
    let mut entrance = Point(500, 0);
    let mut volume = build_volume(&data, &mut entrance)?;

    let mut units = 0;
    'production: loop {
        let mut new = entrance.clone();
        'stepping: loop {
            match step(&volume, &new) {
                Ok(p) => new = p,
                Err(Stop::InAbyss) => break 'production,
                Err(Stop::AtRest) => break 'stepping,
            }
        }

        // Sand unable to enter the room.
        if new == entrance {
            break;
        }

        volume[new.1][new.0] = Material::Sand;
        units += 1;
    }

    Ok(units)
}

#[derive(Debug)]
enum Stop {
    AtRest,
    InAbyss,
}

fn step(volume: &Vec<Vec<Material>>, p: &Point) -> Result<Point, Stop> {
    if p.0 == 0 {
        return Err(Stop::InAbyss);
    }
    let search = vec![
        Point(p.0, p.1 + 1),
        Point(p.0 - 1, p.1 + 1),
        Point(p.0 + 1, p.1 + 1),
    ];

    for candidate in search {
        let row = volume.get(candidate.1).ok_or(Stop::InAbyss)?;
        let point = row.get(candidate.0).ok_or(Stop::InAbyss)?;
        match point {
            Material::Air => return Ok(candidate),
            Material::Rock | Material::Sand => {}
        }
    }
    Err(Stop::AtRest)
}

fn build_volume(data: &str, entrance: &mut Point) -> Result<Vec<Vec<Material>>> {
    // Build coordinate list so we can find the dimensions of the room.
    let mut ledges = Vec::new();
    let mut min = Point(10_000, 10_000);
    let mut max = Point(0, 0);
    for l in data.lines() {
        let mut ledge: Vec<Point> = Vec::new();
        let points = l.split(" -> ").collect::<Vec<&str>>();
        for p in points {
            let (x, y) = p.split_once(",").context("no comma")?;
            let x = x.parse::<usize>()?;
            let y = y.parse::<usize>()?;

            let p = Point(x, y);
            min = p.min(&min);
            max = p.max(&max);
            ledge.push(p);
        }
        ledges.push(ledge);
    }
    min = entrance.min(&min);
    max = entrance.max(&max);

    // Normalize entrance coordinates to grid.
    (entrance.0, entrance.1) = (entrance.0 - min.0, entrance.1 - min.1);

    // Fill the volume with air.
    let mut volume: Vec<Vec<Material>> = Vec::from(vec![
        vec![Material::Air; 1 + (max.0 - min.0)];
        1 + (max.1 - min.1)
    ]);

    // Draw the ledges.
    for w in &ledges {
        for p in w.windows(2) {
            let mut start = (p[0].0 - min.0, p[0].1 - min.1);
            let mut stop = (p[1].0 - min.0, p[1].1 - min.1);

            // Draw left/right
            if start.0 == stop.0 {
                if stop.1 < start.1 {
                    (start, stop) = (stop, start);
                }
                for y in start.1..=stop.1 {
                    volume[y][start.0] = Material::Rock;
                }
            }

            // Draw up/down
            if start.1 == stop.1 {
                if stop.0 < start.0 {
                    (start, stop) = (stop, start);
                }
                for x in start.0..=stop.0 {
                    volume[start.1][x] = Material::Rock;
                }
            }
        }
    }

    Ok(volume)
}

fn draw_volume(v: &Vec<Vec<Material>>) {
    for row in v {
        for col in row {
            print!("{}", col);
        }
        print!("\n");
    }
}
