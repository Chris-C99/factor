Factor
====

Find the factors for any integer.

## Usage

Cargo.toml  

```toml
[dependencies]
factor = "0.4"
```

Add this to your crate root -

```toml#
extern crate factor;

use factor::factor::factor;
use factor::factor_include::factor_include;
```
   factor() - Will return factors of a given integer EXCLUDING 1 and the given number.
   
   factor_include() - Will return factors of a given integer INCLUDING 1 and the given number.
   
***Note : Factors are returned in a vector.

#Examples::

```
-println!("{:?}", factor(144));
    -Prints [2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 36, 48, 72]
```

```
-println!("{:?}", factor_include(144));
    -Prints [1, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 36, 48, 72, 144]
```

