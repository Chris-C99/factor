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