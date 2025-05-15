## A simpler proof for Law of Total Variance:

$$ Var(Y) = E(Var(Y| X)) + Var(E(Y | X)) $$

We will use 3 laws:

Law 1: Linearity of the Expected Values:

$$E(X + Y) = E(X) + E(Y)$$

Law 2: Law of Total Expectation

$$ E(Y) = E(E(Y|X)) $$

Law 3: Partition of variance into Expected Values

$$ Var(X) = E(X^2) - E(X)^2 $$

Rearranging:

$$ E(X^2) = Var(X) + E(X) ^ 2 $$

### Proof:

Start with the RHS:
$$ RHS = E(Var(Y| X)) + Var(E(Y | X)) $$

Using law 3:

$$ RHS = E(Var(Y| X)) + E(E(Y | X)^2) - E(E(Y | X))^2 $$

Using Law 1 and Law 2:

$$ RHS = E(Var(Y| X) + E(Y | X)^2) - E(Y)^2 $$

Using law 3:

$$ RHS = E(E(Y^2 | X)) - E(Y)^2 $$

Using law 2:

$$ RHS = E(Y^2) - E(Y)^2 $$

Using law 3:

$$ RHS = Var(Y) = LHS $$
