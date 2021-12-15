/// A rectangular grid of objects, randomly addressable.
#[derive(Clone)]
pub struct ArrayGrid<T>
where
    T: Clone + Default,
{
    width: usize,
    height: usize,
    arr: Vec<T>,
}

#[allow(dead_code)]
impl<T> ArrayGrid<T>
where
    T: Clone + Default,
{
    pub fn create(width: usize, height: usize) -> ArrayGrid<T> {
        let vec_size = width * height;
        let mut arr: Vec<T> = Vec::with_capacity(vec_size);
        arr.resize(vec_size, T::default());

        ArrayGrid { width, height, arr }
    }

    pub fn create_square(size: usize) -> ArrayGrid<T> {
        ArrayGrid::create(size, size)
    }

    pub fn create_from(width: usize, height: usize, arr: Vec<T>) -> ArrayGrid<T> {
        debug_assert_eq!(arr.len(), width * height);

        ArrayGrid { width, height, arr }
    }

    pub fn set(&mut self, xu: usize, yu: usize, val: T) {
        let idx = self.get_index(xu, yu);
        self.arr[idx] = val;
    }

    pub fn get(&self, xu: usize, yu: usize) -> &T {
        let idx = self.get_index(xu, yu);
        &self.arr[idx]
    }

    pub fn get_checked(&self, x: i64, y: i64) -> Option<&T> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            None
        } else {
            Some(self.get(x as usize, y as usize))
        }
    }

    pub fn get_mut(&mut self, xu: usize, yu: usize) -> &mut T {
        let idx = self.get_index(xu, yu);
        &mut self.arr[idx]
    }

    fn get_index(&self, xu: usize, yu: usize) -> usize {
        xu + yu * self.width
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.arr.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = Box<dyn Iterator<Item = &T> + '_>> + '_ {
        self.arr.chunks(self.width).map(|chunks| {
            let row: Box<dyn Iterator<Item = &T>> = Box::new(chunks.iter());
            row
        })
    }

    pub fn columns(&self) -> impl Iterator<Item = Box<dyn Iterator<Item = &T> + '_>> + '_ {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.width {
                let col: Box<dyn Iterator<Item = &T>> = Box::new(self.get_column(index));
                index += 1;
                Some(col)
            } else {
                None
            }
        })
    }

    pub fn get_row(&self, row_idx: usize) -> impl Iterator<Item = &T> {
        let start = row_idx * self.width;
        let end = start + self.width;
        self.arr[start..end].iter()
    }

    pub fn get_column(&self, col_idx: usize) -> impl Iterator<Item = &T> {
        self.arr[col_idx..].iter().step_by(self.width)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn rows_iterator_works() {
        let mut grid: ArrayGrid<u16> = ArrayGrid::create_square(3);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);
        grid.set(2, 0, 3);
        grid.set(0, 1, 4);
        grid.set(1, 1, 5);
        grid.set(2, 1, 6);
        grid.set(0, 2, 7);
        grid.set(1, 2, 8);
        grid.set(2, 2, 9);

        let rows: Vec<Vec<&u16>> = grid.rows().map(|r| r.collect()).collect();
        assert_eq!(3, rows.len());
        assert_eq!(1, *rows[0][0]);
        assert_eq!(2, *rows[0][1]);
        assert_eq!(3, *rows[0][2]);
        assert_eq!(4, *rows[1][0]);
        assert_eq!(5, *rows[1][1]);
        assert_eq!(6, *rows[1][2]);
        assert_eq!(7, *rows[2][0]);
        assert_eq!(8, *rows[2][1]);
        assert_eq!(9, *rows[2][2]);
    }
}
