#[derive(Debug)]
pub struct Pos {
  pub x: usize,
  pub y: usize
}

#[derive(Debug)]
pub struct Stage (pub Vec<Vec<usize>>, pub Vec<Pos>);

pub fn new (
  width: usize,
  height: usize,
  walls: Vec<Pos>
) -> Stage {
  Stage(vec![vec![0; width]; height], walls)
}
