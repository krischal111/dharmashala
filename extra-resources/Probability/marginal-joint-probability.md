# Marginal, Joint and Conditional Probability.
## And also the Bayes' Theorem


Let's consider a deck of cards:

Let's choose a card randomly. We need to determine the probibility that:
- The card is a red.
- The card is a 4.

** In logic, the english "but" is equivalent to logical "and".

<style>
    .with_border {
        border: 1 px solid black;
        border-collapse: collapse;
    }
    th {
        border: 2px solid black;
        text-align: center;
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
        <th colspan = "2" rowspan = "2">A randomly chosen card</th>
        <th colspan = "2">Color (Joint probability) </th>
        <th rowspan = "2">Rank (Marginal probability) </th>
    </tr>
    <tr>
        <th>Red</th>
        <th>Not Red</th>
    </tr>
    <tr>
        <th rowspan="2">Rank (Joint Probability)</th>
        <th>4</th>
        <td>P(4 and Red)</td>
        <td>P(4 but not Red)</td>
        <td>P(4)</td>
    </tr>
    <tr>
        <th>Not 4</th>
        <td>P(Red but not 4)</td>
        <td>P(Neither Red nor 4)</td>
        <td>P(Not 4)</td>
    </tr>
    <tr>
        <th colspan="2">Color (Marginal probability)</th>
        <td>P(Red)</td>
        <td>P(Not Red)</td>
        <td>100%</td>
    </tr>
</table>

First of all, we have two sets marginal probabilites:

1. Color:
    1. $P(Red)$
    1. $P(\text{not Red}) = P(\overline{Red})$
1. Rank:
    1. $P(4)$
    1. $P(\text{not 4}) = P(\overline{4}))$

Now we have 4 sets of the Joint distributions over the color and rank.

1. A red 4 <br> $P(4) \cup P(Red)$
2. A red but not 4 <br> $P(\bar{4}) \cup P(Red)$
3. Not red but 4 <br> $P(4) \cup P(\overline{Red})$
4. Red but not 4 <br> $P(\bar{4}) \cup P(\overline{Red})$


Now the conditional probalities:

1. On color:
    1. When Red:
        1. It's 4 <br> $P(4\ |\ Red)$
        1. It's not 4 <br> $P(\bar{4}\ |\ Red)$
    2. When not red:
        1. It's 4 <br> $P(4\ |\ \overline{Red})$
        1. It's not 4 <br> $P(\bar{4}\ |\ \overline{Red})$
2. On rank
    1. When 4:
        1. It's red <br> $P(Red\ |\ 4)$
        1. It's not red <br> $P(\overline{Red}\ |\ 4)$
    2. When not 4:
        1. It's red <br> $P(Red\ |\ \bar{4})$
        1. It's not red <br> $P(\overline{Red}\ |\ \bar{4})$


Now, we can get the conditional probabilites from the joint and marginal probabilites as:

$$P(\text{Red when 4}) = \frac{P(\text{4 and Red})}{P(4)} $$

Generalizing:

$$\text{Conditional Probability} = \frac{\text{Joint Probability}}{\text{Marginal Probability}}$$


This simple relation give rise to the Bayes' theorem as:

### [Bayes-theorem-proof](Bayes-theorem-simple-proof.md)