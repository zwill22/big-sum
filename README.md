# Big Sum

[![Rust][rust-badge]][rust]
[![GitHub][github-badge]][repo]
[![GitHub Actions][actions-badge]][repo-actions]
[![CI][rust-test-badge]][rust-test]
[![Coffee][buy-me-coffee]][coffee]
[![License: MIT][license-badge]][license]

A simple Rust program which calculates the sum \
$$S_n = \sum_{a=1}^n \sum_{b>0, c>0; a+b+c=n} \text{lcm} (c, g)$$\
where\
$$g = \gcd( a, b ).$$

The reference implementation for this calculation scales poorly with system size.
However, by use of the identities\
$$gcd(a, b) = \sum_{k|a, k|b} \varphi (k),$$\
where $\varphi (k)$ is Euler's Totient function, and\
$$\text{lcm} (a,b) = \frac{a \cdot b}{\gcd (a, b)}$$

The equation may be reformulated entirely in terms of $\varphi (k)$, such that\
$$S_n = \sum_{g=1}^{(n-1)/2} \frac{g}{\gcd (g, n) } \sum_{k=2}^{(n-1)/g} \left[ n \varphi (k) - g  k \varphi (k) \right].$$\
This can be written in the form:\
$$S_n = \sum_{g=1}^{(n-1)/2} \frac{g}{\gcd (g, n) } \left[ n \Phi \left( \frac{n-1}{g} \right) - g  \tilde{\Phi} \left( \frac{n-1}{g} \right) \right],$$\
where\
$$\Phi (m) = \sum_{k=2}^{m} \varphi (k)$$\
$$\tilde{\Phi} (m) = \sum_{k=2}^{m} k \varphi (k)$$\
For which the values $\lbrace \varphi (k) \rbrace$, $\lbrace \Phi (m) \rbrace$, and $\lbrace \tilde{\Phi} (m) \rbrace$ can be precomputed. 

This reformulation drastically reduces scaling, making the computation efficient for large values of $n$.

[//]: # (Links)
[rust]: https://www.rust-lang.org
[repo]: https://github.com/zwill22/big-sum
[repo-actions]: https://github.com/zwill22/big-sum/actions
[rust-test]: https://github.com/zwill22/big-sum/actions/workflows/test.yml
[coffee]: https://coff.ee/zmwill
[license]: https://github.com/zwill22/big-sum/blob/main/LICENSE

[//]: # (Badges)
[rust-badge]: https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white
[github-badge]: https://img.shields.io/badge/GitHub-%23121011.svg?logo=github&logoColor=white
[actions-badge]: https://img.shields.io/badge/GitHub_Actions-2088FF?logo=github-actions&logoColor=white
[rust-test-badge]: https://github.com/zwill22/big-sum/actions/workflows/test.yml/badge.svg
[buy-me-coffee]: https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?logo=buy-me-a-coffee&logoColor=black
[license-badge]: https://img.shields.io/github/license/zwill22/big-sum
