## Bayes' Theorem Intuitive proof

We have established that:
$$\text{Conditional Probability} = \frac{\text{Joint Probability}}{\text{Marginal Probability}}$$

For the same example of cards:

We can say that:

1. First
$$P(\text{Red when 4}) = \frac{P(\text{4 and Red})}{P(4)}$$
$$\implies P(\text{4 and Red}) = P(\text{Red when 4}) \cdot P(4)$$

2. Second

$$P(\text{4 when Red}) = \frac{P(\text{4 and Red})}{P(Red)} $$
$$\implies P(\text{4 and Red}) = P(\text{4 when Red}) \cdot P(Red)$$

From the first and the second, equation the equal terms:


$$P(\text{Red when 4}) \cdot P(4) = P(\text{4 when Red}) \cdot P(Red)$$
$$\implies P(\text{Red when 4}) = \frac{P(\text{4 when Red}) \cdot P(Red)}{P(4)}$$

We can generalize it as:

$$P(A\ |\ B) = \frac{P(B\ |\ A)\cdot P(A)}{P(B)}$$

This is the Bayes' Theorem.