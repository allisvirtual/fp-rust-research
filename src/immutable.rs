//! Immutable data and persistent lists: a custom list type with map, filter and fold.
//! Owner: Bartosz Nicolas Aniola

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn nil() -> Rc<List<T>> {
        Rc::new(List::Nil)
    }

    pub fn cons(head: T, tail: &Rc<List<T>>) -> Rc<List<T>> {
        Rc::new(List::Cons(head, Rc::clone(tail)))
    }

    pub fn map<U>(&self, f: &impl Fn(&T) -> U) -> Rc<List<U>> {
        match self {
            List::Nil => List::nil(),
            List::Cons(x, rest) => Rc::new(List::Cons(f(x), rest.map(f))),
        }
    }

    pub fn filter(&self, p: &impl Fn(&T) -> bool) -> Rc<List<T>>
    where
        T: Clone,
    {
        match self {
            List::Nil => List::nil(),
            List::Cons(x, rest) if p(x) => Rc::new(List::Cons(x.clone(), rest.filter(p))),
            List::Cons(_, rest) => rest.filter(p),
        }
    }

    pub fn foldr<B>(&self, init: B, f: &impl Fn(&T, B) -> B) -> B {
        match self {
            List::Nil => init,
            List::Cons(x, rest) => f(x, rest.foldr(init, f)),
        }
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.foldr(Vec::new(), &|x, acc| [vec![x.clone()], acc].concat())
    }
}

pub fn demo() {
    let shared = List::cons(3, &List::cons(4, &List::cons(5, &List::nil())));
    let a = List::cons(1, &shared);
    let b = List::cons(2, &shared);

    println!("shared       {:?}", shared.to_vec());
    println!("cons 1       {:?}", a.to_vec());
    println!("cons 2       {:?}", b.to_vec());
    println!("shared again {:?}", shared.to_vec());
    println!("Rc count     {}", Rc::strong_count(&shared));
    println!("map (* 2)    {:?}", shared.map(&|x| x * 2).to_vec());
    println!("filter even  {:?}", shared.filter(&|x| x % 2 == 0).to_vec());
    println!("foldr (+) 0  {}", shared.foldr(0, &|x, acc| x + acc));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Rc<List<i32>> {
        List::cons(1, &List::cons(2, &List::cons(3, &List::nil())))
    }

    #[test]
    fn cons_shares_the_tail_and_leaves_the_original_alone() {
        let shared = sample();
        let extended = List::cons(0, &shared);

        assert_eq!(extended.to_vec(), vec![0, 1, 2, 3]);
        assert_eq!(shared.to_vec(), vec![1, 2, 3]);
        assert_eq!(Rc::strong_count(&shared), 2);
    }

    #[test]
    fn map_filter_and_fold() {
        assert_eq!(sample().map(&|x| x * 10).to_vec(), vec![10, 20, 30]);
        assert_eq!(sample().filter(&|x| x % 2 == 1).to_vec(), vec![1, 3]);
        assert_eq!(sample().foldr(0, &|x, acc| x + acc), 6);
    }
}
