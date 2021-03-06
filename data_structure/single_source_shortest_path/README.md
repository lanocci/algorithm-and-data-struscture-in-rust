## Shortest Path Problem 最短経路問題

- 重み付きグラフにおいて、ある与えられた頂点の組 `(s, d)` を接続する経路の中で、辺の重みの総和が最小であるパスを求める問題

### Single Source Shortest Path(SSSP) 単一始点最短経路

- 与えられた一つの頂点から他の全ての頂点への最短経路を求める問題

### All Pairs Shortest Path(APSP) 全点対間最短経路

- 全ての頂点のペア間の最短経路を求める問題

### Shortest Path Spanning Tree 最短経路木

- グラフGのなかの頂点sからGの全ての頂点に対してパスがある時、sを根としてsからGの全ての頂点への最短経路を包含するGの全域木

### Dijkstra's algorithm

- Graph `G = (V, E)`
    - `V` is the set of all vertices in the graph
    - `E` is the set of all edges in the graph
- The start point: `s`
- The set of all vertices in the shortest path `S`
- For each vertex `i`, minimum cost of the path to the `s` is `d[i]`
- The parent vertex of a vertex `i` in the shortest path tree is `p[i]`
