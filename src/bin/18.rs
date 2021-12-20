use color_eyre::eyre::Result;
use itertools::Itertools;
use reformation::Reformation;

#[derive(Debug, Clone)]
enum Element {
    Number(u8),
    Pair(Box<Snailfish>),
}

impl Element {
    fn explode_pair(&mut self, depth: usize, index: usize) -> (Option<(u8, u8)>, usize) {
        match self {
            Element::Number(_) => (None, index + 1),
            Element::Pair(fish) => {
                let (exploded, explosion_index) = fish.explode_pair(depth + 1, index);
                if depth == 3 {
                    *self = Element::Number(0);
                }
                (exploded, explosion_index)
            }
        }
    }

    fn get_mut(&mut self, n: usize) -> (Option<&mut u8>, usize) {
        match self {
            Element::Number(v) if n == 0 => (Some(v), 1),
            Element::Number(_) => (None, 1),
            Element::Pair(fish) => fish.get_mut(n),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Number(v) if *v >= 10 => {
                let left = *v / 2;
                let right = *v - left;
                let fish = Snailfish {
                    left: Element::Number(left),
                    right: Element::Number(right),
                };
                *self = Element::Pair(Box::new(fish));
                true
            }
            Element::Number(_) => false,
            Element::Pair(fish) => fish.split(),
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Element::Number(n) => usize::from(*n),
            Element::Pair(fish) => fish.magnitude(),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Pair(fish) => write!(f, "{}", fish),
        }
    }
}

#[derive(Debug, Clone)]
struct Snailfish {
    left: Element,
    right: Element,
}

// [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]
//       _________
//      /         \
//     /\         /\
//    3 /\       6 /\
//     2 /\       5 /\
//      1 /\       4 /\
//       7  3       3  2
//
// [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
//       _________
//      /         \
//     /\         /\
//    3 /\       9 /\
//     2 /\       5 /\
//      8  0       4 /\
//                  3  2

impl Snailfish {
    fn reduce(mut self) -> Self {
        loop {
            // Apply the first explosion, if any.
            let (exploded, at_index) = self.explode_pair(0, 0);
            if let Some((left, right)) = exploded {
                if at_index > 0 {
                    self.get_mut(at_index - 1).0.and_then(|v| Some(*v += left));
                }
                self.get_mut(at_index + 1).0.and_then(|v| Some(*v += right));
                continue;
            }

            // Then apply any splits, if there is on.
            if self.split() {
                continue;
            }

            // If we haven't hit any reduction condition by now, we're done.
            break;
        }

        self
    }

    fn explode_pair(&mut self, depth: usize, index: usize) -> (Option<(u8, u8)>, usize) {
        if depth == 4 {
            return (
                Some((self.left.magnitude() as u8, self.right.magnitude() as u8)),
                index,
            );
        }

        let (result, index_after_left) = self.left.explode_pair(depth, index);
        if result.is_some() {
            return (result, index_after_left);
        }
        self.right.explode_pair(depth, index_after_left)
    }

    // Accesses the value of a node given an in-order traversal index.
    fn get_mut(&mut self, n: usize) -> (Option<&mut u8>, usize) {
        let (value, left_visited) = self.left.get_mut(n);
        if value.is_some() {
            return (value, left_visited);
        }
        let (value, right_visited) = self.right.get_mut(n - left_visited);
        (value, left_visited + right_visited)
    }

    fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }

    fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

impl std::iter::Sum for Snailfish {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut fish = iter.next().unwrap();
        while let Some(right) = iter.next() {
            fish = Snailfish {
                left: Element::Pair(Box::new(fish)),
                right: Element::Pair(Box::new(right)),
            }
            .reduce();
        }
        fish
    }
}

impl std::fmt::Display for Snailfish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

fn expect(s: &[u8], e: u8) -> &[u8] {
    assert_eq!(s[0], e);
    &s[1..]
}

fn parse_element(s: &[u8]) -> (Element, &[u8]) {
    match s[0] {
        b'[' => {
            let (fish, tail) = parse_snailfish(s);
            (Element::Pair(Box::new(fish)), tail)
        }
        _ => {
            let end = s.iter().position(|&b| b == b',' || b == b']').unwrap();
            let number = u8::parse(std::str::from_utf8(&s[..end]).unwrap()).unwrap();
            (Element::Number(number), &s[end..])
        }
    }
}

