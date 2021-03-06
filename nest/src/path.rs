use std::convert::Into;
use std::fmt;
use std::path;

#[derive(Debug, Clone)]
pub struct Path(Vec<String>);

impl Path {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> &String {
        &self.0[0]
    }

    pub fn rest(&self) -> Self {
        self.skip(1)
    }

    pub fn take(&self, num: usize) -> Self {
        Path(self.0[0..num].to_vec())
    }

    pub fn skip(&self, num: usize) -> Self {
        Path(self.0[num..self.len()].to_vec())
    }

    pub fn append(&self, item: &str) -> Self {
        let mut vec = Vec::new();
        vec.extend(self.0.iter().cloned());
        vec.push(item.to_owned());
        Path(vec)
    }

    pub fn to_path(&self) -> path::PathBuf {
        path::PathBuf::from(self.0.join(&path::MAIN_SEPARATOR.to_string()))
    }
}

/*

// STALE:
// with help from https://deterministic.space/impl-a-trait-for-str-slices-and-slices-of-strs.html
// but unfortunately can't implement again for String due to "conflicting implementations"
impl<'a, A> From<A> for Path<'a>
where
    A: AsRef<[&'a str]>,
{
    fn from(path: A) -> Path<'a> {
        Path(path.as_ref().to_vec())
    }
}

*/

// macro for implementing n-element array functions and operations
// copied from array source code for AsRef: https://doc.rust-lang.org/src/core/array.rs.html
macro_rules! store_path_from_array_impls {
    ($($N:expr)+) => {
        $(
            impl<A> From<&[A; $N]> for Path
            where
                A: Into<String> + Clone
            {
                fn from(path: &[A; $N]) -> Path {
                    let path: Vec<String> = path.into_iter().cloned().map(Into::into).collect();
                    Path(path)
                }
            }
        )+
    }
}

store_path_from_array_impls! {
    0 1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

impl<A> From<&Vec<A>> for Path
where
    A: Into<String> + Clone,
{
    fn from(path: &Vec<A>) -> Path {
        let path: Vec<String> = path.iter().cloned().map(Into::into).collect();
        Path(path)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_path().display())
    }
}
