use std::{cmp::Ordering, ops::Range, rc::Rc};
use uni_path::{Path, PathBuf};

#[derive(Debug)]
struct IndexEntry {
    name: Rc<PathBuf>,
    start: usize,
    range: Range<usize>,
}

/// Index that maps generated code locations to their origins
pub struct Index {
    segs: Vec<IndexEntry>,
    size: usize,
}

impl Index {
    fn new() -> Self {
        Self {
            segs: Vec::new(),
            size: 0,
        }
    }

    fn push(&mut self, name: Rc<PathBuf>, start: usize, size: usize) {
        self.segs.push(IndexEntry {
            name,
            start,
            range: self.size..(self.size + size),
        });
        self.size += size;
    }

    fn append(&mut self, mut index: Index) {
        for seg in index.segs.iter_mut() {
            let r = &mut seg.range;
            r.start += self.size;
            r.end += self.size;
        }
        self.segs.append(&mut index.segs);
        self.size += index.size;
    }

    /// Maps line number in generated code to source file name and position in it
    pub fn search(&self, pos: usize) -> Option<(PathBuf, usize)> {
        match self.segs.binary_search_by(|seg| {
            if pos < seg.range.start {
                Ordering::Greater
            } else if pos >= seg.range.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(i) => {
                let seg = &self.segs[i];
                Some(((*seg.name).clone(), pos - seg.range.start + seg.start))
            }
            Err(_) => None,
        }
    }
}

/// Tree of parsed source files
pub struct Node {
    name: PathBuf,
    inner: Vec<(Node, usize)>,
    text: String,
    index: Vec<Range<usize>>,
}

impl Node {
    pub(crate) fn new(name: &Path) -> Self {
        Self {
            name: name.to_path_buf(),
            inner: Vec::new(),
            text: String::new(),
            index: Vec::new(),
        }
    }

    pub fn name(&self) -> &Path {
        &self.name
    }

    pub(crate) fn add_line(&mut self, line: &str) {
        let plen = self.text.len();
        self.text.push_str(line.trim_end());
        self.text.push('\n');
        self.index.push(plen..self.text.len());
    }

    pub(crate) fn add_child(&mut self, node: Node) {
        self.add_line("");
        self.inner.push((node, self.index.len() - 1));
    }

    pub fn lines_count(&self) -> usize {
        self.index.len()
    }

    /// Generates resulting code string and mapping index for it
    pub fn collect(&self) -> (String, Index) {
        let mut accum = String::new();
        let mut index = Index::new();

        let name = Rc::new(self.name.clone());

        let mut ppos = 0;
        for (node, pos) in self.inner.iter() {
            let start = self.index[ppos].start;
            let end = self.index[*pos].end;

            accum.push_str(&self.text[start..end]);
            index.push(name.clone(), ppos, pos - ppos + 1);

            let (child_str, child_index) = node.collect();
            accum.push_str(&child_str);
            index.append(child_index);

            ppos = *pos + 1;
        }

        if ppos < self.index.len() {
            accum.push_str(&self.text[self.index[ppos].start..]);
            index.push(name, ppos, self.index.len() - ppos);
        }

        (accum, index)
    }
}
