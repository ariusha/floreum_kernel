use alloc::vec::Vec;
pub struct TagMap<T>(Vec<Option<T>>);
impl<T> TagMap<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, value: T) -> usize {
        if let Some(tag) = self.0.iter().position(|entry| entry.is_none()) {
            self.0[tag] = Some(value);
            tag.try_into().unwrap()
        } else {
            self.0.push(Some(value));
            self.0.len() - 1
        }
    }
    pub fn get(&self, tag: usize) -> Option<&T> {
        self.0.get(tag)?.as_ref()
    }
    pub fn get_mut(&mut self, tag: usize) -> Option<&mut T> {
        self.0.get_mut(tag)?.as_mut()
    }
    pub fn remove(&mut self, tag: usize) -> Option<T> {
        if self.0.len() == tag + 1 {
            self.0.pop().flatten()
        } else if let Some(entry) = self.0.get_mut(tag) {
            entry.take()
        } else {
            None
        }
    }
}
