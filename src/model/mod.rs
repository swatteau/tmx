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

pub mod color;
pub mod data;
pub mod image;
pub mod map;
pub mod property;
pub mod reader;
pub mod shape;
pub mod tileset;

pub use self::map::Map;
pub use self::tileset::Tileset;

#[cfg(test)]
mod tests;

