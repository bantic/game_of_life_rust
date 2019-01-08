type Grid = Vec<Vec<bool>>;

fn main() {
  let mut grid = Vec::new();
  for _ in 0..10 {
    let mut row = Vec::new();
    for j in 0..10 {
      row.push(j % 2 == 0);
    }
    grid.push(row);
  }
  print_grid(grid);
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

  println!(
    "x:{},y:{},alive:{},dead:{},cell:{}",
    x, y, alive, dead, cell
  );

  (alive, dead, cell)
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
