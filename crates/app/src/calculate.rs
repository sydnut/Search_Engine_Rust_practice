use search_core::{TF, TFIndex};
/// return the term-frequency of one word in this doc
/// > count(term)/SUM(count(t))
pub fn tf(term: &str, doc: &TF) -> f32 {
    let sum_term_count: usize = doc.iter().map(|(_, f)| *f).sum();
    let term_count = *doc.get(term).unwrap_or(&0);
    term_count as f32 / sum_term_count as f32
}
/// return the inverse doc frequency
/// ```Markdown
/// idx=|log(n/max(count,1))|
/// ```
/// n means the total number of docs,count means the count of the docs
/// where the term occurs in
pub fn idf(term: &str, tf_index: &TFIndex) -> f32 {
    let n: usize = tf_index.len();
    let count = tf_index
        .values()
        .filter(|tf| tf.contains_key(term))
        .count()
        .max(1);
    f32::log10(n as f32 / count as f32).abs()
}
