pub fn next (
  stage: &Vec<Vec<usize>>,
  dir: usize
) -> Vec<Vec<usize>> {
  // 盤面を1方向に定めるため回転させる
  match dir {
    0 => println!("left"),
    1 => println!("up"),
    2 => println!("right"),
    3 => println!("down"),
    _ => println!("Invalid input")
  };

  // 回転させた上で処理を行う

  // 盤面を返す
  Vec::new()
}
