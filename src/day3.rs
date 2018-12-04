use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(",");
        let x: i32 = it.next().unwrap().parse()?;
        let y: i32 = it.next().unwrap().parse()?;
        Ok(Point {x, y})
    }
}
#[derive(Debug)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}
impl FromStr for Size {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split("x");
        let w: i32 = it.next().unwrap().parse()?;
        let h: i32 = it.next().unwrap().parse()?;
        Ok(Size {h, w})
    }
}
#[derive(Debug)]
pub struct Claim {
    pub id:i32,
    pub pos: Point,
    pub size: Size,
}
impl Claim {
    pub fn points(&self) -> impl Iterator<Item=Point> {
        iproduct!(self.pos.x..self.pos.x+self.size.w, self.pos.y..self.pos.y+self.size.h)
            .map(|(x,y)| Point{x, y})
    }
}
impl FromStr for Claim {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" ");
        let id: i32 = it.next().unwrap().trim_start_matches('#').parse()?;
        it.next().unwrap();
        let pos: Point = it.next().unwrap().trim_end_matches(':').parse()?;
        let size: Size = it.next().unwrap().parse()?;
        Ok(Claim { id, pos, size })
    }
}
#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> Vec<Claim> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> i32 {
    let mut area: HashMap<Point, i32> = HashMap::new();
    for c in input {
        for p in c.points() {
            *area.entry(p).or_default() += 1;
        }
    }
    area.values().filter(|&&v| v > 1).count() as i32
}

#[aoc(day3, part2)]
pub fn part2(input: &[Claim]) -> i32 {
    let mut area: HashMap<Point, i32> = HashMap::new();
    for c in input {
        for p in c.points() {
            *area.entry(p).or_default() += 1;
        }
    }
    'outer: for c in input {
        for p in c.points() {
            if *area.get(&p).unwrap() > 1 {
                continue 'outer;
            }
        }
        return c.id;
    }
    panic!("no match");
}
