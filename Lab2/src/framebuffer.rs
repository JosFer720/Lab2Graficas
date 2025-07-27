pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 100;

#[derive(Clone)]
pub struct Framebuffer {
    pub cells: [[bool; WIDTH]; HEIGHT],
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            cells: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < WIDTH && y < HEIGHT {
            self.cells[y][x] = true;
        }
    }

    pub fn next_generation(&self) -> Self {
        let mut next = Self::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = self.count_alive_neighbors(x, y);
                let alive = self.cells[y][x];

                next.cells[y][x] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        next
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in [-1isize, 0, 1] {
            for dx in [-1isize, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                    if self.cells[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
