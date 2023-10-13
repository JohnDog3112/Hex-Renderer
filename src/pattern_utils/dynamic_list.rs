use std::mem;

pub struct DynamicList<T> {
    lower_bound: i32,
    contents: Vec<Option<T>>,
}

const DEFAULT_BOUND: i32 = 10;
impl<T> DynamicList<T> {
    pub fn new() -> DynamicList<T> {
        let mut contents = Vec::new();
        for _ in 0..DEFAULT_BOUND * 2 {
            contents.push(None);
        }
        DynamicList {
            lower_bound: -DEFAULT_BOUND,
            contents,
        }
    }
    pub fn set(&mut self, index: i32, val: T) {
        let upper_bound = self.lower_bound + self.contents.len() as i32;
        let offset_index = (index - self.lower_bound) as usize;

        if index >= self.lower_bound && index < upper_bound {
            self.contents[offset_index] = Some(val);
        } else if index < self.lower_bound {
            let mut old_contents = Vec::new();

            mem::swap(&mut old_contents, &mut self.contents);

            let new_bound = index - DEFAULT_BOUND;
            for _ in 0..self.lower_bound - new_bound {
                self.contents.push(None);
            }
            self.lower_bound = new_bound;

            for item in old_contents {
                self.contents.push(item);
            }

            self.contents[(index - self.lower_bound) as usize] = Some(val);
        } else {
            for _ in 0..index - upper_bound + DEFAULT_BOUND {
                self.contents.push(None);
            }
            self.contents[offset_index] = Some(val);
        }
    }
    pub fn get(&self, index: i32) -> &Option<T> {
        let offset_index = (index - self.lower_bound) as usize;

        if offset_index > self.contents.len() {
            &None
        } else {
            &self.contents[offset_index]
        }
    }
    pub fn into_vector(self) -> Vec<T> {
        self.contents.into_iter().flatten().collect()
    }
}
