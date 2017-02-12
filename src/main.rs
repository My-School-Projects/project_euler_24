
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
/// Given a sequence S, this function finds the first element e[0]
/// of the sequence P(S), which is the pth lexicographic permutation of S.
///
/// e[0] is given by  p mod n! / (n - 1)!, where p is the permutation,
/// and n is the size of the sequence.
///
/// This works because the permutations are listed in lexical order,
/// and so is the sequence S. As p increases from 0 to n, the first element e[0]
/// starts at S[0] (the first element of S), and repeats (n-1)! times before
/// becoming S[1], and repeating (n-1)! again, and so on. In this way, we can
/// identify groupings where e[0] is the same value for certain ranges of p.
/// Because there are (n-1)! permutations in each group, for any valid p,
/// e[0] can be found by taking  p / (n - 1)!.
///
/// Unfortunately there's a slight problem. Because this function will be called on
/// ever decreasing sizes of n, p will eventually be too large to be considered
/// a permutation of S. To fix this, we need to make the "permutation space" wrap around.
/// In other words, if there are only 24 possible permutations of S, and p = 28, we need
/// p to refer to the 4th permutation of S. To accomplish that we must take p mod 24, or n!.
/// So our actual formula for the first element of S, e[0] is  p mod n! / (n-1)!.
///
/// Note: we actually use p-1, not p. This is just because I defined the permutations to
/// start at 0, not 1, so I have to adjust it to make the value come out as expected.
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
/// RemoveFirst is provided as a succinct way
/// to remove the first occurrence of an element from a vec.
///
/// If you're not familiar with Rust syntax, basically this is just
/// an extension to Vec.
/// First you declare a trait, which can be applied to any class
/// if you're willing to take the time to write the class-specific implementation.
///
trait RemoveFirst<E> {
    fn remove_first(&mut self, e: E);
}

///
/// This is the class specific implementation for the trait RemoveFirst.
/// It only applies to Vec's which hold types that implement the PartialEq trait,
/// and the Copy trait. This is because the elements need to be comparable, and
/// they need to be copyable in order to use the iter().position() method.
///
impl <E: PartialEq<E> + Copy> RemoveFirst<E> for Vec<E>
{
    fn remove_first(&mut self, e: E)
    {
        if let Some(p) = self.iter().position(|&n| n == e) {
            &self.remove(p);
        }
    }
}
