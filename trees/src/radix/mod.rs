//mod node;
//mod bunch;
//mod access;
//mod sync;
//mod link;
//mod trie;
//mod map;
mod sync;

/*pub trait Node {
    type Value;
    type Neighbor;
    type Child;

    fn size(&self) -> usize;
    fn label(&self) -> &[u8];
    fn neighbor(&self) -> Option<&Self::Neighbor>;
    fn child(&self) -> Option<&Self::Child>;
    fn value(&self) -> Option<&Self::Value>;
    fn replace_child(&mut self, new: Option<&mut Self::Child>) -> Option<&mut Self::Child>;
    fn replace_neighbor(&mut self, new: Option<&mut Self::Neighbor>) -> Option<&mut Self::Neighbor>;
    fn replace_value(&mut self, new: Option<Self::Value>) -> Option<Self::Value>;

    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn end(&self) -> *const u8 {
        self.as_ptr() as *mut u8
    }

    fn set_child(&mut self, new: Option<&mut Self::Child>) {
        let _ = self.replace_child(new);
    }

    fn set_neighbor(&mut self, new: Option<&mut Self::Neighbor>) {
        let _ = self.replace_neighbor(new);
    }

    fn set_value(&mut self, new: Option<Self::Value>) {
        let _ = self.replace_value(new);
    }
}*/
