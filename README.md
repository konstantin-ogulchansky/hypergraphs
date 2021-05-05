# hypergraphs

A repository that contains an implementation of a hypergraph model with vertex ageing.
This model generates hypergraphs whose degree distribution follows a power-law
distribution with an exponential cutoff: `P(k) ~ C k^{-a} / b^k`.

### Model Description

Given the initial hypergraph `H_0`, construct a hypergraph `H_t` from `H_{t-1}` as follows:
- wp `pv`, add a vertex and a preferentially selected hyperedge of size `Y_t`,
- wp `pe`, add a preferentially selected hyperedge of size `Y_t`,
- wp `pd`, deactivate a preferentially selected vertex;

(wp denotes "with probability").

Preferential selection means that the probability of selecting a vertex is proportional to its
degree. That is, `P(v is chosen) = deg v / sum [deg u, u in V]`.
