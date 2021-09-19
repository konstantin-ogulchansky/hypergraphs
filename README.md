# Hypergraphs

An implementation of the _random preferential attachment hypergraph model with vertex deactivation_.

## Model

### Motivation

The main idea of this model is to generate random hypergraphs whose degree distribution follows a power-law distribution with an exponential cutoff:

```
P(k) ~ C k^{-a} / b^k
```
where `C`, `a` and `b` are some constant parameters of the distribution.

Such hypergraphs can be used to model real-world collaboration networks, where vertices correspond to authors and hyperedges correspond to publications.
It has been observed that collaboration networks expose the presence of the exponential cutoff in their degree distribution.

Basically, the model is a combination of the Avin et al. model [1], which generates scale-free hypergraphs, and the Fenner et al. model [2], which introduces vertex deactivation leading to the appearance of the exponential cutoff.

### Description

The model can be described as a 5-tuple `H(H_0, pv, pe, pd, Y)`, where
- `H_0` is the initial hypergraph (we assume that it consist of a single vertex with degree 1);
- `pv` is the probability of the vertex arrival event;
- `pe` is the probability of the edge arrival event;
- `pd` is the probability of the vertex deactivation event;
- `Y = (Y_1, Y_2, ...)` is a sequence of random variables, which represent sizes of added hyperedges.

The model is then defined as follows:
* **Step `t = 0`**. Initialize the model with `H_0`.
* **Step `t > 0`**. Construct a hypergraph `H_t` from `H_{t-1}` as follows:

  - with probability `pv`, add a vertex and a preferentially selected hyperedge of size `Y_t` between active vertices of `H_{t-1}`,
  - with probability `pe`, add a preferentially selected hyperedge of size `Y_t` between active vertices of `H_{t-1}`,
  - with probability `pd`, deactivate a preferentially selected active vertex of `H_{t-1}`;

Preferential selection means that the probability of selecting a vertex `v` from a set `A` is proportional to its degree. 
That is, 
```
P[v is chosen] = deg v / sum [deg u, u in A]
```

## Usage

The program can be used by simply running

```
cargo run --release
```

Additional parameters of the model can be specified after `--`.
Running

```bash
cargo run --release -- --help
```

yields

```
USAGE:
    hypergraphs.exe [FLAGS] [OPTIONS] [ARGS]

ARGS:
    <pv>    Probability of the `vertex-arrival` event [default: 0.30]
    <pe>    Probability of the `edge-arrival` event [default: 0.49]
    <pd>    Probability of the `vertex-deactivation` event [default: 0.21]
    <m>     Distribution of cardinalities of hyperedges [default: 3]
    <t>     Number of iterations to perform [default: 1000]

FLAGS:
    -h, --help       Prints help information
        --par        Whether runs should be parallelized
    -V, --version    Prints version information

OPTIONS:
        --retries <retries>    Number of retries to perform until the model finishes with success
                               [default: 100]
        --runs <runs>          Number of hypergraphs to generate [default: 5]
        --save <save>          Template path to a JSON file to save the generated hypergraph to
                               [default: data/hypergraph]
```

### Example

```bash
cargo run --release -- 0.3 0.49 0.21 3 1000 --save="data/hypergraph" --runs=100 --par
```

This command generates 100 hypergraphs according to the model with parameters `pv = 0.3`, `pe = 0.49`, `pd = 0.21` and `Y = 3` after `t = 1000` steps.
Also,
- hypergraphs are saved to files of the format `data/hypergraph-{i}.json`, where `i` corresponds to the index of the generated hypergraph;
- hypergraphs are generated in parallel.

### Format

A generated hypergraph is saved to a file in the JSON format:

```json
{
  "parameters": {
    "pv": 0.30,
    "pe": 0.49,
    "pd": 0.21,
    "m": 3,
    "t": 1000
  },
  "nodes": 311,
  "edges": [[0], [0, 0, 1], [0, 0, 2], ...],
  "degree": [27, 2, 2, 6, 12, 6, ...],
  "theta": [1.0, 2.5, 3.857142857142857, 6.6, ...]
}
```

Fields in this format represent the following:
* `parameters` contains parameters of the model that the hypergraph was generated with;
* `nodes` is the number of nodes in the hypergraph;
* `edges` is a list of hyperedges (each hyperedge may contain the same vertex several times);
* `degree` is a list of degrees of vertices;
* `theta` is a list of the expected degrees of deactivated vertices:
  - `theta[t] = sum of squares of active degrees / sum of active degrees` at step `t`.

## References

[1] Chen Avin, Zvi Lotker, Yinon Nahum, and David Peleg. 
    Random preferential attachment hypergraph. 
    In _ASONAM ’19: International Conference on Advances in Social Networks Analysis and Mining, 
    Vancouver, British Columbia, Canada, 27-30 August, 2019_,
    pages 398–405. ACM, August 2019. 
    [doi:10.1145/3341161.3342867](https://doi.org/10.1145/3341161.3342867).

[2] Trevor I. Fenner, Mark Levene, and George Loizou. 
    A model for collaboration networks giving rise to a power-law distribution with an exponential cutoff. 
    _Social Networks_, 
    29(1):70–80, 2007.
    [doi:10.1016/j.socnet.2005.12.003](https://doi.org/10.1016/j.socnet.2005.12.003).

## License

This package is licensed under the [MIT](LICENSE) license.
