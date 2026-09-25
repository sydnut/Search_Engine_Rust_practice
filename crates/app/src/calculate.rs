use search_core::{Model, TF, lexer::Lexer};
use std::path::PathBuf;
/// return the term-frequency of one word in this doc
/// > count(term)/SUM(count(t))
fn tf(term: &str, doc: &TF, sum_term_count: usize) -> f32 {
    let term_count = *doc.get(term).unwrap_or(&0);
    term_count as f32 / sum_term_count as f32
}
/// return the inverse doc frequency
/// ```Markdown
/// idx=|log(n/max(count,1))|
/// ```
/// n means the total number of docs,count means the count of the docs
/// where the term occurs in
fn idf(term: &str, model: &Model) -> f32 {
    let n: usize = model.tf_index().len();
    let count = (*model.df().get(term).unwrap_or(&0)).max(1);
    f32::log10(n as f32 / count as f32).abs()
}
/// core search function,apply the index to the input data so you can the get the top 10 path
pub fn search(data: &str, model: &Model) -> Vec<(String, f32)> {
    let tf_index = model.tf_index();
    let data = data.chars().collect::<Vec<_>>();
    let tokens: Vec<String> = Lexer::new(&data).collect();
    // record the rank of each doc's tf-idf score
    let mut res: Vec<(&PathBuf, f32)> = Vec::with_capacity(tf_index.len());
    for (path, doc) in tf_index {
        let mut rank = 0f32;
        let tf_table = doc.get_tf();
        let sum_term_count: usize = tf_table.iter().map(|(_, f)| *f).sum();
        for token in &tokens {
            rank += tf(token, tf_table, sum_term_count) * idf(token, model);
        }
        res.push((path, rank));
    }
    res.sort_by(|(_, r1), (_, r2)| r2.partial_cmp(r1).unwrap());
    // Return the paths of the top 10 results as JSON.
    res.iter()
        .take(10)
        .map(|(path, score)| (path.display().to_string().replace("\\", "/"), *score))
        .collect::<Vec<_>>()
}
