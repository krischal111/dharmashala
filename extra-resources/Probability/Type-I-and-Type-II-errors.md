# Type I and Type II errors
## in cases of Null and Alternative Hypotheses

In cases like:

1. Probabilistic Experiment
2. Sceintific Testing
3. Statistical Research
4. Measurement
0. and more cases

We perform a calculation or estimation or judgement based on various things. We make an estimation about something. Those estimation may be true or false. Those estimation may be close to truth or be far from truth. We have no absolute guarantee that the estimation is true.

Then there is something called Ground Truth. It is the actual truth value of the experiment. Ground Truth is the ultimate truth. All of our experiment are centered around knowing or getting closer to the ground truth.

#### Examples of Hypothesis
Now, there is something called Hypothesis. In general terms, the word 'theory' plays that role, but in scientific terms, that word means something that is already well established. However the term 'Hypothesis' means a statement that might be true but we don't know it yet.

Like, we could say that "Average Height of this Class is exactly 65 inch." It is likely be untrue because height is continuous. But how far is it from the truth? Could it be 66 inch? If so, it's not so significant.

We could also say a person accused of crime as "He is innocent." But do we know if it's actually the case? It may be that he may actually be innocent or he could be guilty. We generally don't know the ground truth.

## Legal system - presumption of innocence.

While testing a particular hypothesis, we create two hypotheses:

Null Hypothesis: In general null hypothesis is so chosen that it agrees with the presumption hypothesis we are tryna verify.
But various sources try to explain that null hypothesis claims that the existence of relation of one experiment to another is null, i.e. there is no significant relationship. I am finding hard time wrapping my head around that.

Alternate hypothesis: It's any hypothesis that disagrees with the Null Hypothesis. The books I am reading claims that it doesn't have to be complement of the null hypothesis.

Let us consider a legal system where a person is accused of a crime.

<!-- $$ \sum_{n = 1}^{100} n^2 + 2n + 1 $$ -->


<style>
    .with_border {
        border: 1 px solid black;
        border-collapse: collapse;
    }
    th {
        border: 2px solid black;
    }
    tr {
        border: 1px;
    }
    td {
        border: none;
    }
</style>

<table style="border: none; text-align: center;">
    <tr>
        <th colspan = "2" rowspan = "2">A person in a Trial</th>
        <th colspan = "2">Ground Truth</th>
        <th rowspan = "2">Marginal Probabilities of Jury's Verdict</th>
    </tr>
    <tr>
        <th>Actually Innocent</th>
        <th>Actually Guilty</th>
    </tr>
    <tr>
        <th rowspan="2"> Jury's Verdict</th>
        <th>Judged Innocent </th>
        <td>Judge Correctly Identified the Innocent</td>
        <td>Judge failed to convict the Guilty (Type II error)</td>
        <td>Judge declares innocence</td>
    </tr>
    <tr>
        <th>Judged Guilty</th>
        <td>Judge wrongly convicted the innocent (Type I error)</td>
        <td>Judge Correctly Identified the Guilty</td>
        <td>Judge declares guilty</td>
    </tr>
    <tr>
        <th colspan="2">Marginal Probabilities of Ground Truth</th>
        <td>Person is actually innocent.</td>
        <td>Person is actually guilty</td>
        <td>Total</td>
    </tr>
</table>