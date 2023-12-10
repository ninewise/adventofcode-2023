use std::env;
use std::fs::File;
use std::io::Result;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
	let mut args = env::args();
	args.next();
	let _day: String = args.next().unwrap();
	let part: String = args.next().unwrap();
	let file: String = args.next().unwrap();
	let grid: Grid<Tile> = parse(file).unwrap();
	match part.as_str() {
		"1" => part1(grid),
		"2" => part2(zoom(grid)),
		_   => todo!(),
	}
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Dir { North, East, South, West }

impl Dir {
	fn flip(&self) -> Self {
		match self {
			Dir::North => Dir::South,
			Dir::East => Dir::West,
			Dir::South => Dir::North,
			Dir::West => Dir::East,
		}
	}
}

#[derive(Debug, Clone)]
enum Tile { Empty, Any, Pipe(Dir, Dir) }

struct Grid<I> {
	startr: usize,
	startc: usize,
	items: Vec<Vec<I>>,
}

fn parse(filename: String) -> Result<Grid<Tile>> {
	let mut items: Vec<Vec<Tile>> = Vec::new();
	let mut startr = 0;
	let mut startc = 0;
	for (r, line) in BufReader::new(File::open(filename)?).lines().enumerate() {
		let mut row: Vec<Tile> = Vec::new();
		for (c, character) in line?.chars().enumerate() {
			row.push(match character {
				'|' => Tile::Pipe(Dir::North, Dir::South),
				'-' => Tile::Pipe(Dir::East, Dir::West),
				'L' => Tile::Pipe(Dir::North, Dir::East),
				'J' => Tile::Pipe(Dir::North, Dir::West),
				'7' => Tile::Pipe(Dir::South, Dir::West),
				'F' => Tile::Pipe(Dir::South, Dir::East),
				'S' => {
					startr = r;
					startc = c;
					Tile::Any
				},
				_ => Tile::Empty,
			});
		}
		items.push(row);
	}
	Ok(Grid { startr, startc, items })
}

fn part1(grid: Grid<Tile>) {
	// TODO: fout: kan een random andere pijp zijn
	let (mut r, mut c, mut p) = if let Tile::Pipe(_, _) = grid.items[grid.startr + 1][grid.startc] {
		(grid.startr + 1, grid.startc, Dir::North)
	} else if let Tile::Pipe(_, _) = grid.items[grid.startr][grid.startc + 1] {
		(grid.startr, grid.startc + 1, Dir::West)
	} else if grid.startc > 0 {
		(grid.startr, grid.startc - 1, Dir::East)
	} else {
		(grid.startr - 1, grid.startc, Dir::South)
	};
	let mut count: usize = 0;
	while let Tile::Pipe(x, y) = grid.items[r][c] {
		// println!("{} {} {:?} {:?}", r, c, p, grid.items[r][c]);
		p = if x == p { y.flip() } else { x.flip() };
		(r, c) = match p {
			Dir::North => (r + 1, c),
			Dir::East => (r, c - 1),
			Dir::South => (r - 1, c),
			Dir::West => (r, c + 1),
		};
		count += 1;
	}
	// println!("{} {} {:?}", r, c, p);
	println!("{}", count / 2 + 1);
}

#[derive(PartialEq, Debug)]
enum ZoomTile { Start, Blocked, Open, Flooded }

