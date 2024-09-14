#[allow(dead_code)]
#[derive(Debug)]
struct Node<T>
where
    T: PartialEq
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[allow(dead_code)]
impl<T> Node<T>
where
    T: PartialEq
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
struct ArenaTree<T> 
where
    T: PartialEq
{
    arena: Vec<Node<T>>,
}

#[allow(dead_code)]
impl<T> ArenaTree<T>
where 
    T: PartialEq
{
    fn insert(&mut self, _parent: usize, _child: usize) {
        // self.
    }
}
