use super::util;

pub fn solution() {
    let expected: String = util::read_expected_results("15").get(0).unwrap().to_string();
    let (n, lines) = util::read_input_lines_with_n("15");
    let integers: Vec<isize> = lines.iter().map(|n| n.parse().unwrap()).collect();

    let mut head: Link = None;
    for int in integers {
        head = insert(head, int);
    }

    let result = display(&head);
    assert_eq!(expected, result);
}

fn insert(mut head: Link, data: isize) -> Link {
    if head.is_none() {
        Some(Box::new(Node { data, next: None }))
    } else {
        head.as_mut().unwrap().add_to_tail(data);
        head
    }
}

fn display(head: &Link) -> String {
    // for test purposes
    let mut res = "".to_string();

    let mut start: &Link = head;
    while start.is_some() {
        let node = start.as_ref().unwrap();
        print!("{} ", node.data);
        res = format!("{}{} ", res, node.data);
        start = &node.next;
    }

    res.trim().to_string()
}

type Link = Option<Box<Node>>;

struct Node {
    data: isize,
    next: Link
}

impl Node {
    fn from(d: isize) -> Node {
        Node { data: d, next: None }
    }

    fn set_next(mut self, data: isize) {
        self.next = Some(Box::from(Node::from(data)));
    }

    fn add_to_tail(&mut self, data: isize) {
        let mut curr = self;
        loop {
            match &curr.next {
                None => break,
                Some(node) => curr = curr.next.as_mut().unwrap()
            }
        }
        curr.next = Some(Box::from(Node::from(data)));
    }
}