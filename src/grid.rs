use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::IterMut;
use std::str::FromStr;

#[derive(Clone)]
pub struct Grid {
    vec: Vec<usize>,
    size: usize,
}

impl Grid {
    pub fn new(elements: Vec<usize>, width: usize) -> Grid {
        Grid {
            vec: elements,
            size: width,
        }
    }

    pub fn width(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        self.vec.len() / self.width()
    }

    pub fn first(&self) -> usize {
        0
    }

    pub fn last(&self) -> usize {
        self.width() * self.height() - 1
    }

    pub fn neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        // Left
        if index % self.size != 0 {
            neighbors.push(index - 1);
        }
        // Right
        if index % self.size < self.size - 1 {
            neighbors.push(index + 1);
        }
        // Up
        if index >= self.size {
            neighbors.push(index - self.size);
        }
        // Down
        if index + self.size < self.vec.len() {
            neighbors.push(index + self.size);
        }
        neighbors
    }

    pub fn neighbors_diag(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let right = index % self.size < self.size - 1;
        let up = index >= self.size;
        let left = index % self.size != 0;
        let down = index + self.size < self.vec.len();

        // Neighbors positive around starting at the right
        if right {
            neighbors.push(index + 1);
        }
        if right && up {
            neighbors.push(index + 1 - self.size);
        }
        if up {
            neighbors.push(index - self.size);
        }
        if left && up {
            neighbors.push(index - 1 - self.size);
        }
        if left {
            neighbors.push(index - 1);
        }
        if left && down {
            neighbors.push(index - 1 + self.size);
        }
        if down {
            neighbors.push(index + self.size);
        }
        if right && down {
            neighbors.push(index + 1 + self.size);
        }

        neighbors
    }

    pub fn get_index(&self, index: usize) -> usize {
        self.vec[index]
    }

    pub fn get_coord(&self, x: usize, y: usize) -> usize {
        let width = self.width();
        self.vec[x + width * y]
    }

    pub fn get_mut_coord(&mut self, x: usize, y: usize) -> &mut usize {
        let width = self.width();
        &mut self.vec[x + width * y]
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, usize> {
        self.vec.iter_mut()
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            vec: s
                .replace("\n", "")
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
            size: s.lines().next().unwrap().len(),
        })
    }
}

impl Index<usize> for Grid {
    type Output = [usize];

    fn index(&self, y: usize) -> &Self::Output {
        let width = self.width();
        let row = y % width;
        let start = row * width;
        &self.vec[start..start + width]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        let width = self.width();
        let row = y % width;
        let start = row * width;
        &mut self.vec[start..start + width]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("{:?}", self.vec);
        write!(
            f,
            "{}",
            self.vec
                .chunks(self.width())
                .fold(String::new(), |s, row| {
                    s + &row.iter().fold(String::new(), |s, &x| s + &x.to_string()) + "\n"
                })
                .trim()
        )
    }
}

//impl IntoIterator for Grid {
//type Item = usize;
//type IntoIter = std::vec::IntoIter<Self::Item>;

//fn into_iter(self) -> Self::IntoIter {
//self.vec.into_iter()
//}
//}
