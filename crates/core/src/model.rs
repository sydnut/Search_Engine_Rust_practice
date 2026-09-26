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
/// 批处理操作
pub type Bulk<T> = Vec<T>;
/// 对Model的单次操作
pub enum Update {
    //df/tf term + cnt 统一在执行后如果为0进行删除
    Modify {
        t: Modified,
        term: String,
        cnt: isize,
    },
    //remove path
    Remove(std::path::PathBuf),
    Set(std::path::PathBuf, std::time::SystemTime),
}
pub enum Modified {
    DF,
    TF(std::path::PathBuf),
}
impl Model {
    pub fn new(df: DF, index: Index) -> Self {
        Self { df, index }
    }
    pub fn index(&self) -> &Index {
        &self.index
    }
    pub fn df(&self) -> &DF {
        &self.df
    }
    pub fn index_mut(&mut self) -> &mut Index {
        &mut self.index
    }
    pub fn df_mut(&mut self) -> &mut DF {
        &mut self.df
    }
    pub fn is_empty(&self) -> bool {
        self.df().is_empty() && self.index().is_empty()
    }
    pub fn apply(&mut self, bulk: Bulk<Update>) {
        bulk.into_iter()
            .for_each(|cmd|cmd.exec(self));
    }
}
impl Update {
    pub fn modify(t: Modified, term: String, cnt: isize) -> Self {
        Self::Modify { t, term, cnt }
    }
    pub fn remove(path: std::path::PathBuf) -> Self {
        Self::Remove(path)
    }
    pub fn set(path: std::path::PathBuf, ts: std::time::SystemTime) -> Self {
        Self::Set(path, ts)
    }
    pub(self) fn exec(self, model: &mut Model) {
        match self {
            Self::Modify { t, term, cnt } => {
                let map = match t {
                    Modified::DF => model.df_mut(),
                    Modified::TF(path) => model.index_mut().get_mut(&path).unwrap().get_tf_mut(),
                };
                let mut need_to_remove = false;
                map.entry(term.clone()).and_modify(|old_cnt| {
                    *old_cnt = (*old_cnt as isize + cnt) as usize;
                    need_to_remove = if *old_cnt == 0 { true } else { false }
                });
                //如果结果为0，删掉
                if need_to_remove{
                    map.remove(&term);
                }
            }
            Self::Remove(path) => {
                model.index_mut().remove(&path);
            }
            Self::Set(path, ts) => {
                model
                    .index_mut()
                    .entry(path)
                    .and_modify(|doc| doc.set_ts(ts));
            }
        }
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
    /// calculate the diff set of two doc and convert it into the Bulk
    pub(crate) fn diff(&self, old: &Self, path: std::path::PathBuf) -> Bulk<Update> {
        let mut bulk = Bulk::new();
        //update: set sys_time
        bulk.push(Update::set(path.clone(), self.ts));
        let new = self.get_tf();
        let old = old.get_tf();
        //1 公共的词数量变化
        for (term, cnt) in new {
            if let Some(old_cnt) = old.get(term) {
                //update old[term]+=cnt-old_cnt
                bulk.push(Update::modify(
                    Modified::TF(path.clone()),
                    term.clone(),
                    (cnt - old_cnt) as isize,
                ));
            } else {
                //2 新增词的数量变化
                //update: df[term]+=1
                bulk.push(Update::modify(Modified::DF, term.clone(), 1));
                //update:old[term]+=cnt
                bulk.push(Update::modify(
                    Modified::TF(path.clone()),
                    term.clone(),
                    *cnt as isize,
                ));
            }
        }
        //3 删除词的数量变化
        for (remove_term, cnt) in old {
            if new.get(remove_term).is_none() {
                //update: df[remove_term]-=1
                bulk.push(Update::modify(Modified::DF, remove_term.clone(), -1));
                //update: old[remove_term]-=cnt
                bulk.push(Update::modify(
                    Modified::TF(path.clone()),
                    remove_term.clone(),
                    -(*cnt as isize),
                ));
            }
        }
        bulk
    }
}
