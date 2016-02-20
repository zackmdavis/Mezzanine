*We will classify our gloss'ry of shapes into compliance!  
We are magical methodical apes doing triangle science!*  

---

*(October 2015)*

Today the Code Muse spoke to me on the train and said that I should think writing an AI for Zendo-like guessing games should be doable.

We need to maintain a finite (finite _and small_—it has to fit in memory!) list of hypotheses weighted by simplicity—more constraints mean lower prior probability. Our notion of updating will have a discrete sort of Solomonoff/Popperian flavor in that each observation can only falsify or not-falsify a hypothesis—an observation can't quantitatively favor one not-definitively-falsified hypothesis over another. The interesting part of the endeavor, then, is _not_ how we update our beliefs in response to evidence, but rather, _what evidence we choose to seek out_. Given a finite and small state space of examples, we can iterate over them, considering what effect each possible answer (Yes or No) will have on some measure of the degree of "uncertainness" our distribution over hypotheses represents—that's [entropy](https://en.wikipedia.org/wiki/Entropy_%28information_theory%29), I think. The alternative that reduces entropy the most (averaged over Yes and No—with equal weight??—I don't think that's right, but pressing on—) will be our guess.

Instead of the complicated triangle science of Zendo proper, let's start by considering the universe of integers from 1 through 100 inclusive. (I was going to start counting at zero, but that's no good for reasons that will become clear in a moment.) If we consider the class of hypotheses which suppose that one particular number has the Buddha-nature, it's pretty clear that the principle of indifference says we have to give all 100 equal probability. That's boring. A more interesting class of hypotheses with a nontrivial "simplicity structure" would be rules of the form "A number has the Buddha-nature iff it is divisible by _n_." Then the prior probability of the "_n_ = 6" hypothesis would have to be strictly less than that of the "_n_ = 2" and "_n_ = 3" hypotheses.

... _except_ that can't be the right conceptual model. If the master's rule is "A number has the Buddha-nature iff it is divisible by 6", then it's impossible to falsify the hypothesis that _n_ = 2 because that hypothesis is in fact _true_. We need to be able to express the idea that "_n_ = 2" isn't a _sufficient_ answer. ... no, never mind, I think I was right the first time, and then I confused myself, and then I just unconfused myself by happening to use the word "sufficient". Remember I said _iff_ with two 'f's: that means necessary _and_ sufficient conditions. So a number that is divisible by 2 and yet does not have the Buddha-nature would falsify the _n_ = 2 hypothesis.

But then where do my uneven prior probabilities come from, if anywhere?

—they don't. Priors are a distraction. The prior distribution represents our beliefs about the master's beliefs about the structure of the Buddha-nature. That's just not something that can be governed by the fundamental theorem of arithmetic! But the evidential impact of the master's feedback is: if we guess 79 and hear "No", we don't learn much, whereas if we guess 60 and hear "No," we can rule out _n_ being 2, 3, 5, 6, 12, or 30. So we _probably_ want to inquire about integers with a number of factors as close as possible to half the number of remaining hypotheses, for the same reason that must underlie the information-theoretic correctness of binary search.

---

*(several months later—scene: a coffeeshop welded to a bookstore on Valencia Street)*

"I hate this game," I said.

---

Standard hypothesis complement:

 * exactly _n_ triangles of color _c_ for 1 ≤ _n_ ≤ 3
 * at least _n_ triangles of color _c_ for 1 ≤ _n_ ≤ 3
 * not more than _n_ triangles of color _c_ for 0 ≤ _n_ ≤ 2
 * exactly _n_ triangles of size _s_ for 1 ≤ _n_ ≤ 3
 * at least _n_ triangles of size _s_ for 1 ≤ _n_ ≤ 3
 * not more than _n_ triangles of size _s_ for 0 ≤ _n_ ≤ 2
 * total pip count is even or odd
 * total pip count is divisible by 3, 4, or 5

_also_—

 * sensible (single) conjunctions or disjunctions of the above

---

Inspired by [compwron/mez](https://github.com/compwron/mez).
