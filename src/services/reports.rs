use std::{collections::HashSet, hash::Hash, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Default)]
pub struct Report {
    pub saved: usize,
    pub cleaned: usize,
    pub deleted: usize,
    pub skipped: usize,
    pub failed: HashSet<Failure>,
}

#[derive(Debug)]
pub struct Failure {
    pub path: PathBuf,
    pub error: anyhow::Error,
}

#[derive(Clone, Copy)]
pub enum Outcome {
    Saved,
    Cleaned,
    Deleted,
    Skipped,
}

#[derive(Error, Debug)]
pub enum FloordError {
    #[error("Couldn't get file name: {0}")]
    FileNameNotFound(PathBuf),

    #[error("Couldn't get last modification time: {0}")]
    ModificationTimeNotFound(PathBuf),

    #[error("Couldn't remove: {0}")]
    RemoveFailure(PathBuf),
}

impl PartialEq for Failure {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Eq for Failure {}
impl Hash for Failure {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl Report {
    pub(crate) fn record(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Saved => self.saved += 1,
            Outcome::Cleaned => self.cleaned += 1,
            Outcome::Deleted => self.deleted += 1,
            Outcome::Skipped => self.skipped += 1,
        }
    }

    pub(crate) fn fail(&mut self, path: PathBuf, error: anyhow::Error) {
        self.failed.insert(Failure { path, error });
    }

    //pub(crate) fn add(&mut self, report: Report) {
    //    self.saved += report.saved;
    //    self.cleaned += report.cleaned;
    //    self.deleted += report.deleted;
    //    self.skipped += report.skipped;
    //    for fail in report.failed {
    //        self.failed.insert(fail);
    //    }
    //}
}
