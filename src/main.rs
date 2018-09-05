/*!

#
    factor() - Will return factors of a given integer EXCLUDING 1 and the given number.

    factor_include() - Will return factors of a given integer INCLUDING 1 and the given number.


        #Examples::

            -println!("{:?}", factor(144));
                Prints [2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 36, 48, 72]

            -println!("{:?}", factor_include(144));
                Prints [1, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 36, 48, 72, 144]


***Note : Factors are returned in a vector.


*/

fn main(){

    println!("{:?}", factor_include(144));

}
pub fn factor(input :i64) -> Vec<i64>{

    let count = 2;

    let mut vector = vec![];

    for count in count..input {
        if input % count == 0 {
            vector.push(count);
        }
    }

    return vector;

}

pub fn factor_include(input :i64) -> Vec<i64>{

    let count = 1;

    let mut vector = vec![];

    for count in count..input+1 {
        if input % count == 0 {
            vector.push(count);
        }
    }

    return vector;

}