use colored::*;
use image::{Rgb, Rgba};
use std::io::Error;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Move {
    coord: Coord,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Tile {
    c: char,
    visited: bool,
}

fn convert_char(c: char) -> char {
    match c {
        'L' => '╚',
        'F' => '╔',
        '-' => '═',
        '|' => '║',
        'J' => '╝',
        '7' => '╗',
        _ => c,
    }
}
#[derive(Debug, Clone)]
struct Maze(Vec<Vec<Tile>>);

impl Maze {
    fn find_start(&self) -> Option<Coord> {
        for (idx, val) in self.0.iter().enumerate() {
            for (idy, val) in val.iter().enumerate() {
                if val.c == 'S' {
                    return Some(Coord { x: idx, y: idy });
                }
            }
        }
        None
    }

    fn is_linked(&self, coord: &Coord, direction: &Direction) -> Option<Coord> {
        match direction {
            Direction::N => {
                let point = self.0[coord.x - 1][coord.y].c;
                if point == '|' || point == '7' || point == 'F' || point == 'S' {
                    return Some(Coord {
                        x: coord.x - 1,
                        y: coord.y,
                    });
                }
                None
            }
            Direction::E => {
                let point = self.0[coord.x][coord.y + 1].c;
                if point == '-' || point == '7' || point == 'J' || point == 'S' {
                    return Some(Coord {
                        x: coord.x,
                        y: coord.y + 1,
                    });
                }
                None
            }
            Direction::S => {
                let point = self.0[coord.x + 1][coord.y].c;
                if point == '|' || point == 'L' || point == 'J' || point == 'S' {
                    return Some(Coord {
                        x: coord.x + 1,
                        y: coord.y,
                    });
                }
                None
            }
            Direction::W => {
                let point = self.0[coord.x][coord.y - 1].c;
                if point == '-' || point == 'L' || point == 'F' || point == 'S' {
                    return Some(Coord {
                        x: coord.x,
                        y: coord.y - 1,
                    });
                }
                None
            }
        }
    }

    fn get_coord_from_direction(&self, coord: &Coord, direction: &Direction) -> Option<Coord> {
        match direction {
            Direction::N => {
                if coord.x > 0 {
                    Some(Coord {
                        x: coord.x - 1,
                        y: coord.y,
                    })
                } else {
                    None
                }
            }
            Direction::E => {
                if coord.y < self.0[0].len() {
                    Some(Coord {
                        x: coord.x,
                        y: coord.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::S => {
                if coord.x < self.0.len() {
                    Some(Coord {
                        x: coord.x + 1,
                        y: coord.y,
                    })
                } else {
                    None
                }
            }
            Direction::W => {
                if coord.y > 0 {
                    Some(Coord {
                        x: coord.x,
                        y: coord.y - 1,
                    })
                } else {
                    None
                }
            }
        }
    }

    fn get_available_path(
        &self,
        coord: &Coord,
        last_direction: &Direction,
        visited: &[Coord],
    ) -> Option<(Coord, Direction)> {
        let curr_pipe = self.0[coord.x][coord.y].c;

        let next_direction = if curr_pipe == '|' || curr_pipe == '-' {
            Some(*last_direction)
        } else if curr_pipe == 'L' {
            if *last_direction == Direction::S {
                Some(Direction::E)
            } else {
                Some(Direction::N)
            }
        } else if curr_pipe == 'J' {
            if *last_direction == Direction::S {
                Some(Direction::W)
            } else {
                Some(Direction::N)
            }
        } else if curr_pipe == '7' {
            if *last_direction == Direction::N {
                Some(Direction::W)
            } else {
                Some(Direction::S)
            }
        } else if curr_pipe == 'F' {
            if *last_direction == Direction::N {
                Some(Direction::E)
            } else {
                Some(Direction::S)
            }
        } else {
            None
        };

        if let Some(next_direction) = next_direction {
            // let next_direction_ref = next_direction.as_ref();

            let next_coord = self
                .get_coord_from_direction(coord, &next_direction)
                .unwrap();

            if !visited
                .iter()
                .any(|c| c.x == next_coord.x && c.y == next_coord.y)
                && self.is_linked(coord, &next_direction).is_some()
            {
                // println!(
                //     "Next coord is {:?} direction {:?}",
                //     next_coord, next_direction
                // );
                return Some((next_coord, next_direction));
            }
        }

        None
    }

    fn display(&self) {
        // for i in 0..self.0[0].len() {
        //     print!("#");
        // }
        // print!("\n");
        // Create a blank image with a white background

        let mut img = image::ImageBuffer::<Rgba<u8>, Vec<u8>>::new(
            self.0[0].len() as u32,
            self.0.len() as u32,
        );

        // // Set some pixels to red
        // for y in 100..200 {
        //     for x in 300..500 {
        //         img.put_pixel(x, y, Rgba([255, 0, 0, 255])); // RGBA color (red)
        //     }
        // }

        // Save the image to a file

        for (x, l) in self.0.iter().enumerate() {
            for (y, c) in l.iter().enumerate() {
                let pixel = img.get_pixel_mut(x as u32, y as u32);
                if c.visited {
                    // print!("{}", convert_char(c.c).to_string().red());
                    // img.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255])); // RGBA color (red)
                    *pixel = Rgba([255, 0, 0, 255]);
                    // img.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]))
                    img.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
                } else {
                    img.put_pixel(x as u32, y as u32, Rgba([0, 255, 0, 255]));
                    // img.put_pixel(x as u32, y as u32, Rgb([0, 255, 0]));

                    // img.put_pixel(x as u32, y as u32, Rgba([255, 255, 0, 255]));
                    // RGBA color (red)
                }
            }
            print!("\n");
        }
        img.save("output.png").unwrap();
        // for i in 0..self.0[0].len() {
        //     print!("#");
        // }
        // print!("\n");
    }

    fn scan(&mut self) {
        let mut tiles_enclosed = 0;

        let mut top_wall: Vec<Coord> = Vec::new();
        let mut bottom_wall: Vec<Coord> = Vec::new();
        let mut left_walls: Vec<Coord> = Vec::new();
        let mut right_walls: Vec<Coord> = Vec::new();
        for (idcol, c) in self.0[0].iter().enumerate() {
            for (idlig, l) in self.0.iter().enumerate() {
                if self.0[idlig][idcol].visited {
                    top_wall.push(Coord { x: idlig, y: idcol });
                    break;
                }
            }
        }

        for (idcol, c) in self.0[0].iter().enumerate() {
            for (idlig, l) in self.0.iter().enumerate().rev() {
                if self.0[idlig][idcol].visited {
                    bottom_wall.push(Coord { x: idlig, y: idcol });
                    break;
                }
            }
        }

        for (idlig, l) in self.0.iter().enumerate() {
            for (idcol, l) in l.iter().enumerate() {
                if self.0[idlig][idcol].visited {
                    left_walls.push(Coord { x: idlig, y: idcol });
                    break;
                }
            }
        }

        for (idlig, l) in self.0.iter().enumerate() {
            for (idcol, l) in l.iter().enumerate().rev() {
                if self.0[idlig][idcol].visited {
                    right_walls.push(Coord { x: idlig, y: idcol });
                    break;
                }
            }
        }

        for (idx, l) in self.0.iter().enumerate() {
            let mut left_wall = false;
            let (lastwall, _) = l
                .iter()
                .enumerate()
                .filter(|(_, c)| c.visited)
                .last()
                .unwrap();
            for (idy, c) in l.iter().enumerate() {
                if idy == 0 {
                    continue;
                }
                if c.visited {
                    if top_wall
                        .iter()
                        .find(|coord| coord.x == idx && coord.y == idy)
                        .is_some()
                        || bottom_wall
                            .iter()
                            .find(|coord| coord.x == idx && coord.y == idy)
                            .is_some()
                        || left_walls
                            .iter()
                            .find(|coord| coord.x == idx && coord.y == idy)
                            .is_some()
                        || right_walls
                            .iter()
                            .find(|coord| coord.x == idx && coord.y == idy)
                            .is_some()
                    {
                        print!("{}", c.c.to_string().yellow().to_string());
                    } else {
                        print!("{}", c.c.to_string().blue().to_string());
                    }
                    if !left_wall || idy < lastwall {
                        left_wall = true;
                    } else {
                        left_wall = false;
                    }
                } else {
                    let before_left_wall = left_walls
                        .iter()
                        .find(|w| w.x == idx && idy < w.y)
                        .is_some();

                    let before_top_wall =
                        top_wall.iter().find(|w| idx < w.x && idy == w.y).is_some();

                    let after_bottom_wall = bottom_wall
                        .iter()
                        .find(|w| idx > w.x && idy == w.y)
                        .is_some();

                    let after_right_wall = right_walls
                        .iter()
                        .find(|w| idx == w.x && idy > w.y)
                        .is_some();

                    if left_wall
                        && !before_left_wall
                        && !before_top_wall
                        && !after_bottom_wall
                        && !after_right_wall
                    {
                        tiles_enclosed += 1;
                        print!("{}", "I".green().to_string());
                    } else {
                        print!("{}", self.0[idx][idy].c.to_string().red().to_string());
                    }
                }
            }
            print!("\n");
        }
        println!("Tiles {}", tiles_enclosed);
    }

    fn scan_new(&self) {
        let total_internal_nodes = 0;
        for y in 0..self.0[0].len() {
            let row_internal_nodes = 0;

            // Here, we're going to count the number of internal nodes in this row.
            // We do this by keeping track of whether we're in the "inside" or "outside"
            // sections of the loop.

            // Inside or outside the loop?
            let is_inside_loop = false;
            // Are we currently on the loop itself?
            let is_on_cycle = false;
            // When we got on the loop, were we entering from the top ('L') or bottom ('F')?
            let entered_cycle_from_bottom = false;
            for x in 0..self.0.len() {
                let node = self.0[x][y];
                
            }
        }
    }
}

fn parse_input(input: Vec<String>) -> Maze {
    Maze(
        input
            .iter()
            .map(|line| line.chars().map(|c| Tile { c, visited: false }).collect())
            .collect(),
    )
}

fn part1(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;
    let maze = parse_input(lines);
    let start = maze.find_start().unwrap();
    println!("start coords {:?}", start);

    let mut paths: Vec<Move> = Vec::new();
    if start.x > 0 && maze.is_linked(&start, &Direction::N).is_some() {
        println!("can start at north");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::N).unwrap(),
            direction: Direction::N,
        });
    }

    if start.y < maze.0[0].len() && maze.is_linked(&start, &Direction::E).is_some() {
        println!("can start at east");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::E).unwrap(),
            direction: Direction::E,
        });
    }

    if start.x < maze.0.len() && maze.is_linked(&start, &Direction::S).is_some() {
        println!("can start at south");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::S).unwrap(),
            direction: Direction::S,
        });
    }

    if start.y > 0 && maze.is_linked(&start, &Direction::W).is_some() {
        println!("can start at west");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::W).unwrap(),
            direction: Direction::W,
        });
    }

    let mut distances: Vec<i32> = Vec::new();

    for path in paths {
        let mut moving = true;
        let mut last_move = path.clone();
        // let mut last_direction = Direction::N;
        let mut distance = 1;
        let mut visited: Vec<Coord> = Vec::new();
        visited.push(path.coord.clone());
        while moving {
            distance += 1;

            if maze.0[last_move.coord.x][last_move.coord.y].c == 'S' {
                distance /= 2;
                moving = false;
            }

            let path = maze.get_available_path(&last_move.coord, &last_move.direction, &visited);
            if path.is_none() {
                moving = false;
            } else if let Some(path) = path {
                visited.push(path.0.clone());
                last_move = Move {
                    coord: path.0,
                    direction: path.1,
                };
            }
        }
        println!("distance {}", distance);
        distances.push(distance);
    }

    Ok(*distances.iter().max().unwrap())
}

