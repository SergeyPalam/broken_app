pub mod algo;
pub mod concurrency;

pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter_map(|item|{
        if *item & 1 == 0 {
            Some(*item)
        }else{
            None
        }
    }).sum()
}

pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|item| **item != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let sum: i64 = values.iter().sum();
    if values.is_empty() {
        return 0.0;
    }
    sum as f64 / values.len() as f64
}
