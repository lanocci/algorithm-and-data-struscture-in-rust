# Matrix Chain Multiplication Problem

- an example of dynamic programming(DP)

## どんな問題？

- 複数の行列を掛け算するときに、もっとも計算回数が少なくなるような計算順を求めたい
- 2つの行列の掛け算を行う際、1つめの行列の行数を p0, 列数を p1, 2つめの行列の行数を p1, 列数を p2 としたとき、p0 x p1 x p2 回のスカラ演算が必要
- 一般化すると
  - M<i>: i番目の行列
  - p<0>: M<1>の行数
  - p<1>: M<1>の列数かつM<2>の行数
  - ...
  - (M<i> * M<i+1>) を計算する方法は一通りで p<i-1> * p<i> * p<i+1> 回の掛け算が必要

## Input

- number of matrices (n)
- n pairs of numbers; pairs of a number of rows(r) and a number of columns(c)

## Output

- possible minimum count of multiplication

## Variables

- memo[i][j]: i 番目から j 番目までの行列を計算する時の最小の計算数を記録しておく配列
- l: 計算しようとしている行列の数(j - i)