#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        let node = Node {
            data: _element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|val| {
            self.head = val.next;
            self.size -= 1;
            val.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&mut self) -> SimpleLinkedList<T> {
        let mut acc = SimpleLinkedList::new();
        while let Some(i) = self.pop() {
            acc.push(i)
        }
        acc
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        items.iter().for_each(|item: &T| list.push(item.clone()));
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut acc = vec![];
        while let Some(i) = self.pop() {
            acc.push(i)
        }
        acc.reverse();
        acc
    }
}
