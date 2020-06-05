extern crate piston_window;
extern crate find_folder;
extern crate freetype;

use piston::input::*;
use piston_window::*;
use image::*;
use std::env;

mod stage;

const SIZE: f64 = 80.0;
const PADDING: f64 = 10.0;

const WIDTH: usize = 6;
const HEIGHT: usize = 6;
const WINDOW_SIZE: usize = 600;

fn main() {
  let game = stage::new(WIDTH, HEIGHT, vec![
    stage::Pos{x: 0, y: 0}, stage::Pos{x: 3, y: 0}, stage::Pos{x: 4, y: 0}, stage::Pos{x: 5, y: 0},
    stage::Pos{x: 0, y: 1},
    stage::Pos{x: 0, y: 2},
    stage::Pos{x: 5, y: 3},
    stage::Pos{x: 5, y: 4},
    stage::Pos{x: 0, y: 5}, stage::Pos{x: 1, y: 5}, stage::Pos{x: 2, y: 5}, stage::Pos{x: 5, y: 5}
  ]);

  // 壁になるマスの座標
  let (mut base, walls) = match game {
    stage::Stage(base, walls) => (base, walls),
    _ => panic!()
  };

  // 壁の位置にフラグを立てる
  for id in 0 .. walls.len() {
    let x = walls[id].x;
    let y = walls[id].y;
    base[y][x] = 1;
  }

  let opengl = OpenGL::V3_2;
  let mut window: PistonWindow = WindowSettings::new("2048 Insane", (600, 600))
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .expect("Failed to build window.");

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    // 描画イベント
    if let Some(args) = e.render_args() {
      window.draw_2d(&e, |c, g, _| {
        clear([1.0, 1.0, 1.0, 1.0], g);

        // 枠線
        for rows in 0 .. HEIGHT {
          for columns in 0 .. WIDTH {
            // 奇数は現れない性質を利用して
            // 1を壁の判定とする
            if base[rows][columns] != 1 {
              let x = WINDOW_SIZE as f64 / 2.0 + (columns as f64 - WIDTH as f64 / 2.0) * (SIZE - PADDING);
              let y = WINDOW_SIZE as f64 / 2.0 + (rows as f64 - HEIGHT as f64 / 2.0) * (SIZE - PADDING);

              // 外側
              rectangle(
                [0.733, 0.674, 0.627, 1.0],
                rectangle::square(0.0, 0.0, SIZE - PADDING / 2.0),
                c.transform.trans(x, y),
                g
              );

              // 内側
              rectangle(
                [0.803, 0.752, 0.705, 1.0],
                rectangle::square(0.0, 0.0, SIZE - PADDING * 2.0),
                c.transform.trans(x + PADDING / 1.5, y + PADDING / 1.5),
                g
              );
            }
          }
        }
      });
    }

    // キー入力イベント
    if let Some(args) = e.button_args() {
      if args.state == ButtonState::Press {
        println!("pressed key {:?}" , &args.button);
      }
    }
  }
}
