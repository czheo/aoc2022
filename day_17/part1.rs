use std::cmp::max;

const TOTAL: usize = 2022;

#[derive(Debug)]
enum D {
    L,
    R,
}

struct Shape {
    blocks: Vec<(usize, usize)>
}

struct Game {
    time: usize,
    patterns: Vec<D>,
    shapes: Vec<Shape>,
    space: Vec<Vec<bool>>,
    curr_shape: Shape,
    settled: bool,
    shape_count: usize,
}

impl Game {
    fn new() -> Game {
        let patterns = parse();
        let shapes = shapes();
        let space = vec![];
        Game {
            time: 0,
            patterns: patterns,
            shapes: shapes,
            space: space,
            shape_count: 0,
            settled: true,
            curr_shape: Shape {
                blocks: vec![]
            },
        }
    }

    fn next_tick(&mut self){
        let blocks = if self.settled {
            self.settled = false;
            let s = &self.shapes[self.shape_count % self.shapes.len()];
            self.shape_count += 1;
            s.blocks.iter()
                .map(|(x, y)| (*x, y + self.space.len() + 3))
                .collect()
        } else {
            self.curr_shape.blocks.iter()
                .map(|(x, y)| (*x, y - 1)).collect()
        };
        let dir = &self.patterns[self.time % self.patterns.len()];
        self.time += 1;
        self.curr_shape.blocks = match shift(&dir, &blocks, self) {
            Some(b) => b,
            _ => blocks,
        };
        try_settle(self);
    }

    fn _print(&self) {
        println!("time = {}, next shift = {:?}", self.time, self.patterns[self.time % self.patterns.len()]);
        let mut max_y = 0;
        for (_, y) in self.curr_shape.blocks.iter() {
            max_y = max(max_y, *y + 1);
        }
        max_y = max(max_y, self.space.len());
        for y in (0..max_y).rev() {
            for x in 0..7 {
                if self.curr_shape.blocks.contains(&(x, y)) {
                    if self.settled {
                        print!("#");
                    } else {
                        print!("@");
                    }
                } else if y < self.space.len() && self.space[y][x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!()
    }
}

fn try_settle(game: &mut Game) {
    let mut should_settle = false;
    for (x, y) in game.curr_shape.blocks.iter() {
        if *y == 0 || y - 1 < game.space.len() && game.space[y - 1][*x] {
            should_settle = true;
        }
    }
    game.settled = should_settle;
    if game.settled {
        for (x, y) in game.curr_shape.blocks.iter() {
            while *y >= game.space.len() {
                game.space.push(vec![false;7]);
            }
            game.space[*y][*x] = true;
        }
    }
}

fn shift(dir: &D, blocks: &Vec<(usize, usize)>, game: &Game) -> Option<Vec<(usize, usize)>> {
    let mut v = vec![];
    match dir {
        D::L => {
            for (x, y) in blocks {
                if *x == 0 || *y < game.space.len() && game.space[*y][x - 1] {
                    return None;
                }
                v.push((x - 1, *y));
            }
        },
        D::R => {
            for (x, y) in blocks {
                if *x == 6 || *y < game.space.len() && game.space[*y][x + 1] {
                    return None;
                }
                v.push((x + 1, *y));
            }
        }
    }
    Some(v)
}


fn main() {
    let mut game = Game::new();
    while game.shape_count <= TOTAL {
        game.next_tick();
        // game._print();
    }
    println!("ans = {}", game.space.len());
}

fn shapes() -> Vec<Shape> {
    vec![
        // ..####
        Shape {
            blocks: vec![(2,0), (3,0), (4,0), (5,0)]
        },
        // ...#
        // ..###
        // ...#
        Shape {
            blocks: vec![(3,0), (2,1), (3,1), (4,1), (3,2)]
        },
        // ....#
        // ....#
        // ..###
        Shape {
            blocks: vec![(2,0), (3,0), (4,0), (4,1), (4,2)]
        },
        // ..#
        // ..#
        // ..#
        // ..#
        Shape {
            blocks: vec![(2,0), (2,1), (2,2), (2,3)]
        },
        // ..##
        // ..##
        Shape {
            blocks: vec![(2,0), (3,0), (2,1), (3,1)]
        },
    ]
}

fn parse() -> Vec<D>{
    if let Some(Ok(pattern)) = std::io::stdin().lines().next() {
        pattern.chars().map(|x| match x {
            '<' => D::L,
            '>' => D::R,
            _ => panic!(),
        }).collect()
    } else {
        panic!();
    }
}
