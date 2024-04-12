---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: data-structure/weighted-union-find/src/lib.rs
    title: Weighted Union Find
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Zero {\n    fn zero() -> Self;\n    fn is_zero(&self) -> bool;\n\
    }\n\npub trait One {\n    fn one() -> Self;\n    fn is_one(&self) -> bool;\n}\n\
    \nmacro_rules! impl_zero {\n    ($($type:ty), *) => {\n        $(\n          \
    \  impl Zero for $type {\n                fn zero() -> Self {\n              \
    \      0\n                }\n                fn is_zero(&self) -> bool {\n   \
    \                 *self == 0\n                }\n            }\n        )*\n \
    \   };\n}\n\nmacro_rules! impl_one {\n    ($($type:ty), *) => {\n        $(\n\
    \            impl One for $type {\n                fn one() -> Self {\n      \
    \              1\n                }\n                fn is_one(&self) -> bool\
    \ {\n                    *self == 1\n                }\n            }\n      \
    \  )*\n    };\n}\n\nimpl_zero!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);\n\
    \nimpl_one!(i8, u8, i16, u16, i32, u32, u64, i64, isize, usize);\n"
  dependsOn: []
  isVerificationFile: false
  path: algebra/internal-type/src/lib.rs
  requiredBy:
  - data-structure/weighted-union-find/src/lib.rs
  timestamp: '2024-04-12 20:06:17+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: algebra/internal-type/src/lib.rs
layout: document
redirect_from:
- /library/algebra/internal-type/src/lib.rs
- /library/algebra/internal-type/src/lib.rs.html
title: algebra/internal-type/src/lib.rs
---