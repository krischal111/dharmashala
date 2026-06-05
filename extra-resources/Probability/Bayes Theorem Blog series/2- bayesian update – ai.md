By: Krischal Khanal

In the previous blog, we saw how Bayes' Theorem lets us *invert* a conditional probability. Now we look at its second — and arguably more profound — application: **updating belief**.

## The rare disease problem

Suppose there is a rare disease. Only **1 in 1000 people** have it. You go for a routine checkup and take a diagnostic test. The test comes back **positive**. Should you panic?

Before we answer that, we need to know a bit more about the test:
- If you *have* the disease, the test correctly catches it **99% of the time** (sensitivity).
- If you *don't* have the disease, the test incorrectly flags you **1% of the time** (false positive rate).

So: how likely are you to actually have the disease, given a positive test?

Intuitively, many people — including doctors — answer somewhere around 99%. The actual answer is roughly **9%**. Bayes' Theorem is what lets us compute this correctly.

---

## Priori and Posteriori

Before seeing any evidence, you have some belief about a hypothesis. This is the **prior** (or *priori*) — it encodes what you know *before* the evidence arrives.

After seeing evidence, your belief gets updated. The updated belief is the **posterior** (or *posteriori*).

The whole machinery of Bayesian updating is just this: you start with a prior, you observe evidence, and you compute a posterior.

$$\underbrace{P(H|E)}_{\text{posterior}} = \frac{\underbrace{P(H)}_{\text{prior}} \cdot \underbrace{P(E|H)}_{\text{likelihood}}}{\underbrace{P(E)}_{\text{marginal}}}$$

In the disease example:
- $H$ : You have the disease
- $E$ : You tested positive
- $P(H) = 0.001$ — prior (1 in 1000 people have it)
- $P(E|H) = 0.99$ — likelihood (test catches disease 99% of the time)
- $P(\neg H) = 0.999$ — prior for not having the disease
- $P(E|\neg H) = 0.01$ — false positive rate

The marginal $P(E)$ is the total probability of testing positive, regardless of whether you have the disease:

$$P(E) = P(H) \cdot P(E|H) + P(\neg H) \cdot P(E|\neg H)$$
$$P(E) = (0.001)(0.99) + (0.999)(0.01) = 0.00099 + 0.00999 = 0.01098$$

Now applying Bayes' Theorem:

$$P(H|E) = \frac{0.001 \times 0.99}{0.01098} \approx 0.0902$$

So there is only about a **9% chance** you actually have the disease, even after a positive test.

---

## The thinking process

Here is the intuition, written as a stream of thought:

> *"In general, 1 in 1000 people have this disease — so before any test, that's my starting belief.*
>
> *Now, if I did have it, this test would be positive 99% of the time. And if I didn't, it would still be positive 1% of the time.*
>
> *The disease is so rare that even a 1% false alarm rate across 999 healthy people produces far more false positives than the test produces true positives from the 1 sick person.*
>
> *So yes, I tested positive — but most people who test positive are healthy. I'm probably fine, but I should take a follow-up test."*

This is Bayesian reasoning. The evidence updates your belief — but it does not override it. The prior matters.

---

## What information do we need?

To apply Bayesian updating, you need three things:

1. **Prior** $P(H)$ — your belief before the evidence. Often comes from base rates, domain knowledge, or past experience.
2. **Likelihood** $P(E|H)$ — how probable the evidence is, assuming the hypothesis is true. Often comes from empirical measurements (e.g., test sensitivity).
3. **Likelihood under negation** $P(E|\neg H)$ — how probable the evidence is if the hypothesis is *false*. (e.g., false positive rate).

The marginal $P(E)$ is derived from these, not independently supplied.

---

## The assumptions

Bayesian updating assumes:
- Your prior is a genuine probability — it reflects your actual degree of belief.
- The likelihoods are accurate — garbage in, garbage out.
- The hypothesis and its negation are **exhaustive** — one of them must be true.
- Each update is independent — if you're doing sequential updates, each new piece of evidence should be conditionally independent of the others, given the hypothesis.

---

## The constraint: you cannot update a certainty

Here is something subtle but important. What happens if your prior is $P(H) = 0$ or $P(H) = 1$?

If $P(H) = 0$:
$$P(H|E) = \frac{0 \cdot P(E|H)}{P(E)} = 0$$

If $P(H) = 1$:
$$P(H|E) = \frac{1 \cdot P(E|H)}{P(E)} = \frac{P(E|H)}{P(E|H)} = 1$$

No evidence can change a belief of 0% or 100%. The math simply returns the same value.

This is not just a mathematical curiosity — it is a statement about epistemology. **If you are completely certain of something, no evidence can make you reconsider.** Bayes' Theorem is, in a quiet way, an argument for keeping an open mind. Reserve a little probability for being wrong. Even $P(H) = 0.0001$ can be updated by strong enough evidence. But $P(H) = 0$ never can.

---

## Coding it up

```python
def bayesian_update(prior, likelihood_given_true, likelihood_given_false):
    """
    prior                 : P(H)       — prior probability of hypothesis
    likelihood_given_true : P(E|H)     — probability of evidence if H is true
    likelihood_given_false: P(E|¬H)    — probability of evidence if H is false
    """
    marginal = (prior * likelihood_given_true) + ((1 - prior) * likelihood_given_false)
    posterior = (prior * likelihood_given_true) / marginal
    return posterior

# The rare disease example
prior = 0.001       # 1 in 1000 people have the disease
p_e_given_h = 0.99  # test sensitivity
p_e_given_not_h = 0.01  # false positive rate

posterior = bayesian_update(prior, p_e_given_h, p_e_given_not_h)
print(f"Posterior probability of disease given positive test: {posterior:.4f}")
# Output: Posterior probability of disease given positive test: 0.0902
```

---

## Sequential updating

One elegant property of Bayesian updating: you can do it *repeatedly*. The posterior from one update becomes the prior for the next.

Say you take a second, independent test, and it also comes back positive:

```python
# First positive test
posterior_1 = bayesian_update(0.001, 0.99, 0.01)
print(f"After first positive test:  {posterior_1:.4f}")

# Second positive test — use posterior_1 as new prior
posterior_2 = bayesian_update(posterior_1, 0.99, 0.01)
print(f"After second positive test: {posterior_2:.4f}")
```

```
After first positive test:  0.0902
After second positive test: 0.9075
```

After two independent positive tests, you are now over 90% likely to have the disease. The evidence accumulates.

---

In the next blog, we will look at what happens when the hypothesis is not just $H$ and $\neg H$ — but one of *many* possible hypotheses. That is the missing half.