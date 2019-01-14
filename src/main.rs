extern crate cursive;

use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::vec::Vec2;
use cursive::views::{Dialog, LinearLayout};
use cursive::Cursive;
use cursive::Printer;

type Grid = Vec<Vec<bool>>;

fn main() {
  let mut siv = Cursive::default();
  siv.add_global_callback('q', |s| s.quit());
  let mut grid = make_default_grid();
  display_curses_grid(&mut siv, &mut grid);
  siv.run();
}

fn display_curses_grid(siv: &mut cursive::Cursive, grid: &mut Grid) {
  siv.add_layer(
    Dialog::new()
      .title("Game of Life")
      .padding((2, 2, 1, 1))
      .content(LinearLayout::vertical().child(BoardView::new(grid.to_vec()))),
  );
  siv.add_global_callback('n', move |s| {
    grid = tick(grid.to_vec()); // <-- error: expected mutable reference, found struct `std::vec::Vec`
    s.pop_layer();
    s.add_layer(
      Dialog::new()
        .title("Game of Life")
        .padding((2, 2, 1, 1))
        .content(LinearLayout::vertical().child(BoardView::new(grid.to_vec()))),
    );
  });
}

pub fn tick(grid: Grid) -> Grid {
  let len = grid[0].len();
  let mut new_grid = vec![vec![false; len]; len];

  for y in 0..len {
    for x in 0..len {
      let data = neighbor_data(x as i32, y as i32, &grid);
      new_grid[y][x] = match data {
        (alive, _, true) if alive < 2 => false,
        (alive, _, true) => alive == 2 || alive == 3,
        (alive, _, false) => alive == 3,
      };
    }
  }
  new_grid
}

struct BoardView {
  grid: Grid,
}

impl BoardView {
  pub fn new(grid: Grid) -> Self {
    BoardView { grid }
  }
}

impl cursive::view::View for BoardView {
  fn draw(&self, printer: &Printer) {
    for y in 0..self.grid[0].len() {
      for x in 0..self.grid[0].len() {
        let cell = self.grid[y][x];
        let text = if cell { "X" } else { " " };
        let color = if cell {
          Color::RgbLowRes(4, 4, 2)
        } else {
          Color::RgbLowRes(0, 0, 0)
        };
        printer.with_color(
          ColorStyle::new(Color::Dark(BaseColor::Black), color),
          |printer| printer.print((x, y), text),
        );
      }
    }
  }

  fn required_size(&mut self, _: Vec2) -> Vec2 {
    Vec2::new(self.grid[0].len(), self.grid[0].len())
  }
}

fn make_default_grid() -> Grid {
  let mut grid = Vec::new();
  let size = 50;
  for _ in 0..size {
    let mut row = Vec::new();
    for j in 0..size {
      row.push(j % 2 == 0 || j % 7 == 0);
    }
    grid.push(row);
  }
  grid
}

fn print_grid(grid: Grid) {
  let rows: String = grid
    .iter()
    .map(|row| {
      let r: Vec<String> = row
        .iter()
        .map(|cell| match cell {
          true => "X".to_string(),
          false => "_".to_string(),
        })
        .collect::<Vec<String>>();
      r.join("")
    })
    .collect::<Vec<String>>()
    .join("\n");
  println!("{}", rows);
}

fn neighbor_data(x: i32, y: i32, grid: &Grid) -> (i32, i32, bool) {
  let cell: bool = grid[y as usize][x as usize];
  let len = grid[0].len() as i32;
  let mut alive = 0;
  let mut dead = 0;
  for dx in -1..=1 {
    for dy in -1..=1 {
      if dx == 0 && dy == 0 {
        continue;
      }
      let _x = x + dx;
      let _y = y + dy;
      if _x < 0 || _y < 0 {
        continue;
      }
      if _x >= len || _y >= len {
        continue;
      }
      let _cell = grid[_y as usize][_x as usize];
      if _cell {
        alive += 1;
      } else {
        dead += 1;
      }
    }
  }

  // println!(
  //   "x:{},y:{},alive:{},dead:{},cell:{}",
  //   x, y, alive, dead, cell
  // );

  (alive, dead, cell)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn empty_test() {
    assert_eq!(1, 1);
  }

  #[test]
  fn one_alive() {
    let grid = vec![
      vec![false, false, false, false],
      vec![false, true, false, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
    ];

    let expected_grid = vec![
      vec![false, false, false, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
    ];

    assert_eq!(tick(grid), expected_grid);
  }

  #[test]
  fn three_alive() {
    let grid = vec![
      vec![false, true, false, false],
      vec![false, true, true, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
    ];

    let expected_grid = vec![
      vec![false, true, true, false],
      vec![false, true, true, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
    ];

    assert_eq!(tick(grid), expected_grid);
  }

  #[test]
  fn four_alive() {
    let grid = vec![
      vec![false, true, true, false],
      vec![false, true, true, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
    ];

    let expected_grid = vec![
      vec![false, true, true, false],
      vec![false, true, true, false],
      vec![false, false, false, false],
      vec![false, false, false, false],
    ];

    assert_eq!(tick(grid), expected_grid);
  }
}
