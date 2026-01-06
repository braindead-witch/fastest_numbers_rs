## DISCLAIMER

This is not in any way, shape, or form meant to be a 'better' or 'more optimized' version of the [original project](https://github.com/thegraycuber/fastest_numbers) by TheGrayCuber. This is just an interesting coding challenge for myself, as I want to learn Rust. 

**If you would like to contribute, please do so to [the original project](https://github.com/thegraycuber/fastest_numbers/issues/6#issuecomment-3712957318), not mine.**

Or, you could wait until [TheGrayCuber](https://www.youtube.com/@TheGrayCuber) uploads the second video in the series they're cooking up, [which will contain much more silliness](https://github.com/thegraycuber/fastest_numbers/issues/5#issuecomment-3712953631)!

## What's the fastest way to say each number? 

Please note: pull requests will only be approved for the 'collaborative' titled file or added files in other languages. The existing English version will stay static to match the logic used in the video and on the website.

'Twenty-seven' is 4 syllables, but 'three cubed' is only 2! This program calculates the least amount of syllables needed to say each number, as discussed in my video: https://youtu.be/Ff8qIBUu4wM

The process is as follows:

  - Generate the original name and syllable count for each number in the range. This is in the form of 'one hundred twenty-three thousand four hundred fifty-six' for 123456.
  - Find all numbers that can be said using 1 syllable.
  - Find all numbers that can be said using 2 syllables.
  - Find all numbers that can be said using 3 syllables.
  - Continue until all numbers in the range have a name.

During the step of finding names with N syllables, we attempt to construct numbers using results from lower syllable counts. For example:

  - (N-1 syllables) squared
  - (N-1 syllables) cubed
  - (N-2 syllables) plus (1 syllable)
  - (N-3 syllables) plus (2 syllables)
  - (N-4 syllables) plus (3 syllables)
  - and so forth.

A lot of the complexity comes from order of operations. We assume that parentheses cannot be said, so "twelve times nine plus one" will evaluate to 109, not 120. For each number we store the shortest name at each level: original, exponent and fraction, multiplication, division, addition and subtraction. While attempting construction, we take the highest level allowed for the operation. "eight times nine" would not be allowed as the input for multiplication, while "seventy-two" would be allowed. 

There is additional complexity from the use of fractions. For example, we can say 32 as "four cubed halves". This use of "halves" or "thirds" makes division faster, but it can be ambiguous. With the term "one hundred twenty fifths", it's not clear whether this means 120/5 or 100/25, so this is not allowed. The code includes logic to check whether a fraction will have more than one interpretation, and should therefore by disallowed.

### About the versions
There are four versions of the English code:

  - main. This was used in the initial video and the main version of the website.
  - collaborative. This is open for anyone to make changes.
  - ance. This is the adv-ance-d version used in the sequel video.
  - serious. This is a more detailed but serious version that adds rounding, irrationals, huge numbers, and more.
