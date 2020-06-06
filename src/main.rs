extern crate piston_window;
extern crate find_folder;
extern crate freetype;

use std::time::{Duration, SystemTime};
use piston::input::*;
use piston_window::*;
use image::*;
use std::env;

mod stage;
mod step;

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

  // テスト
  base[0][1] = 2;
  base[0][2] = 4;
  base[1][1] = 8;
  base[1][2] = 16;
  base[1][3] = 32;
  base[1][4] = 64;
  base[1][5] = 128;

  // 連続したキー入力を防ぐ（誤操作を防ぐ、負荷をかけない目的）
  let mut key_buffer: u128 = 0;
  let mut key_flag: bool = true;

  // Windowを生成する
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
            let number = base[rows][columns];
            if number != 1 {
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
                match number {
                  2 => [0.933, 0.894, 0.854, 1.0],
                  4 => [0.929, 0.878, 0.784, 1.0],
                  8 => [0.949, 0.694, 0.474, 1.0],
                  16 => [0.956, 0.584, 0.388, 1.0],
                  32 => [0.964, 0.482, 0.372, 1.0],
                  64 => [0.964, 0.368, 0.235, 1.0],
                  128 => [0.929, 0.811, 0.443, 1.0],
                  _ => [0.803, 0.752, 0.705, 1.0]
                },
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
        // println!("pressed key {:?}" , &args.button);

        let dir = match &args.button {
          Button::Keyboard(Key::Left) => 0,
          Button::Keyboard(Key::Up) => 1,
          Button::Keyboard(Key::Right) => 2,
          Button::Keyboard(Key::Down) => 3,
          _ => 4 // Other
        };

        if dir < 4 {
          match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
              let now = n.as_millis();
              let diff = now - key_buffer;

              if diff >= 100 {
                key_buffer = now;
                step::next(&base, dir);
              }
            },

            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
          };

        }
      }
    }
  }
}
