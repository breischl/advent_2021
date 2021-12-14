use itertools::Itertools;

pub fn run(input: String) -> Result<String, String> {
    let mut age_tracker = AgeTracker::new(9, 6);
    input
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .for_each(|a| age_tracker.increment(a));

    dump_ages(&age_tracker);
    for _ in 0..256 {
        age_tracker.advance();
        dump_ages(&age_tracker);
    }

    let total_fish: u64 = age_tracker.iter().sum();

    Ok(String::from(format!("fish: {}", total_fish)))
}

fn dump_ages(tracker: &AgeTracker) {
    println!(
        "{}  total={}",
        tracker.iter().join(", "),
        tracker.iter().sum::<u64>()
    );
}

struct AgeTracker {
    storage: Vec<u64>,
    zero_age_idx: usize,
    breeding_age: usize,
}

impl AgeTracker {
    fn new(num_ages: usize, breeding_age: usize) -> AgeTracker {
        let mut vec: Vec<u64> = Vec::with_capacity(num_ages);
        vec.resize(num_ages, 0);

        AgeTracker {
            storage: vec,
            zero_age_idx: 0,
            breeding_age: breeding_age,
        }
    }

    fn advance(&mut self) {
        let num_giving_birth = self.get(0);
        if self.zero_age_idx == self.storage.len() - 1 {
            self.zero_age_idx = 0;
        } else {
            self.zero_age_idx += 1;
        }
        self.add(self.breeding_age, num_giving_birth);
    }

    fn get(&self, age: usize) -> u64 {
        self.storage[self.get_index(age)]
    }

    fn increment(&mut self, age: usize) {
        self.add(age, 1);
    }

    fn add(&mut self, age: usize, amt: u64) {
        let idx = self.get_index(age);
        self.storage[idx] += amt;
    }

    fn get_index(&self, age: usize) -> usize {
        let mut idx = self.zero_age_idx + age;
        if idx >= self.storage.len() {
            idx -= self.storage.len();
        }
        idx
    }

    fn iter(&self) -> impl Iterator<Item = u64> + '_ {
        let mut index: usize = 0;
        std::iter::from_fn(move || {
            if index < self.storage.len() {
                let count = self.get(index);
                index += 1;
                Some(count)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn age_tracker_works() {
        let mut tracker = AgeTracker::new(5, 3);
        tracker.add(4, 5);
        tracker.add(3, 4);
        tracker.add(2, 3);
        tracker.add(1, 2);
        tracker.add(0, 1);

        let ages: Vec<u64> = tracker.iter().collect();
        assert_eq!(1, ages[0]);
        assert_eq!(2, ages[1]);
        assert_eq!(3, ages[2]);
        assert_eq!(4, ages[3]);
        assert_eq!(5, ages[4]);

        tracker.advance();
        let ages: Vec<u64> = tracker.iter().collect();
        assert_eq!(2, ages[0]);
        assert_eq!(3, ages[1]);
        assert_eq!(4, ages[2]);
        assert_eq!(6, ages[3]);
        assert_eq!(1, ages[4]);

        tracker.advance();
        let ages: Vec<u64> = tracker.iter().collect();
        assert_eq!(3, ages[0]);
        assert_eq!(4, ages[1]);
        assert_eq!(6, ages[2]);
        assert_eq!(3, ages[3]);
        assert_eq!(2, ages[4]);
    }
}
