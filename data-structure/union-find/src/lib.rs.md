---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind/src/main.rs
    title: verification/library-checker/unionfind/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone)]\npub struct UnionFind {\n    n: usize,\n    par:\
    \ Vec<usize>,\n    siz: Vec<usize>,\n}\n\nimpl UnionFind {\n    pub fn new(n:\
    \ usize) -> Self {\n        Self {\n            n,\n            par: (0..n).collect::<Vec<usize>>(),\n\
    \            siz: vec![1; n],\n        }\n    }\n\n    pub fn merge(&mut self,\
    \ a: usize, b: usize) -> bool {\n        let a = self.leader(a);\n        let\
    \ b = self.leader(b);\n        if a == b {\n            return false;\n      \
    \  }\n        if self.siz[a] > self.siz[b] {\n            self.par[b] = a;\n \
    \           self.siz[a] += self.siz[b];\n        } else {\n            self.par[a]\
    \ = b;\n            self.siz[b] += self.siz[a];\n        }\n        true\n   \
    \ }\n\n    pub fn same(&mut self, a: usize, b: usize) -> bool {\n        self.leader(a)\
    \ == self.leader(b)\n    }\n\n    pub fn leader(&mut self, a: usize) -> usize\
    \ {\n        if self.par[a] == a {\n            return a;\n        }\n       \
    \ self.par[a] = self.leader(self.par[a]);\n        self.par[a]\n    }\n\n    pub\
    \ fn size(&mut self, a: usize) -> usize {\n        let a = self.leader(a);\n \
    \       self.siz[a]\n    }\n\n    pub fn groups(&mut self) -> Vec<Vec<usize>>\
    \ {\n        let mut res = vec![vec![]; self.n];\n        for i in 0..self.n {\n\
    \            res[self.leader(i)].push(i);\n        }\n        res.into_iter()\n\
    \            .filter(|f| !f.is_empty())\n            .collect::<Vec<_>>()\n  \
    \  }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: data-structure/union-find/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-11 21:49:40+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/unionfind/src/main.rs
documentation_of: data-structure/union-find/src/lib.rs
layout: document
redirect_from:
- /library/data-structure/union-find/src/lib.rs
- /library/data-structure/union-find/src/lib.rs.html
title: data-structure/union-find/src/lib.rs
---