use std::rc::Rc;

#[derive(Debug)]
pub struct List<T>(Option<Rc<Node<T>>>);

impl<T> List<T> {
    #[inline]
    pub fn empty() -> Self {
        Self(None)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    #[inline]
    pub fn head(&self) -> Option<&T> {
        self.0.as_deref().map(Node::value)
    }

    #[inline]
    pub fn tail(&self) -> Option<&List<T>> {
        self.0.as_deref().map(Node::tail)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.into()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.iter().skip(index).next()
    }

    #[inline]
    pub fn put(self, item: T) -> Self {
        Self(Some(Rc::new(Node::new(item, self))))
    }
}

impl<T> AsRef<List<T>> for List<T> {
    fn as_ref(&self) -> &List<T> {
        &self
    }
}

impl<T> From<T> for List<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(Some(Rc::new(value.into())))
    }
}

impl<T> From<&List<T>> for List<T> {
    #[inline]
    fn from(list: &List<T>) -> Self {
        Self(list.0.clone())
    }
}

pub struct Iter<'i, T> {
    node: Option<&'i Node<T>>,
}

impl<'l, T> From<&'l List<T>> for Iter<'l, T> {
    fn from(list: &'l List<T>) -> Self {
        let node = list.as_ref().0.as_deref();
        Self { node }
    }
}

impl<'i, T> Iterator for Iter<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node.take() {
            None => None,
            Some(n) => {
                self.node = n.tail().0.as_deref();
                Some(n.value())
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    tail: List<T>,
}

impl<T> Node<T> {
    fn new(value: T, tail: List<T>) -> Self {
        Self { value, tail }
    }

    #[inline]
    fn value(&self) -> &T {
        &self.value
    }

    #[inline]
    fn tail(&self) -> &List<T> {
        &self.tail
    }

    #[inline]
    fn take(self) -> (T, List<T>) {
        (self.value, self.tail)
    }
}

impl<T> From<T> for Node<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self { value, tail: List::empty() }
    }
}
