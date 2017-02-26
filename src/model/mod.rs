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

mod color;
mod data;
mod image;
mod map;
mod property;
mod reader;
mod shape;
mod tileset;

pub use self::map::Map;
pub use self::tileset::Tileset;

#[cfg(test)]
mod tests;

