# Fuzzy Extractors

## Usage

### Init

To create a Fuzzy Extractor, define a ```lattice`` and ``lattice_dim``` as below: 
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

For more examples, look at [pyFuzzExample.py](https://github.com/spaceybread/crispy-fiesta/blob/main/pyFuzzExample.py)

## Notes

Currently, this only works with numeric values. 