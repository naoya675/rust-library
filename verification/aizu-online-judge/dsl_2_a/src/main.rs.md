---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/segment-tree/src/lib.rs
    title: data-structure/segment-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A\n\
    \nuse proconio::input;\nuse std::cmp::min;\n\nuse segment_tree::SegmentTree;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n        com:\
    \ [(usize, usize, usize); q],\n    }\n    let mut st = SegmentTree::new(n, |a,\
    \ b| min(a, b), (1 << 31) - 1);\n    for (com, x, y) in com {\n        match com\
    \ {\n            0 => st.set(x, y),\n            1 => {\n                println!(\"\
    {}\", st.prod(x, y + 1));\n            }\n            _ => unreachable!(),\n \
    \       }\n    }\n}\n"
  dependsOn:
  - data-structure/segment-tree/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/dsl_2_a/src/main.rs
  requiredBy: []
  timestamp: '2024-03-11 21:49:40+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/dsl_2_a/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/dsl_2_a/src/main.rs
- /verify/verification/aizu-online-judge/dsl_2_a/src/main.rs.html
title: verification/aizu-online-judge/dsl_2_a/src/main.rs
---