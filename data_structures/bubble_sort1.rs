pub fn add(left: usize, right: usize) -> usize {
    left + right 
}

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]){
    for _ in 0..v.len(){ // if the array is a million, this will check a million checks
        for i in 0..v.len()-1{
            if v[i] > v[i+1] {
                v.swap(i, i+1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort(){
        let mut v = vec![4,6,1,8,11,13];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,4,6,8,11,13]);
    }
}

//only run this in a lib folder
// to create a new lib(package): cargo new lib_package --lib
//cargo test 
