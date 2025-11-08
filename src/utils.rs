use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::parser::Course;

pub fn is_valid_path(path_str: &str) -> bool {
    if path_str.contains('\0') {
        return false;
    }

    let path = Path::new(path_str);

    let normalized: PathBuf = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => return false,
        }
    };

    if normalized.as_os_str().is_empty() {
        return false;
    }

    true
}

pub trait CourseFilter {
    fn deduplicate(&self) -> Vec<Course>;
    fn find_by_ids(&self, ids: &[i32]) -> Option<Vec<Course>>;
}

impl CourseFilter for Vec<Course> {
    fn deduplicate(&self) -> Vec<Course> {
        let mut seen = HashSet::new();
        let mut unique = Vec::new();

        for c in self {
            let key = (c.title.clone(), c.code.clone(), c.course_id);
            if seen.insert(key) {
                unique.push(c.clone());
            }
        }

        unique
    }

    fn find_by_ids(&self, ids: &[i32]) -> Option<Vec<Course>> {
        let id_set: HashSet<i32> = ids.iter().copied().collect();

        let filtered: Vec<Course> = self
            .iter()
            .filter(|c| id_set.contains(&c.course_id))
            .cloned()
            .collect();

        let found_ids: HashSet<i32> = filtered.iter().map(|c| c.course_id).collect();
        if id_set.is_subset(&found_ids) {
            Some(filtered)
        } else {
            None
        }
    }
}
