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

pub fn average_positive(values: &[i64]) -> f64 {
    let mut sum = 0;
    let mut count = 0i64;
    for val in values {
        if *val > 0 {
            sum += val;
            count += 1;
        }
    }
    sum as f64 / count as f64
}
