use std::fmt;
use std::option::Option;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            value: value,
            next: next,
        }
    }
}

impl<T> Clone for Node<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            size: 0,
        }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}
/*
impl<T> PartialEq for LinkedList<T>
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    fn eq(&self, another: &LinkedList<T>) -> bool {
        if self.size != another.size {
            return false;
        }
        let ret;
        let mut left = &self.head;
        let mut right = &another.head;
        loop {
            if left.is_some() && right.is_some() {
                let lv = &left.as_ref().unwrap().value;
                let rv = &right.as_ref().unwrap().value;
                if *lv != *rv {
                    ret = false;
                    break;
                }
                left = &left.as_ref().unwrap().next;
                right = &right.as_ref().unwrap().next;
            } else if left.is_none() && right.is_none() {
                ret = true;
                break;
            } else {
                ret = false;
                break;
            }
        }
        ret
    }
}*/
impl<T> PartialEq for LinkedList<T>
where
    T: Clone + std::fmt::Debug + PartialEq,
{
    fn eq(&self, another: &LinkedList<T>) -> bool {
        if self.size != another.size {
            return false;
        }
        let mut ret = true;
        let mut left = &self.head;
        let mut right = &another.head;
        let mut left_val;
        let mut right_val;
        while left.is_some() || right.is_some() {
            match left {
                Some(node) => left_val = &node.value,
                None => {
                    ret = false;
                    break;
                }
            }
            match right {
                Some(node) => right_val = &node.value,
                None => {
                    ret = false;
                    break;
                }
            }

            if *left_val != *right_val {
                ret = false;
                break;
            }

            left = &left.as_ref().unwrap().next;
            right = &right.as_ref().unwrap().next;
        }
        ret
    }
}

/*
impl<T> Clone for LinkedList<T>
where
    T: Clone + std::fmt::Debug,
{
    fn clone(&self) -> Self {
        let mut current = &self.head;
        let mut new_list = LinkedList::new();
        let mut new_current: &mut Option<Box<Node<T>>> = &mut new_list.head;
        loop {
            match current {
                Some(node) => {
                    /* reverse copy
                    let new_head: Node<T> = Node::new(node.value.clone(), new_list.head.take());
                    new_list.head = Some(Box::new(new_head));
                    */
                    /* in-order copy */
                    let new_tail: Box<Node<T>> = Box::new(Node::new(node.value.clone(), None));
                    match new_current {
                        Some(ref mut n) => n.next = Some(new_tail),
                        None => *new_current = Some(new_tail),
                    }

                    new_list.size += 1;
                    current = &node.next;
                    // new_current.as_mut() => Option<&mut Box<Node<T>>>
                    // .unwrap() => get "&mut Box<Node<T>>"
                    // .next => get Option<Box<Node<T>>>
                    // as_mut(): we can change the Box<Node<T>> inside of Option??
                    // without as_mut(), new_current is consumed and freed??
                    new_current = &mut new_current.as_mut().unwrap().next;
                }
                None => break,
            }
        }
        new_list
    }
}
*/
impl<T> Clone for LinkedList<T>
where
    T: Clone + std::fmt::Debug,
{
    fn clone(&self) -> Self {
        let mut current = &self.head;
        let mut new_list = LinkedList::new();
        let mut point_tail: &mut Option<Box<Node<T>>> = &mut None;
        loop {
            match current {
                Some(node) => {
                    /* reverse copy
                    let new_head: Node<T> = Node::new(node.value.clone(), new_list.head.take());
                    new_list.head = Some(Box::new(new_head));
                    */
                    /* in-order copy */
                    let new_tail: Box<Node<T>> = Box::new(Node::new(node.value.clone(), None));
                    match point_tail {
                        Some(ref mut n) => {
                            n.next = Some(new_tail);
                            point_tail = &mut n.next;
                        }
                        None => {
                            new_list.head = Some(new_tail);
                            point_tail = &mut new_list.head;
                        }
                    }

                    new_list.size += 1;
                    current = &node.next;
                }
                None => break,
            }
        }
        new_list
    }
}

impl<T> Iterator for LinkedList<T>
where
    T: std::fmt::Debug + Clone,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.head.take();
        match current {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            Some(node) => {
                // YOU FILL THIS IN!
                self.current = &node.next;
                Some(&node.value)
            }
            None => None, // YOU FILL THIS IN!
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T>
where
    T: std::fmt::Debug + Clone,
{
    type Item = &'a T;
    // Iterator of LinkedListIter must be implemented first
    // then into_iter works.
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            current: &self.head,
        }
    }
}

impl<T> fmt::Display for LinkedList<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {:?}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        write!(f, "size:({}) {}", self.size, result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let mut lista: LinkedList<u32> = LinkedList::new();
        let mut listb: LinkedList<u32> = LinkedList::new();
        for i in 0..10 {
            lista.push_front(i);
            listb.push_front(i);
        }
        assert_eq!(lista, listb);
    }

    #[test]
    fn test_clone() {
        let mut lista: LinkedList<u32> = LinkedList::new();
        for i in 0..10 {
            lista.push_front(i);
        }
        let listb = lista.clone();
        assert_eq!(lista, listb);
    }

    #[test]
    fn test_iter() {
        let mut lista: LinkedList<u32> = LinkedList::new();
        for i in 1..10 {
            lista.push_front(i);
        }
        let mut count = 9;
        for v in lista {
            assert_eq!(count, v);
            count -= 1;
        }
    }
}
