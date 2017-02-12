
fn main()
{
    let p = 13;
    let s = &vec![1, 2, 3, 4];
    println!("The {}th permutation of the sequence {:?} is: {:?}", p, s, permutation(p, s));
    let p = 1000000;
    let s = &vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("The {}th permutation of the sequence {:?} is: {:?}", p, s, permutation(p, s));
}

///
/// Given a sequence S, this function finds the sequence P(S), which is
/// the pth lexicographic permutation of S.
///
/// Finding the first element of P(s) is trivial, and is handled by
/// the first_element_of_permutation() function.
/// Once the first element e[0] of P(S) is found, the next element, e[1]
/// is the first element of P(S - e).
/// Note: "S - e" means "the sequence S, minus the element e".
///
fn permutation(p: u64, s: &Vec<u64>) -> Vec<u64>
{
    if s.len() == 1 {
        s.to_vec()
    } else {
        // rebind s to be mutable
        let mut s = s.clone();
        let mut v : Vec<u64> = Vec::with_capacity(s.len() + 1);
        // find the first element of the permutation and add it to v
        let e = first_element_of_permutation(p, &s);
        v.push(e);
        // remove e from s
        s.remove_first(e);
        // find the rest of the permutation (recursively) and append it to v
        v.append(&mut permutation(p, &s));
        v
    }
}

///
/// Given a sequence S, this function finds the first element
/// of the sequence P(S), which is the pth lexicographic permutation of S.
///
fn first_element_of_permutation(p: u64, s: &Vec<u64>) -> u64
{
    let n = s.len() as u64;
    
    let i = ((p-1) % factorial(n) / factorial(n-1)) as usize;
    
    s[i]
}

fn factorial(n: u64) -> u64
{
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n-1)
    }
}

///
/// RemoveFirst is provided simply as a succinct way
/// to remove the first occurrence of an element from a vec.
///
trait RemoveFirst<E> {
    fn remove_first(&mut self, e: E);
}

impl <E: PartialEq<E> + Copy> RemoveFirst<E> for Vec<E>
{
    fn remove_first(&mut self, e: E)
    {
        if let Some(p) = self.iter().position(|&n| n == e) {
            &self.remove(p);
        }
    }
}
