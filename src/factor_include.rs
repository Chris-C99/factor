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