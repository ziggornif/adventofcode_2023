use std::io::Error;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone)]
struct Move {
    coord: Coord,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Maze(Vec<Vec<char>>);

impl Maze {
    fn find_start(&self) -> Option<Coord> {
        for (idx, val) in self.0.iter().enumerate() {
            for (idy, val) in val.iter().enumerate() {
                if *val == 'S' {
                    return Some(Coord { x: idx, y: idy });
                }
            }
        }
        None
    }

    fn is_linked(&self, coord: &Coord, direction: &Direction) -> Option<Coord> {
        match direction {
            Direction::N => {
                let point = self.0[coord.x - 1][coord.y];
                if point == '|' || point == '7' || point == 'F' || point == 'S' {
                    return Some(Coord {
                        x: coord.x - 1,
                        y: coord.y,
                    });
                }
                None
            }
            Direction::E => {
                let point = self.0[coord.x][coord.y + 1];
                if point == '-' || point == '7' || point == 'J' || point == 'S' {
                    return Some(Coord {
                        x: coord.x,
                        y: coord.y + 1,
                    });
                }
                None
            }
            Direction::S => {
                let point = self.0[coord.x + 1][coord.y];
                if point == '|' || point == 'L' || point == 'J' || point == 'S' {
                    return Some(Coord {
                        x: coord.x + 1,
                        y: coord.y,
                    });
                }
                None
            }
            Direction::W => {
                let point = self.0[coord.x][coord.y - 1];
                if point == '-' || point == 'L' || point == 'F' || point == 'S' {
                    return Some(Coord {
                        x: coord.x,
                        y: coord.y - 1,
                    });
                }
                None
            }
            _ => None,
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
            _ => None,
        }
    }

    fn get_available_path(
        &self,
        coord: &Coord,
        last_direction: &Direction,
        visited: &[Coord],
    ) -> Option<(Coord, Direction)> {
        let curr_pipe = self.0[coord.x][coord.y];
        let out_coord = Coord { x: 0, y: 0 };

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

        if next_direction.is_none() {
            return None;
        }

        let next_direction_ref = next_direction.as_ref().unwrap();

        let next_coord = self
            .get_coord_from_direction(coord, next_direction_ref)
            .unwrap();

        if !visited
            .iter()
            .any(|c| c.x == next_coord.x && c.y == next_coord.y)
            && self.is_linked(coord, next_direction_ref).is_some()
        {
            return Some((next_coord, next_direction.unwrap()));
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse_input(input: Vec<String>) -> Maze {
    Maze(input.iter().map(|line| line.chars().collect()).collect())
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

            if maze.0[last_move.coord.x][last_move.coord.y] == 'S' {
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

        distances.push(distance);
    }

    Ok(*distances.iter().max().unwrap())
}

fn main() -> Result<(), Error> {
    println!("Hello advent of code day 10 !");
    let p1_res = part1("day10/src/resources/input.txt".to_owned())?;
    println!("The result is {}", p1_res);
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
