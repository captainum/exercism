pub struct SimpleLinkedList<T> where T: Copy + Clone {
    node: Option<Box<Node<T>>>,
}

pub struct Node<T> where T: Copy + Clone {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> SimpleLinkedList<T> where T: Copy + Clone {
    pub fn new() -> Self {
        Self { node: None }
    }

    pub fn is_empty(&self) -> bool {
        self.node.is_none()
    }

    pub fn len(&self) -> usize {
        let mut result = 0;

        let mut current_node = self.node.as_ref();
        loop {
            match current_node {
                Some(val) => {
                    result += 1;
                    current_node = val.next.as_ref();
                },
                None => break result,
            }
        }
    }

    pub fn push(&mut self, _element: T) {
        if self.node.is_none() {
            self.node = Some(
                Box::new(
                    Node {
                        next: None,
                        value: _element,
                    }
                )
            );

            return;
        }

        let mut current_node = self.node.as_mut().unwrap();

        loop {
            match current_node.next {
                Some(ref mut next) => {
                    current_node = next;
                },
                None => {
                    current_node.next = Some(
                        Box::new(
                            Node {
                                next: None,
                                value: _element
                            }
                        )
                    );

                    break;
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut prev_node = self.node.as_mut()?;

        if prev_node.next.is_none() {
            let result = prev_node.value;

            self.node = None;

            return Some(result);
        }

        loop {
            match prev_node.next.as_ref()?.next {
                Some(_) => {
                    prev_node = prev_node.next.as_mut()?;
                },
                None => {
                    let result = prev_node.next.take()?.value;

                    break Some(result);
                },
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let mut current_node = self.node.as_ref()?;

        loop {
            match current_node.next {
                Some(_) => current_node = current_node.next.as_ref()?,
                None => break Some(&current_node.value),
            }
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut last_node: Option<Box<Node<T>>> = None;

        let mut current_node = self.node.as_ref();

        while let Some(node) = current_node {
            let new_node = Some(
                Box::new(
                    Node {
                        next: last_node,
                        value: node.value,
                    }
                )
            );

            last_node = new_node;
            current_node = current_node.unwrap().next.as_ref()
        }

        let mut result = Self::new();
        result.node = last_node;

        result
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> where T: Copy + Clone {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = Self::new();
        let mut current_node = &mut result.node;

        for element in _iter {
            *current_node = Some(
                Box::new(
                    Node {
                        next: None,
                        value: element,
                    }
                )
            );

            current_node = &mut current_node.as_mut().unwrap().next;
        }

        result
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> where T: Copy + Clone {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();

        let mut current_node = _linked_list.node.take();

        while let Some(node) = current_node {
            result.push(node.value);
            current_node = node.next;
        }

        result
    }
}