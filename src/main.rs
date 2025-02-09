use std::env;

#[derive(Clone, Debug)]
enum Direction {
    LEFT,
    DOWN,
    RIGHT,
    UP,
}

impl Direction {
    fn change(&self) -> Direction {
        let next_direction = match &self {
            Direction::LEFT => Direction::DOWN,
            Direction::DOWN => Direction::RIGHT,
            Direction::RIGHT => Direction::UP,
            Direction::UP => Direction::LEFT,
        };

        next_direction
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn next_step(&self, current_direction: &Direction, n: &i32) -> Option<Point> {
        let (i, j) = match current_direction {
            Direction::LEFT => (self.x, self.y - 1),
            Direction::DOWN => (self.x + 1, self.y),
            Direction::RIGHT => (self.x, self.y + 1),
            Direction::UP => (self.x - 1, self.y),
        };

        let next_point = Point { x: i, y: j };

        if next_point.valid_step(n) {
            Some(next_point)
        } else {
            None
        }
    }

    fn valid_step(&self, n: &i32) -> bool {
        if self.x >= *n || self.y >= *n {
            false
        } else if self.x < 0 || self.y < 0 {
            false
        } else {
            true
        }
    }
}

fn next_direction(
    point: Point,
    current_direction: &Direction,
    matrix: &Vec<Vec<i32>>,
    n: &i32,
) -> Direction {
    let next_direction = match point.next_step(&current_direction, &n) {
        None => current_direction.change(),
        Some(p) => {
            let next_row: usize = p.x.try_into().unwrap_or(0);

            let next_col: usize = p.y.try_into().unwrap_or(0);

            let next_direction = match matrix[next_row][next_col] {
                0 => current_direction.clone(),
                _ => current_direction.change(),
            };

            next_direction
        }
    };

    next_direction
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = &args[1].parse::<i32>().unwrap();

    let mut total = n * n;

    let padding_size = total.to_string().len();

    let bound = n.clone().try_into().unwrap();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; bound]; bound];

    let mut direction = Direction::LEFT;

    let mut point = Point { x: 0, y: n - 1 };

    loop {
        let row: usize = point.x.try_into().unwrap_or_default();

        let col: usize = point.y.try_into().unwrap_or_default();

        matrix[row][col] = total;

        direction = next_direction(point, &direction, &matrix, &n);

        point = point
            .next_step(&direction, &n)
            .expect("Cannot take a step, bailing...");

        total = total - 1;

        if total == 0 {
            break;
        }
    }

    for row in matrix {
        let row_str = row
            .into_iter()
            .map(|i| format!("{:0width$}", i.to_string(), width = padding_size))
            .collect::<Vec<String>>()
            .join(",");

        println!("{row_str}");
    }
}
