Today the Code Muse spoke to me on the train and said that I should think writing an AI for Zendo-like guessing games should be doable.

We need to maintain a finite (finite _and small_—it has to fit in memory!) list of hypotheses weighted by simplicity—more constraints mean lower prior probability. Our notion of updating will have a discrete sort of Solomonoff/Popperian flavor in that each observation can only falsify or not-falsify a hypothesis—an observation can't quantitatively favor one not-definitively-falsified hypothesis over another. The interesting part of the endeavor, then, is _not_ how we update our beliefs in response to evidence, but rather, _what evidence we choose to seek out_. Given a finite and small state space of examples, we can iterate over them, considering what effect each possible answer (Yes or No) will have on some measure of the degree of "uncertainness" our distribution over hypotheses represents—that's [entropy](https://en.wikipedia.org/wiki/Entropy_%28information_theory%29), I think. The alternative that reduces entropy the most (averaged over Yes and No—with equal weight??—I don't think that's right, but pressing on—) will be our guess.

Instead of the complicated triangle science of Zendo proper, let's start by considering the universe of integers from 1 through 100 inclusive. (I was going to start counting at zero, but that's no good for reasons that will become clear in a moment.) If we consider the class of hypotheses which suppose that one particular number has the Buddha-nature, it's pretty clear that the principle of indifference says we have to give all 100 equal probability. That's boring. A more interesting class of hypotheses with a nontrivial "simplicity structure" would be rules of the form "A number has the Buddha-nature iff it is divisible by _n_." Then the prior probability of the "_n_ = 6" hypothesis would have to be strictly less than that of the "_n_ = 2" and "_n_ = 3" hypotheses.

... _except_ that can't be the right conceptual model. If the master's rule is "A number has the Buddha-nature iff it is divisible by 6", then it's impossible to falsify the hypothesis that _n_ = 2 because that hypothesis is in fact _true_. We need to be able to express the idea that "_n_ = 2" isn't a _sufficient_ answer. ... no, never mind, I think I was right the first time, and then I confused myself, and then I just unconfused myself by happening to use the word "sufficient". Remember I said _iff_ with two _f_s: that means necessary _and_ sufficient conditions. So a number that is divisible by 2 and yet does not have the Buddha-nature would falsify the _n_ = 2 hypothesis.

But then where do my uneven prior probabilities come from, if anywhere?