use std::marker::PhantomData;

pub trait Combine {
    type Item;
    fn combine(&mut self, )
}

pub struct Combinations<T> {
    _mark: PhantomData<T>,
}

impl<T> Combinations<T> {
    pub fn count<I, L>(items: I) -> usize
    where
        L: AsRef<[T]>,
        I: Iterator<Item = L>,
    {
        items.fold(1, |acc, l| acc * l.as_ref().len().max(1))
    }

    pub fn combine_all<I, V, F, U>(data: U, mut items: I, f: F) -> Vec<U>
    where
        U: Clone,
        V: AsRef<[T]>,
        I: Clone + Iterator<Item = V>,
        F: Fn(&mut U, &T) + Copy,
    {
        if let Some(vars) = items.next() {
            let list = Combinations::combine(data, vars, f);
            let mut res = vec![];
            for n in list.into_iter() {
                res.extend(Combinations::combine_all(n, items.clone(), f));
            }
            res
        } else {
            vec![data]
        }
    }

    pub fn combine<V, F, U>(data: U, alts: V, f: F) -> Vec<U>
    where
        U: Clone,
        V: AsRef<[T]>,
        F: Fn(&mut U, &T),
    {
        let mut res = vec![];
        for v in alts.as_ref() {
            let i = res.len();
            res.push(data.clone());
            f(&mut res[i], v);
        }
        if res.is_empty() {
            res.push(data);
        }
        res
    }

    pub fn combine_once<V, F, U>(variants: &mut Vec<U>, val: V, f: F)
    where
        V: AsRef<T>,
        F: Fn(&mut U, &T),
    {
        for v in variants.iter_mut() {
            f(v, val.as_ref());
        }
    }

    pub fn combine_inplace<A, F, U>(variants: &mut Vec<U>, mut alts: A, f: F)
    where
        U: Clone,
        A: Iterator<Item = T>,
        T: Copy,
        F: Fn(&mut U, T),
    {
        if let Some(first) = alts.next() {
            let l = variants.len();
            for alt in alts {
                for j in 0..l {
                    let mut var = variants[j].clone();
                    f(&mut var, alt);
                    variants.push(var);
                }
            }
            for j in 0..l {
                f(&mut variants[j], first);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Combinations;

    #[test]
    fn makes_all_combinations() {
        let data = vec![vec!['a', 'b', 'c'], vec!['a', 'b'], vec![], vec!['a', 'd']];
        let count = Combinations::count(data.iter());
        let mut res = Combinations::combine_all(String::new(), data.into_iter(), |s, c| {
            s.push(*c);
        });
        assert_eq!(res.len(), count);
        res.sort();
        assert_eq!(
            res,
            vec![
                "aaa", "aad", "aba", "abd", "baa", "bad", "bba", "bbd", "caa", "cad", "cba", "cbd"
            ]
        )
    }

    #[test]
    fn modify_with_combine_inplace() {
        let mut data = vec!["a".to_owned()];
        Combinations::combine_inplace(&mut data, ['a', 'b'].iter(), |s, ch| s.push(*ch));
        assert_eq!(data.len(), 2);
        assert_eq!(data, vec!["aa".to_owned(), "ab".to_owned()]);
    }

    #[test]
    fn init_with_combine_inplace() {
        let mut data = vec![String::new()];
        Combinations::combine_inplace(&mut data, ['a', 'b'].iter(), |s, ch| s.push(*ch));
        Combinations::combine_inplace(&mut data, ['a'].iter(), |s, ch| s.push(*ch));
        Combinations::combine_inplace(&mut data, ['c'].iter(), |s, ch| s.push(*ch));
        assert_eq!(data.len(), 2);
        assert_eq!(data, vec!["aac".to_owned(), "bac".to_owned()]);
    }

    #[test]
    fn counts_all_combinations() {
        use rand::{thread_rng, Rng};
        let mut r = thread_rng();
        let len: usize = r.gen_range(0..5);
        let mut data = vec![];
        let mut expected = 1; // no combinations is an acceptable variant
        for _ in 0..len {
            let n: usize = r.gen_range(0..5);
            let mut list = vec![];
            expected *= n.max(1);
            for _ in 0..n {
                list.push(r.gen::<u32>())
            }
            data.push(list);
        }
        let count = Combinations::count(data.iter());
        println!("Combinations {} {} {:?}", count, len, data);
        assert_eq!(expected, count);
    }
}
