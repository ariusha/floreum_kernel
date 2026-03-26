use alloc::vec::Vec;
pub struct TagVec<T>(Vec<Option<T>>);
impl<T> TagVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, value: T) -> u64 {
        if let Some(tag) = self.0.iter().position(|entry| entry.is_none()) {
            *self.0.get_mut(tag).unwrap() = Some(value);
            tag.try_into().unwrap()
        } else {
            self.0.push(Some(value));
            (self.0.len() - 1).try_into().unwrap()
        }
    }
    pub fn get(&self, tag: u64) -> Option<&T> {
        let tag_usize: usize = tag.try_into().unwrap();
        self.0.get(tag_usize)?.as_ref()
    }
    pub fn get_mut(&mut self, tag: u64) -> Option<&mut T> {
        let tag_usize: usize = tag.try_into().unwrap();
        self.0.get_mut(tag_usize)?.as_mut()
    }
    pub fn remove(&mut self, tag: u64) -> Option<T> {
        let tag_usize: usize = tag.try_into().unwrap();
        if self.0.len() == tag_usize + 1 {
            self.0.pop().flatten()
        } else if let Some(entry) = self.0.get_mut(tag_usize) {
            entry.take()
        } else {
            None
        }
    }
}