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

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.size {
                let row = self.get_row(index);
                index += 1;
                Some(row)
            } else {
                None
            }
        })
    }

    pub fn columns(&self) -> impl Iterator<Item = Vec<&T>> {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.size {
                let row = self.get_column(index);
                index += 1;
                Some(row)
            } else {
                None
            }
        })
    }

    pub fn get_row(&self, row_idx: usize) -> Vec<&T> {
        self.arr
            .iter()
            .skip(row_idx * self.size)
            .take(self.size)
            .collect()
    }

    pub fn get_column(&self, col_idx: usize) -> Vec<&T> {
        self.arr.iter().skip(col_idx).step_by(self.size).collect()
    }
}
