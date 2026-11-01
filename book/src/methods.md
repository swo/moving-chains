# Methods

## General conceptual overview

We consider housing *units* and the *households* the occupy those units, for a single metro area.
Unoccupied units are called *vacancies*.
Units have one associated property, *price*, reflecting rent(?).

> [!CAUTION]
>
> How to deal with rentals versus homes?

Households have preferences about units, reflecting things like household income.
In the simulation, households will move to different units, with certain probabilities and on certain timescales.
Households can be created, reflecting processes like individuals separating from their previous households or people moving into the metro from other metros.
Households can be destroyed, reflecting processes like out-migration, combining households, and death.

We are interested in how the introduction of new vacancies (i.e., via construction of new units), especially of new units at market rates (i.e., higher prices), translates into vacancies at lower prices.

## Microsimulation approach

### Bucketing

For convenience, bucket households into categories (e.g., income deciles) and bucket units into categories (e.g., deciles of the median income of their occupants).

### Dynamics

Every household $h$ has a preference for every vacant unit $u$, expressed as a constant hazard $r_{hu}$, representing the instantaneous probability that they will move to that unit.
The actual delay until they move to that unit, if it is still vacant, is distributed like $\mathrm{Exp}(r_{hu})$, which has mean $1/r_{hu}$.
If a household moves, they discard all their existing delays to move into vacant units but then immediately restart their search for vacancies.

There is a race for every vacancy.
If there is only one vacancy, then the family with the shortest delay will move in.
If there is more than one vacancy, then the dynamics become more complicated: while the race for one vacancy is ongoing, another vacancy may open and divert competitors for the first vacancy.

### Implementation

Mathematically, there is a time-varying matrix of times $t_{hu}$ when household $h$ will move into unit $u$, if it is still vacant.
In practice, this is cumbersome and unnecessary; we can instead proceed from the perspective of each vacancy: when a unit $u$ becomes vacant:

1. Draw the time until someone moves in, with hazard $\sum_h r_{hu}$.
1. At that time, select the mover with probability proportional to $r_{hu}$.

> [!CAUTION]
> This implies that doubling the number of vacancies doubles the probability that a person will move.
> This is true in the limit of very few vacancies, but something probably doesn't scale after that.

> [!CAUTION]
> Depending on the parameterization, people might be moving very often.
> There is no delay upon moving in; that delay is encoded in people's preferences for each unit.

> [!CAUTION]
> Rework this math to be in the sense of the "buckets"?
> If people in the buckets are identical, then this is really a stochastic, compartmental model...