fn parse_snailfish(mut s: &[u8]) -> (Snailfish, &[u8]) {
    s = expect(s, b'[');

    let (left, tail) = parse_element(s);
    s = tail;

    s = expect(s, b',');

    let (right, tail) = parse_element(s);
    s = tail;

    s = expect(s, b']');

    (Snailfish { left, right }, s)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;

    // Part One
    let answer = input
        .split("\n")
        .map(|s| s.as_bytes())
        .map(|s| parse_snailfish(s).0)
        .sum::<Snailfish>()
        .magnitude();
    dbg!(answer);

    // Part Two
    let answer = input
        .split("\n")
        .map(|s| s.as_bytes())
        .map(|s| parse_snailfish(s).0)
        .combinations(2)
        .map(|c| {
            let forward = c.iter().cloned().sum::<Snailfish>().magnitude();
            let backward = c.iter().rev().cloned().sum::<Snailfish>().magnitude();
            forward.max(backward)
        })
        .max();
    dbg!(answer);

    Ok(())
}

#[allow(dead_code)]
const EXAMPLE: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

const INPUT: &'static str = "[3,[5,[7,[3,9]]]]
[[[[7,0],0],[2,[2,8]]],[[[7,8],1],3]]
[[[[2,7],0],7],4]
[[2,1],[9,0]]
[[[[7,1],[3,2]],[[9,8],5]],[2,7]]
[[[8,9],[[8,7],0]],[[[8,7],[6,3]],[[1,7],[8,9]]]]
[[8,6],[[9,[1,7]],[6,[3,9]]]]
[[2,[[5,6],6]],[[4,[5,9]],[3,[4,5]]]]
[[[[2,0],[1,1]],[6,6]],[[1,9],[[2,7],[6,8]]]]
[[[4,6],[[6,3],[3,9]]],[[[2,6],[6,1]],[[9,9],[1,5]]]]
[[[4,[3,1]],3],6]
[[0,[[5,2],8]],[1,[9,[4,3]]]]
[[[[8,6],[2,1]],[2,[8,6]]],[[[7,1],[3,9]],0]]
[[[[4,7],[2,7]],[[8,9],2]],[[[2,4],[7,2]],[3,7]]]
[[5,[2,2]],[[1,6],[[9,1],[5,0]]]]
[[5,[[1,2],[6,4]]],[6,8]]
[[[5,[1,7]],7],[7,[8,1]]]
[[1,9],[[0,3],[[6,7],[2,4]]]]
[1,[7,[[0,6],0]]]
[[[[5,7],9],[[3,2],7]],[[5,1],[9,9]]]
[[[[0,4],[9,6]],[[8,3],[7,4]]],[7,[6,2]]]
[[[[1,6],0],[[8,0],[3,4]]],[[3,[0,3]],4]]
[4,[[7,8],[4,[9,7]]]]
[[[2,[3,7]],5],[0,[9,9]]]
[[[2,0],[[5,8],[7,6]]],[[9,[6,2]],[3,2]]]
[[[3,1],3],[[[3,7],6],[9,8]]]
[[7,[[2,5],5]],[5,[3,[4,5]]]]
[[[6,7],6],[2,[[9,3],9]]]
[[[[5,6],7],[[3,2],5]],[[9,[4,3]],[3,8]]]
[0,7]
[[[4,6],[2,9]],[[[7,6],[5,1]],7]]
[[0,5],[[1,[4,1]],[[7,3],9]]]
[[[2,[3,8]],5],[[[5,9],8],[7,0]]]
[[[6,[8,6]],[[3,6],7]],[[2,1],[6,[7,5]]]]
[[2,[[6,3],[8,9]]],[[[5,6],4],[[7,0],1]]]
[[[[7,1],[5,6]],8],[[[8,9],4],[8,3]]]
[[[9,2],[1,0]],0]
[[5,[5,[8,5]]],4]
[[3,[5,[4,9]]],3]
[[8,[[7,7],6]],5]
[[4,[[5,1],1]],[1,[1,[9,8]]]]
[[[7,[3,6]],[[2,8],[4,7]]],[[[8,8],[4,0]],[2,4]]]
[[[[3,6],3],[0,9]],2]
[[2,8],[[8,[8,6]],[[1,1],[4,5]]]]
[[2,[1,[1,0]]],[[[6,2],[7,4]],[[7,1],6]]]
[3,[8,[7,[8,6]]]]
[[1,0],[[[0,4],[0,5]],[1,5]]]
[[[[5,0],4],[[7,8],[8,8]]],[[1,7],0]]
[1,[[[4,1],7],[6,[9,0]]]]
[[[1,8],2],[[5,5],[8,5]]]
[[4,[9,[0,6]]],[[[8,9],[4,5]],4]]
[[[[5,4],[1,7]],[[3,1],[7,9]]],[[[0,8],[4,7]],[[5,9],6]]]
[[[[8,0],9],4],[[7,[1,3]],5]]
[[[[5,0],6],[[6,1],8]],[[9,1],7]]
[[9,[6,[8,8]]],[7,[[7,1],6]]]
[[[5,[1,5]],[3,[4,2]]],[[[5,2],7],[[6,9],[2,8]]]]
[[[5,[5,5]],[5,7]],[4,[[2,9],7]]]
[[[[0,4],0],[[0,6],[3,0]]],[0,[[8,1],2]]]
[[[7,[4,6]],[[7,2],[4,6]]],[[[9,3],[4,9]],6]]
[[6,7],7]
[[[4,1],[8,[1,5]]],[[4,6],0]]
[[[4,[5,5]],5],[[0,[2,7]],[1,1]]]
[[[[0,1],3],[6,7]],[4,7]]
[[4,[6,4]],[[[9,8],1],[9,3]]]
[[[4,9],0],[[[7,0],[0,9]],[1,[1,0]]]]
[[[7,9],[[9,5],[6,9]]],[[0,[3,0]],[0,[5,9]]]]
[9,[[0,0],[[1,9],9]]]
[[[5,[0,5]],[[9,8],[9,5]]],[[0,[2,5]],7]]
[[[[5,8],6],9],[[[2,7],7],[[7,8],5]]]
[[8,[[4,7],6]],2]
[[[[7,1],[9,0]],[9,[1,7]]],[[8,[6,7]],[2,5]]]
[[4,[2,9]],8]
[[[[7,6],[5,3]],[5,[9,7]]],[[6,[8,1]],[[6,4],9]]]
[[7,[[7,8],4]],[[1,3],[4,[9,7]]]]
[[[6,[6,7]],[[2,8],3]],[7,[6,[0,3]]]]
[[9,8],[[0,[4,8]],[[9,1],1]]]
[[[[4,0],[5,9]],7],[6,[[5,9],[9,6]]]]
[[8,1],[1,[9,[8,3]]]]
[[[1,[5,1]],[6,7]],[[5,9],[2,[6,7]]]]
[[[3,7],[[7,8],1]],[[0,[6,3]],[8,0]]]
[[5,[[9,3],[1,2]]],7]
[[[1,[9,9]],3],[[6,4],[4,1]]]
[[6,[1,[3,6]]],[2,9]]
[[2,[0,2]],[5,[[9,4],[5,0]]]]
[[4,[[3,1],[7,0]]],[[9,1],[[5,5],[6,7]]]]
[[3,[[7,1],[3,4]]],[7,[9,[9,4]]]]
[[9,9],[[5,4],[[9,7],4]]]
[[[5,1],8],[[6,7],9]]
[[[0,[9,5]],[4,3]],[3,2]]
[[[6,[4,1]],[[8,7],[5,3]]],[[[1,2],5],[[9,2],5]]]
[[[[7,4],[9,0]],[[1,8],[2,9]]],[[5,[1,9]],[4,0]]]
[[[4,[3,8]],[[3,3],[2,8]]],[[[1,3],9],[[8,5],6]]]
[[[[6,4],[7,9]],[[7,6],8]],[7,[9,8]]]
[[7,[3,5]],7]
[[[[5,0],[2,3]],[3,7]],[[4,[6,3]],[7,[4,4]]]]
[[6,[3,[7,6]]],[[[5,8],[8,1]],[3,[1,5]]]]
[[8,[9,[5,2]]],2]
[[1,[5,4]],[[7,[8,0]],8]]
[[[[2,7],4],3],[[1,4],[8,4]]]
[3,[9,2]]";
