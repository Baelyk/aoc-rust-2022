use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut};
use std::slice::IterMut;
use std::str::FromStr;

pub type UsizeGrid = Grid<usize>;

#[derive(Clone)]
pub struct Grid<T> {
    elements: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(elements: Vec<T>, width: usize) -> Grid<T> {
        Grid { elements, width }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.elements.len() / self.width()
    }

    pub fn size(&self) -> usize {
        self.width() * self.height()
    }

    pub fn first(&self) -> usize {
        0
    }

    pub fn last(&self) -> usize {
        self.size() - 1
    }

    pub fn neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        // Left
        if index % self.width != 0 {
            neighbors.push(index - 1);
        }
        // Right
        if index % self.width < self.width - 1 {
            neighbors.push(index + 1);
        }
        // Up
        if index >= self.width {
            neighbors.push(index - self.width);
        }
        // Down
        if index + self.width < self.size() {
            neighbors.push(index + self.width);
        }
        neighbors
    }

    pub fn neighbors_diag(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let right = index % self.width < self.width - 1;
        let up = index >= self.width;
        let left = index % self.width != 0;
        let down = index + self.width < self.size();

        // Neighbors positive around starting at the right
        if right {
            neighbors.push(index + 1);
        }
        if right && up {
            neighbors.push(index + 1 - self.width);
        }
        if up {
            neighbors.push(index - self.width);
        }
        if left && up {
            neighbors.push(index - 1 - self.width);
        }
        if left {
            neighbors.push(index - 1);
        }
        if left && down {
            neighbors.push(index - 1 + self.width);
        }
        if down {
            neighbors.push(index + self.width);
        }
        if right && down {
            neighbors.push(index + 1 + self.width);
        }

        neighbors
    }

    pub fn get_index(&self, index: usize) -> T
    where
        T: Copy,
    {
        self.elements[index]
    }

    pub fn get_coord(&self, x: usize, y: usize) -> T
    where
        T: Copy,
    {
        let width = self.width();
        self.elements[x + width * y]
    }

    pub fn get_mut_coord(&mut self, x: usize, y: usize) -> &mut T {
        let width = self.width();
        &mut self.elements[x + width * y]
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.elements.iter_mut()
    }

    pub fn elements(&self) -> &Vec<T> {
        &self.elements
    }
}

impl<T: FromStr + fmt::Debug> FromStr for Grid<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            elements: s
                .replace("\n", "")
                .split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<T>().unwrap())
                .collect(),
            width: s.lines().next().unwrap().len(),
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, y: usize) -> &Self::Output {
        let width = self.width();
        let row = y % width;
        let start = row * width;
        &self.elements[start..start + width]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        let width = self.width();
        let row = y % width;
        let start = row * width;
        &mut self.elements[start..start + width]
    }
}

impl<T: Copy + fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .chunks(self.width())
                .fold(String::new(), |s, row| {
                    s + &row.iter().fold(String::new(), |s, &x| s + &x.to_string()) + "\n"
                })
                .trim()
        )
    }
}