fn zoom(grid: Grid<Tile>) -> Vec<Vec<ZoomTile>> {
	let mut items = Vec::with_capacity(grid.items.len() * 3);
	for (r, row) in grid.items.iter().enumerate() {
		let mut r1 = Vec::with_capacity(row.len() * 3);
		let mut r2 = Vec::with_capacity(row.len() * 3);
		let mut r3 = Vec::with_capacity(row.len() * 3);
		for (c, tile) in row.iter().cloned().enumerate() {
			match tile {
				Tile::Pipe(x, y) => {
					r1.push(ZoomTile::Open);
					r1.push(if x == Dir::North || y == Dir::North { ZoomTile::Blocked } else { ZoomTile::Open });
					r1.push(ZoomTile::Open);
					r2.push(if x == Dir::West || y == Dir::West { ZoomTile::Blocked } else { ZoomTile::Open });
					r2.push(ZoomTile::Blocked);
					r2.push(if x == Dir::East || y == Dir::East { ZoomTile::Blocked } else { ZoomTile::Open });
					r3.push(ZoomTile::Open);
					r3.push(if x == Dir::South || y == Dir::South { ZoomTile::Blocked } else { ZoomTile::Open });
					r3.push(ZoomTile::Open);
				},
				Tile::Any => {
					r1.push(ZoomTile::Open);
					r1.push(if r == 0 { ZoomTile::Open } else { match grid.items[r - 1][c] {
						Tile::Pipe(Dir::South, _) => ZoomTile::Blocked,
						Tile::Pipe(_, Dir::South) => ZoomTile::Blocked,
						_ => ZoomTile::Open,
					}});
					r1.push(ZoomTile::Open);
					r2.push(if c == 0 { ZoomTile::Open } else { match grid.items[r][c - 1] {
						Tile::Pipe(Dir::East, _) => ZoomTile::Blocked,
						Tile::Pipe(_, Dir::East) => ZoomTile::Blocked,
						_ => ZoomTile::Open,
					}});
					r2.push(ZoomTile::Start);
					r2.push(if c + 1 == grid.items[r].len() { ZoomTile::Open } else { match grid.items[r][c + 1] {
						Tile::Pipe(Dir::West, _) => ZoomTile::Blocked,
						Tile::Pipe(_, Dir::West) => ZoomTile::Blocked,
						_ => ZoomTile::Open,
					}});
					r3.push(ZoomTile::Open);
					r3.push(if r + 1 == grid.items.len() { ZoomTile::Open } else { match grid.items[r + 1][c] {
						Tile::Pipe(Dir::North, _) => ZoomTile::Blocked,
						Tile::Pipe(_, Dir::North) => ZoomTile::Blocked,
						_ => ZoomTile::Open,
					}});
					r3.push(ZoomTile::Open);
				},
				Tile::Empty => {
					r1.push(ZoomTile::Open);
					r1.push(ZoomTile::Open);
					r1.push(ZoomTile::Open);
					r2.push(ZoomTile::Open);
					r2.push(ZoomTile::Open);
					r2.push(ZoomTile::Open);
					r3.push(ZoomTile::Open);
					r3.push(ZoomTile::Open);
					r3.push(ZoomTile::Open);
				}
			}
		}
		items.push(r1);
		items.push(r2);
		items.push(r3);
	}
	items
}

#[allow(dead_code)]
fn print2(cr: usize, cc: usize, grid: &Vec<Vec<ZoomTile>>) {
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			if r == cr && c == cc {
				print!("o");
			} else {
				print!("{}", match grid[r][c] {
					ZoomTile::Open => ' ',
					ZoomTile::Flooded => '.',
					ZoomTile::Blocked => 'X',
					ZoomTile::Start => 'S',
				});
			}
		}
		println!();
	}
	println!();
}

fn part2(mut grid: Vec<Vec<ZoomTile>>) {
	let mut stack = Vec::new();
	for r in 0 .. grid.len() {
		stack.push((r, 0));
		stack.push((r, grid[0].len() - 1));
	}
	for c in 1 .. grid[0].len() - 1 {
		stack.push((0, c));
		stack.push((grid.len() - 1, c));
	}
	while let Some((r, c)) = stack.pop() {
		if grid[r][c] == ZoomTile::Open {
			grid[r][c] = ZoomTile::Flooded;
			if r > 0 { stack.push((r - 1, c)) }
			if c > 0 { stack.push((r, c - 1)) }
			if r + 1 < grid.len() { stack.push((r + 1, c)) }
			if c + 1 < grid[r].len() { stack.push((r, c + 1)) }
		}
	}
	// print2(0, 0, &grid);
	let mut count = 0;
	for r in (0..grid.len()).step_by(3) {
		for c in (0..grid[r].len()).step_by(3) {
			if grid[r][c + 0] != ZoomTile::Flooded
				&& grid[r][c + 1] != ZoomTile::Flooded
				&& grid[r][c + 2] != ZoomTile::Flooded
				&& grid[r + 1][c + 0] != ZoomTile::Flooded
				&& grid[r + 1][c + 1] != ZoomTile::Flooded
				&& grid[r + 1][c + 2] != ZoomTile::Flooded
				&& grid[r + 2][c + 0] != ZoomTile::Flooded
				&& grid[r + 2][c + 1] != ZoomTile::Flooded
				&& grid[r + 2][c + 2] != ZoomTile::Flooded {
				count += 1;
			}
		}
	}
	println!("{}", count);
}
