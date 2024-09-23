# Fuzzy Extractors

## Usage

### Init

To create a Fuzzy Extractor, define a ```lattice``` and ```lattice_dim``` as below: 
```py
fe = FuzzyExtractor(lattice, lattice_dim)
```
The ```lattice``` must be a 2D array ($n \times n$) where ```lattice_dim``` = n

### Generate

```py
s, e = fe.gen(value)
```

### Recover

```py
ep = fe.recov(s, another_value)
```

## Example

```py
lat = [
    [1, 0, -1, 2],
    [2, 3, 1, -1],
    [1, 1, 2, 0], 
    [0, 1, 1, 3]
]

fe = FuzzyExtractor(lat, 4)
s, e = fe.gen(314)

# close
ep = fe.recov(s, 315)
print(e, ep, np.array_equal(e, ep))

# far
ep = fe.recov(s, 168)
print(e, ep, np.array_equal(e, ep))
```

For more examples, look at [pyFuzzExample.py](https://github.com/spaceybread/crispy-fiesta/blob/main/pyImpl/pyFuzzExample.py)

# Fuzzy Matching

## Usage

### Init

Create fuzzy extractor object and then use it to create a fuzzy matching object like this: 
```py
fe = FuzzyExtractor(lattice, lattice_dim)
fm = FuzzyMatching(fe)
```

### Generate keys and helpers

For a list of elements, ```S_A```, do this: 
```py
keys, helpers = FM.makeHelpers(S_A)
```

### Intermediate Matching

To attempt matching with a list ```S_B``` using ```helpers```, do this: 
```py
matchedMatrix = FM.attemptMatching(helpers, S_B)
```

### Final Matching

To find the final matches from ```S_A``` with the ```matchedMatrix```, do this: 
```py
final = FM.returnMatches(matchedMatrix, S_A, keys)
```
## Example

```py
LATTICE = [[1, 0, -1, 2],[2, 3, 1, -1],[1, 1, 2, 0], [0, 1, 1, 3]]

# Initial setup 
FEZ = FuzzyExtractor(LATTICE, 4)
FM = FuzzyMatching(FEZ)

S_A = [314, 168, 159, 333, 819, 606]
S_B = [68, 69, 358, 359, 444, 555, 818, 700, 717, 999, 819]

keys, helpers = FM.makeHelpers(S_A)
rec = FM.attemptMatching(helpers, S_B)
final = FM.returnMatches(rec, S_A, keys)
```
This is available under [furbyMatchingExample.py](https://github.com/spaceybread/crispy-fiesta/blob/main/pyImpl/furbyMatchingExample.py)

## Notes

Currently, this only works with numeric values. 

## TODO 

- Make it work with the Leech Lattice: 
    - https://www.math.rwth-aachen.de/~Gabriele.Nebe/LATTICES/Leech.html
    - http://neilsloane.com/doc/Me116.pdf
    - https://github.com/kzoacn/Leech

- Rework the message encoding: 
    - Do not split by decimal places, that's slow and useless
    - Chunk by 4-bytes, we don't know lattices larger than 24 dims with fast decoding with high packing density. 
