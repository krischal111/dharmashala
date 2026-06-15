By: Krischal Khanal

## Coding the parallel Bayesian update

```python
def bayesian_update_multi(priors, likelihoods):
    """
    priors      : list of P(H_i)       — must sum to 1
    likelihoods : list of P(E | H_i)   — one per hypothesis
    Returns     : list of posteriors P(H_i | E)
    """
    # Compute unnormalized posteriors
    unnormalized = [p * l for p, l in zip(priors, likelihoods)]

    # Marginal P(E) is the normalizing constant
    marginal = sum(unnormalized)

    # Normalize
    posteriors = [u / marginal for u in unnormalized]
    return posteriors


# Urn example
priors = [0.2, 0.5, 0.3]
likelihoods = [0.9, 0.4, 0.1]

posteriors = bayesian_update_multi(priors, likelihoods)
labels = ["Urn A", "Urn B", "Urn C"]

for label, prior, posterior in zip(labels, priors, posteriors):
    print(f"{label}: prior={prior:.2f}  →  posterior={posterior:.3f}")
```

```
Urn A: prior=0.20  →  posterior=0.439
Urn B: prior=0.50  →  posterior=0.488
Urn C: prior=0.30  →  posterior=0.073
```

The red ball makes urn B slightly more likely than urn A, and strongly disfavors urn C.

## Sequential updating with multiple hypotheses

The same composability from the last blog applies here. Each posterior becomes the next prior.

```python
def simulate_sequential_updates(priors, evidence_sequence, likelihood_table):
    """
    priors            : initial list of P(H_i)
    evidence_sequence : list of evidence labels (e.g., ['red', 'red', 'blue'])
    likelihood_table  : dict mapping evidence label → list of P(E | H_i)
    """
    current = priors[:]
    for evidence in evidence_sequence:
        likelihoods = likelihood_table[evidence]
        current = bayesian_update_multi(current, likelihoods)
        print(f"After observing '{evidence}': {[f'{p:.3f}' for p in current]}")
    return current


likelihood_table = {
    "red":  [0.9, 0.4, 0.1],
    "blue": [0.1, 0.6, 0.9],
}

print(f"Initial priors: {priors}")
final = simulate_sequential_updates([0.2, 0.5, 0.3], ["red", "red", "blue"], likelihood_table)
```

```
Initial priors: [0.2, 0.5, 0.3]
After observing 'red':  ['0.439', '0.488', '0.073']
After observing 'red':  ['0.681', '0.305', '0.015']
After observing 'blue': ['0.120', '0.323', '0.557']
```

Two red balls shifted belief strongly toward urn A. One blue ball then swung it back toward urn C. The hypotheses compete on every draw.

---

In this blog, we got a little peek into what happens if evidences are different, namely observing two "red" and one "blue". In the next blog, we will dive deeply into evidences that are stochastically observed, and how the belief is shaped by them. Stay tuned.

---
Code blocks are licensed under the [MIT License](https://opensource.org/license/mit). All other content is licensed under [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/) Copyright © 2026 Krischal Khanal.