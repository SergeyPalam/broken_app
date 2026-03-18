//! # Библиотека алгоритмов
#![warn(missing_docs)]

/// Модуль алгоритмов
pub mod algo;

/// Модуль Thread safe структур
pub mod concurrency;

/// Сумма четных чисел последовательности
/// (Исправлено UB при обращении за границу массива)
/// Некоторая оптимизация проверки на четность
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter_map(|item|{
        if *item & 1 == 0 {
            Some(*item)
        }else{
            None
        }
    }).sum()
}

/// Подсчёт количества ненулевых элементов
/// Устранена утечка памяти
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|item| **item != 0).count()
}

/// Нормализация строки
/// Исправлена ошибка нормализации
/// Учитываются все пустые символы
pub fn normalize(input: &str) -> String {
    let mut res = String::new();

    for symb in input.chars(){
        if symb.is_whitespace() {
            continue;
        }

        let mut append = String::new();
        for inner in symb.to_lowercase() {
            append.push(inner);
        }
        res.push_str(&append);
    }
    res
}

/// Подсчёт арифметического среднего 
/// Исправлена логическая ошибка подсчёта среднего всех положительных элементов
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
