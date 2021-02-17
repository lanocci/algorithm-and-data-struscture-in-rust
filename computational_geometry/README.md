# 計算幾何学

- 幾何学的な問題をコンピュータで解くための効率的なアルゴリズムとデータ構造について
- 図形、ベクトル、三角関数等の知識が必要

## 基本要素

### 点とベクトル

- ベクトル: 大きさとともに向きを持つ量
  - 大きさのみを持つ量を「スカラー」という
- プログラムのデータ量として表すために、一つのベクトルを原点から対象となる点 `P(x, y)` への有向な線分と考える
- 点: 平面幾何のもっとも基本的な要素
- 構造体として例えば以下のように表現できる
  - `struct Point { x: f32, y: f32}`
- ベクトルもただ一つの点として定義できる
  - `type Vector = Point`

### 線分と直線

- 線分: 2つの端点とそれらの距離で定義された長さを持つ線
- 直線: 2点を通る、長さが定義されていない点（端点がない）
- 直線と線分はプログラム上は同じように定義することができる
  - `type Line = Segment`

### 円

```
struct Circle {
  c: Point,
  r: f32,
}
```

### 多角形

`type Polygon = Vec<Point>`

### ベクトルの演算

```
impl ops::Add for Point {
  type Output = Self;

  fn add(self, rhs: Point) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl ops::Sub for Point {
  type Output = Self;

  fn sub(self, rhs: Point) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl ops::Mul for Point {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    Self {
      x: x * rhs,
      y: y * rhs,
    }
  }
}
```

### ベクトルの大きさ

- ベクトル `a = (a<x>, a<y>)` の大きさ `|a|` は原点からベクトルを表す点までの距離
- ベクトルの大きさの2乗を表す関数 `norm` は以下のように定義できる

```
fn norm(a: Vector) -> f32 {
  a.x * a.x + a.y * a.y
}

fn abs(a: Vector) -> f32 {
  norm(a).sqrt()
}
```