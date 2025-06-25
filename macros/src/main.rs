#[macro_export]
macro_rules! my_macro {
    ($foo:ident, $bar:ident, $baz:ident) => {
        ($foo(), $bar(), $baz())
    };
}

// Реализация функций
fn foo() -> i32 {
    42
}
fn bar() -> &'static str {
    "Hello"
}
fn baz() -> bool {
    true
}

// Основная программа
fn main() {
    let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);

    println!("Результаты:");
    println!("foo: {}", foo_result);
    println!("bar: {}", bar_result);
    println!("baz: {}", baz_result);
}

// Тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_output() {
        let (foo_result, bar_result, baz_result) = my_macro!(foo, bar, baz);

        assert_eq!(foo_result, 42);
        assert_eq!(bar_result, "Hello");
        assert_eq!(baz_result, true);
    }

    #[test]
    fn test_function_types() {
        assert_eq!(foo(), 42);
        assert_eq!(bar(), "Hello");
        assert_eq!(baz(), true);
    }
}
