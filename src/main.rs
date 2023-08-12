use rand;
use std::{thread, time};
use rand::Rng;

const ALIVE_CELL: char = '▒';
const DEAD_CELL: char = '░';

#[derive(Debug)]
struct World {
    days: usize,
    width: usize,
    height: usize,
    length: usize,
    cells: Vec<bool>
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let length = (width * height) as usize;
        Self {
            days: 0,
            width,
            height,
            length,
            cells: vec![false; length]
        }
    }

    pub fn alive(&self) -> bool {
        self.cells.contains(&true)
    }

    fn rand() -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<bool>()
    }

    pub fn randomize(&mut self) {
        self.cells = vec![false; self.length].iter_mut().map(|_| Self::rand() ).collect();
    }

    pub fn toggle_cell( &mut self, index: usize) {
        if index < self.length {
            self.cells[index] = !self.cells[index];
        }
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();

        let width = self.width as usize;
        let height = self.height as usize;

        let up = |index: usize| {
            match index / width == 0
            {
                true => ((height - 1) * width) + index, // first-row
                false => index - width
            }
        };

        let down = |index: usize| {
            match index/width == height - 1
            {
                true => index % width, // last-row
                false => index + width
            }
        };

        let left = |index: usize| {
            match index % width == 0
            {
                true => index + (width - 1), // first-column
                false => index - 1
            }
        };

        let right = |index: usize| {
            match index % width + 1 == width
            {
                true => index - (index % width), // last-column
                false => index + 1
            }
        };

        for i in 0 .. self.length {
            let neighbour_count= [
                up(left(i)), up(i), up(right(i)),
                left(i), right(i),
                down(left(i)), down(i), down(right(i))
            ].map(|index|{self.cells[index] as usize}).iter().sum();
            new_cells[i] = Self::is_alive( self.cells[i], neighbour_count );
        }
        self.days += 1;
        self.cells = new_cells;
    }

    pub fn is_alive( is_alive: bool, neighbour_count: usize ) -> bool {
        if is_alive {
            // A live cell dies if it has fewer than two live neighbors.
            if neighbour_count < 2 {
                return false;
            }
            // A live cell with two or three live neighbors lives on to the next generation.
            if neighbour_count == 2 || neighbour_count == 3 {
                return true;
            }
            // A live cell with more than three live neighbors dies.
            if neighbour_count > 3{
                return false;
            }
        }
        // A dead cell will be brought back to live if it has exactly three live neighbors.
        if ! is_alive && neighbour_count == 3 {
            return true;
        }
        false
    }

    pub fn render(&self) {
        let mut index = 0;
        for row in 0 .. self.height {
            for column in 0 .. self.width {
                let cell = match self.cells[index]{
                    true => ALIVE_CELL,
                    false => DEAD_CELL
                };
                print!("{cell}{cell}");
                index += 1;
            }
            print!("\n");
        }
        println!("Days: {}", self.days);
    }
}

fn main() {
    let mut world = World::new(10, 10);
    let update_frequency = time::Duration::from_millis(250);

    world.randomize();
    world.render();

    while world.alive() {
        clear_screen();
        world.tick();
        world.render();
        thread::sleep(update_frequency );
    }
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}