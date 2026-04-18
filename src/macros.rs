#[macro_export]
macro_rules! variant_classes {
    ($name: ident, $default_value: ident, $default_class: literal, $($value: ident, $class: literal), +) => {
        #[derive(Clone, Copy, PartialEq, Default, Debug)]
        pub enum $name {
            #[default]
            $default_value,
            $($value,)+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl $name {
            pub fn class(&self) -> &str {
                match self {
                    $name::$default_value => $default_class,
                    $($name::$value => $class,)+
                }
            }
        }
    }
}
