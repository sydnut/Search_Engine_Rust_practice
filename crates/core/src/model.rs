use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// record one file's `token->cnt`
pub type TF = HashMap<String, usize>;
pub type Index = HashMap<PathBuf, Doc>;
/// the token count of occurrence of all files
pub type DF = HashMap<String, usize>;
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Model {
    df: DF,
    index: Index,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    tf: TF,
    ts: std::time::SystemTime,
}
impl Model {
    pub fn new(df: DF, index: Index) -> Self {
        Self { df, index }
    }
    pub fn tf_index(&self) -> &Index {
        &self.index
    }
    pub fn df(&self) -> &DF {
        &self.df
    }
    pub fn tf_index_mut(&mut self) -> &mut Index {
        &mut self.index
    }
    pub fn df_mut(&mut self) -> &mut DF {
        &mut self.df
    }
}
impl Doc {
    pub fn new(tf: TF, ts: std::time::SystemTime) -> Self {
        Self { tf, ts }
    }
    pub fn get_tf(&self) -> &TF {
        &self.tf
    }
    pub fn get_ts(&self) -> std::time::SystemTime {
        self.ts
    }
    pub fn get_tf_mut(&mut self) -> &mut TF {
        &mut self.tf
    }
    pub fn set_ts(&mut self, ts: std::time::SystemTime) {
        self.ts = ts;
    }
}
