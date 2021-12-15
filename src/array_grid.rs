/// A square grid of objects, randomly addressable.
#[derive(Clone)]
pub struct ArrayGrid<T>
where
    T: Clone + Default,
{
    size: usize,
    arr: Vec<T>,
}

#[allow(dead_code)]
impl<T> ArrayGrid<T>
where
    T: Clone + Default,
{
    pub fn create(size: usize) -> ArrayGrid<T> {
        let vec_size = size.pow(2);
        let mut arr: Vec<T> = Vec::with_capacity(vec_size);
        arr.resize(vec_size, T::default());

        ArrayGrid { size, arr }
    }

    pub fn set(&mut self, xu: usize, yu: usize, val: T) {
        let idx = self.get_index(xu, yu);
        self.arr[idx] = val;
    }

    pub fn get(&self, xu: usize, yu: usize) -> &T {
        let idx = self.get_index(xu, yu);
        &self.arr[idx]
    }

    pub fn get_mut(&mut self, xu: usize, yu: usize) -> &mut T {
        let idx = self.get_index(xu, yu);
        &mut self.arr[idx]
    }

    fn get_index(&self, xu: usize, yu: usize) -> usize {
        xu + yu * self.size
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.arr.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = Box<dyn Iterator<Item = &T> + '_>> + '_ {
        self.arr.chunks(self.size).map(|chunks| {
            let row: Box<dyn Iterator<Item = &T>> = Box::new(chunks.iter());
            row
        })
    }

    pub fn columns(&self) -> impl Iterator<Item = Box<dyn Iterator<Item = &T> + '_>> + '_ {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.size {
                let col: Box<dyn Iterator<Item = &T>> = Box::new(self.get_column(index));
                index += 1;
                Some(col)
            } else {
                None
            }
        })
    }

    pub fn get_row(&self, row_idx: usize) -> impl Iterator<Item = &T> {
        let start = row_idx * self.size;
        let end = start + self.size;
        self.arr[start..end].iter()
    }

    pub fn get_column(&self, col_idx: usize) -> impl Iterator<Item = &T> {
        self.arr[col_idx..].iter().step_by(self.size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn rows_iterator_works() {
        let mut grid: ArrayGrid<u16> = ArrayGrid::create(3);
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
