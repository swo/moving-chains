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
Households can be removed, reflecting processes like out-migration, combining households, and death.

We are interested in how the introduction of new vacancies (i.e., via construction of new units), especially of new units at market rates (i.e., higher prices), translates into vacancies at lower prices.

## Microsimulation approach

### Bucketing

For convenience, bucket households into categories (e.g., income deciles) and bucket units into categories (e.g., deciles of the median income of their occupants).

### In-metro dynamics

Every household $h$ has a preference $p_{hu}$ for every vacant unit $u$.
Every household $h$ has a *tenure* distribution.
After every move, that household randomly selects a tenure.
What that tenure elapses, the household moves into a new unit $u$, selected from among the then-vacant units with probability proportional to $p_{hu}$.
(If $p_{hu} = 0$ for all vacant units $u$, the household is removed, representing a move out of the metro.)

> [!NOTE]
> This assumption decouples the vacant stock from households' move times.
> The presence of more vacancies does not increase anyone's probability of moving.

### Household creation and removal

- Every household has a *lifetime*, determined upon household creation.
  At the end of its lifetime, the household is removed.
- New households are created at some rate.
  New households have characteristics drawn from some distribution.

### Data & calibration

- Draw in-metro dynamics from observed moving chains
- Select new household characteristics to match the distribution of new tenants
- Generally figure out how to parameterize everything
