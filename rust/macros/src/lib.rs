#[macro_export]
macro_rules! hashmap {
    ($( $key:expr => $value:expr),*) => {
        {
            let mut temp_hashmap = crate::HashMap::new();
            $(
                temp_hashmap.insert($key, $value);
            )*
            temp_hashmap
        }
    };
    ($( $key:expr => $value:expr,)*) => {
        crate::hashmap!($($key => $value),*)
    };
}
