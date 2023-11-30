# Null and Alternative Hypotheses
## In a statistical test

First we measure a set of data from somewhere. It can be:
1. Probabilistic Experiment
2. Scientific Test
3. Statistical Research
4. Measurement
0. and more cases

In general, we don't know how the data are distributed.
They could be normally distributed, or they could just follow some other pattern.

We then make two assumptions about the data:

$H_0$ : A null hypothesis, for which we know how the data should be distributed if the hypothesis is true. 

For example, if we assume there is no other pattern than probabilistically randomly distributed measurement of something, then by central limit theorem, we can assume they take normal distribution.

$H_1$ : An alternative hypothesis. Anything that disagrees with the null hypothesis, and tries to explain the distribution of data.

$\alpha$: A significance level. It only relates to the null hypothesis. It tells us what probability we want for rejecting null hypothesis even if it's true, becaue it can happen, if we are too strict. 

$ 1- \alpha$ : Confidence interval. This gives us a probability of Accepting Null Hypothesis given that it's true.

Let us use some symbols:
$$ \begin{split}
H_0 &: \text{Null Hypothesis} \\
H_1 &: \text{Alternate Hypothesis} \\
T_0 &: \text{Event when Null Hypothesis is accepted}\\
T_1 &: \text{Event when the Alternate is rejected} \\
\\
\alpha &= P(\overline{T_0}\ |\ H_0) \\
1 - \alpha &= P(T_0\ |\ H_0)\\
\end{split}
$$

Here, we  can see that as we decrease $\alpha$, we increase chance of null hypothesis being accepted when it's true.

It has to be noted that, we haven't considered the case when Null Hypothesis is actually false.

In order to accept the null hypothesis when it's true, we might be so much more lenient on the test that, we might even begin to accept it when it is false.

So, to avoid such cases, we might be stringent on the criteria that accepts the null hypothesis. And as a result, the cases when null hypothesis is false, it's filtered out more often.

By decreasing $\alpha$: our significance level, we increase $1-\alpha$, our confidence interval, so we're confident about our Null Hypothesis for wider variations from the expected values. We accept wider variations are from chance alone.

By increase $\alpha$: our signifance level, we decrease $1-\alpha$, our confidence interval, so we accept the Null Hypothesis only if they are withing strict boundaries from the expected values.

Now, if your Null Hypothesis is actually accepted, we would like to know the probability of underlying truth be actually true. For that we can use the Bayes' theorem as:

$$P(H_0 | T_0) = \frac{P(T_0\ |\ H_0) \cdot P(H_0)}{P(T_0)}$$
$$P(H_0 | T_0) = \frac{P(T_0\ |\ H_0) \cdot P(H_0)}{ P(T_0, H_0) + P(T_0, \overline{H_0})}$$
$$P(H_0 | T_0) = \frac{P(T_0\ |\ H_0) \cdot (P(T_0, H_0) + P(\overline{T_0}, H_0))}{ P(T_0)}$$

I don't know what t do with these.