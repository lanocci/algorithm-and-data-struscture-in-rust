# Graph - Basics

## Symbols

- `G = (V, E)`
  - G: A graph
  - V: A group of vertices (node) 頂点
  - E: A group of edges 辺
  - |V|: Number of vertices in the group
  - |E|: Number of edges in the group
- `e = (u, v)`
  - e: An edge
  - u, v: Vertices
  - w(u, v): Weight of the edge from vertex u to vertex v

## Terminology

### General

- A *degree* of vertex u is a number of edges connected to the vertex.
  - In a directed graph, number of edges to a vertex is *indegree*, number of edges from a vertex is *outdegree*.
- A *connected graph* is a graph where any pair of vertex has a path.
- グラフGとG'において、G'の頂点集合と辺集合の両方がGの頂点集合と辺集合の部分集合になっているとき、G'はGの部分集合という

### Undirected Graph

- A graph where all the edges don't have direction (i.e. all the edges are bidirectional).
- When there is an edge between vertex u and v, vertex u and v are *adjacent*.
- A row of adjacent vertices are called *path*.
- *Cycle* is a path where the first vertex is equal to the last vertex.

### Directed Graph

- A graph where all the edges have direction.
- A *Direct Acyclic Graph (DAG)* is a directed graph where no cycle exists.

## Algorithms

- search
  - *Depth First Search (DFS)*
  - *Breadth First Search (BFS)*
    - applies to shortest path problem