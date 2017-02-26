macro_rules! define_iterator_wrapper {
    ($name: ident, $item: ident) => {
        pub struct $name<'a>(::std::slice::Iter<'a, $item>);

        impl<'a> Iterator for $name<'a> {
            type Item = &'a $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }
        }
    }
}

mod map;
mod tileset;
mod data;
mod reader;

pub use self::map::*;
pub use self::tileset::*;
pub use self::data::*;

#[cfg(test)]
mod tests;

