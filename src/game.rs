use rand::Rng;

use self::Direction::*;

enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

impl Direction {

    pub fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::TopLeft => (-1, -1),
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::BottomLeft => (-1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomRight => (1, 1),
        }
    }

    pub fn iterator() -> std::slice::Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [TopLeft, Top, TopRight, Left, Right, BottomLeft, Bottom, BottomRight];
        DIRECTIONS.iter()
    }
}


#[derive(Debug)]
pub struct Game {
    pub height: isize,
    pub width: isize,
    state: Vec<bool>,
}

impl Game {
    
    pub fn new(width: isize, height: isize) -> Self {
        let mut state = Vec::with_capacity((width * height) as usize);

        let mut rng = rand::thread_rng();
        for _i in 0..(width * height) {
            state.push(rng.gen());
        }
        
        Game{height, width, state}
    }


    #[inline]
    pub fn index_of(&mut self, x: isize, y: isize) -> usize {
        return (x + (self.width * y)) as usize;
    }

    pub fn get_value(&mut self, x: isize, y: isize) -> Option<bool> {
        if x>0 && x < self.width && y > 0 && y < self.height {
            let pos = self.index_of(x, y);
            Some(self.state[pos])
        } else {
            None
        }
    }

    pub fn get_value_by_index(&mut self, index: usize) -> Option<bool> {
        if index>0 && index< self.state.len() {
            Some(self.state[index])
        }else{
            None
        }
    }

    pub fn update(&mut self){
        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.index_of(x, y);


                // ------------------------------------------//

                let mut neighbours_alive = 0;

                // count neighbours

                for dir in Direction::iterator(){
                    let offset = dir.to_offset();
                    if self.get_value(x + offset.0, y + offset.1).unwrap_or_else(||{false}) {
                       neighbours_alive = neighbours_alive + 1;
                    }
                }
                // ------------------------------------------//

                // apply rules
                let new_val = match &self.state[index] {
                    true => match neighbours_alive {
                        2|3 => true,
                        _ => false
                    },
                    false => match neighbours_alive {
                        3 => true,
                        _ => false
                    },
                };

                // ------------------------------------------//

                // rewrite to array
                self.state[index] = new_val;
            }
        }
    }
}