fn part2(input: String) -> Result<i32, Error> {
    let lines = utils::read_input_file_as_vec(input)?;
    let mut maze = parse_input(lines);
    let start = maze.find_start().unwrap();
    println!("start coords {:?}", start);

    let mut paths: Vec<Move> = Vec::new();
    if start.x > 0 && maze.is_linked(&start, &Direction::N).is_some() {
        println!("can start at north");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::N).unwrap(),
            direction: Direction::N,
        });
    }

    if start.y < maze.0[0].len() && maze.is_linked(&start, &Direction::E).is_some() {
        println!("can start at east");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::E).unwrap(),
            direction: Direction::E,
        });
    }

    if start.x < maze.0.len() && maze.is_linked(&start, &Direction::S).is_some() {
        println!("can start at south");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::S).unwrap(),
            direction: Direction::S,
        });
    }

    if start.y > 0 && maze.is_linked(&start, &Direction::W).is_some() {
        println!("can start at west");
        paths.push(Move {
            coord: maze.is_linked(&start, &Direction::W).unwrap(),
            direction: Direction::W,
        });
    }

    let mut distances: Vec<i32> = Vec::new();

    for path in paths {
        let mut moving = true;
        let mut last_move = path.clone();
        let mut distance = 1;
        let mut visited: Vec<Coord> = Vec::new();
        visited.push(path.coord.clone());

        while moving {
            distance += 1;
            maze.0[last_move.coord.x][last_move.coord.y].visited = true;

            if maze.0[last_move.coord.x][last_move.coord.y].c == 'S' {
                distance /= 2;
                moving = false;
            }

            let path = maze.get_available_path(&last_move.coord, &last_move.direction, &visited);
            if path.is_none() {
                moving = false;
            } else if let Some(path) = path {
                visited.push(path.0.clone());
                last_move = Move {
                    coord: path.0,
                    direction: path.1,
                };
            }
        }
        println!("distance {}", distance);
        distances.push(distance);
    }

    maze.display();

    Ok(*distances.iter().max().unwrap())
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 10 !");
    // let p1_res = part1("day10/src/resources/input.txt".to_owned())?;
    // println!("The result is {}", p1_res);

    let p2_res = part2("day10/src/resources/input.txt".to_owned())?;
    println!("The result is {}", p2_res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn should_get_p1_result() -> Result<(), String> {
        let result = part1("src/resources/test-input.txt".to_owned())
            .map_err(|e| format!("Test failed with error: {:?}", e))?;
        assert_eq!(result, 8);
        Ok(())
    }
}
