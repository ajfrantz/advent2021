use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Grid<T> {
    x_min: i32,
    x_len: i32,
    y_min: i32,
    y_len: i32,
    map: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(x_min: i32, x_len: i32, y_min: i32, y_len: i32) -> Self {
        let mut map = Vec::new();
        map.resize((x_len * y_len) as usize, T::default());
        Grid {
            x_min,
            x_len,
            y_min,
            y_len,
            map,
        }
    }

    fn coords_to_index(&self, (x, y): (i32, i32)) -> Option<usize> {
        if x < self.x_min || y < self.y_min {
            return None;
        }

        let x = x - self.x_min;
        let y = y - self.y_min;

        if x >= self.x_len || y >= self.y_len {
            return None;
        }

        Some((y * self.x_len + x) as usize)
    }

    pub fn get(&self, coords: (i32, i32)) -> Option<&T> {
        self.coords_to_index(coords).map(|index| &self.map[index])
    }

    pub fn get_mut(&mut self, coords: (i32, i32)) -> Option<&mut T> {
        self.coords_to_index(coords)
            .map(|index| &mut self.map[index])
    }

    pub fn for_each_mut(&mut self, mut f: impl FnMut(&mut T)) {
        for y_off in 0..self.y_len {
            for x_off in 0..self.x_len {
                let index = self
                    .coords_to_index((self.x_min + x_off, self.y_min + y_off))
                    .unwrap();
                f(&mut self.map[index]);
            }
        }
    }

    pub fn cells(&self) -> Cells {
        Cells {
            x: self.x_min,
            y: self.y_min,
            x_min: self.x_min,
            x_max: self.x_min + self.x_len - 1,
            y_max: self.y_min + self.y_len - 1,
        }
    }
}

impl<T> Index<(i32, i32)> for Grid<T>
where
    T: Default + Clone,
{
    type Output = T;
    fn index(&self, coords: (i32, i32)) -> &Self::Output {
        &self.map[self.coords_to_index(coords).unwrap()]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T>
where
    T: Default + Clone,
{
    fn index_mut(&mut self, coords: (i32, i32)) -> &mut Self::Output {
        let index = self.coords_to_index(coords).unwrap();
        &mut self.map[index]
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Default + Clone,
{
    type Output = T;
    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        let coords = (coords.0.try_into().unwrap(), coords.1.try_into().unwrap());
        &self.map[self.coords_to_index(coords.try_into().unwrap()).unwrap()]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Default + Clone,
{
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        let coords = (coords.0.try_into().unwrap(), coords.1.try_into().unwrap());
        let index = self.coords_to_index(coords).unwrap();
        &mut self.map[index]
    }
}

pub struct Cells {
    x: i32,
    y: i32,
    x_min: i32,
    x_max: i32,
    y_max: i32,
}

impl std::iter::Iterator for Cells {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.x_max {
            self.x = self.x_min;
            self.y += 1;
        }
        if self.y > self.y_max {
            return None;
        }
        let result = (self.x, self.y);
        self.x += 1;
        Some(result)
    }
}

pub trait Neighbors {
    type Iter: std::iter::Iterator;
    fn neighbors(&self) -> Self::Iter;
}

impl Neighbors for (i32, i32) {
    type Iter = NeighborsIter;

    fn neighbors(&self) -> Self::Iter {
        NeighborsIter {
            center: *self,
            i: 0,
        }
    }
}

pub struct NeighborsIter {
    center: (i32, i32),
    i: usize,
}

impl Iterator for NeighborsIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i.saturating_add(1);
        match self.i {
            1 => Some((self.center.0 - 1, self.center.1 - 1)),
            2 => Some((self.center.0, self.center.1 - 1)),
            3 => Some((self.center.0 + 1, self.center.1 - 1)),
            4 => Some((self.center.0 - 1, self.center.1)),
            5 => Some((self.center.0 + 1, self.center.1)),
            6 => Some((self.center.0 - 1, self.center.1 + 1)),
            7 => Some((self.center.0, self.center.1 + 1)),
            8 => Some((self.center.0 + 1, self.center.1 + 1)),
            _ => None,
        }
    }
}

pub trait Neighbors4 {
    type Iter: std::iter::Iterator;
    fn neighbors4(&self) -> Self::Iter;
}

impl Neighbors4 for (i32, i32) {
    type Iter = Neighbors4Iter;

    fn neighbors4(&self) -> Self::Iter {
        Neighbors4Iter {
            center: *self,
            i: 0,
        }
    }
}

pub struct Neighbors4Iter {
    center: (i32, i32),
    i: usize,
}

impl Iterator for Neighbors4Iter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i.saturating_add(1);
        match self.i {
            1 => Some((self.center.0, self.center.1 - 1)),
            2 => Some((self.center.0 - 1, self.center.1)),
            3 => Some((self.center.0 + 1, self.center.1)),
            4 => Some((self.center.0, self.center.1 + 1)),
            _ => None,
        }
    }
}
